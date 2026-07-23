import * as assert from 'node:assert';
import * as fs from 'node:fs';
import * as os from 'node:os';
import * as path from 'node:path';

import { suite, test, teardown } from 'mocha';
import * as vscode from 'vscode';

import { CliClient } from '../core/cliClient';
import { createAddNoteCommand } from '../features/add-note/command';
import { AddNoteService } from '../features/add-note/service';
import { createShowNotesForCurrentFileCommand } from '../features/notes-panel/command';
import { FrilVaultNotesProvider } from '../features/notes-panel/provider';
import { NotesPanelService } from '../features/notes-panel/service';

interface TestWorkspace {
  root: string;
  cliPath: string;
  sourceFile: string;
  secondSourceFile: string;
  stateFile: string;
  addLogFile: string;
}

const createdWorkspaces: string[] = [];

suite('Extension Test Suite', () => {
  teardown(async () => {
    await vscode.workspace
      .getConfiguration('frilvault')
      .update('workspaceRoot', '', vscode.ConfigurationTarget.Global);
    await vscode.workspace
      .getConfiguration('frilvault')
      .update('cliPath', 'flvt', vscode.ConfigurationTarget.Global);
  });

  teardown(() => {
    while (createdWorkspaces.length > 0) {
      const workspace = createdWorkspaces.pop();
      if (workspace) {
        fs.rmSync(workspace, { recursive: true, force: true });
      }
    }
  });

  test('NotesPanelService parses JSON output from flvt list', async () => {
    const workspace = createTestWorkspace();
    writeNotesState(workspace, [
      createLineNoteView('src/sample.ts', 3, 5, 'service note'),
    ]);

    const cliClient = new CliClient(() => workspace.cliPath);
    const service = new NotesPanelService(cliClient);
    const notes = await service.listNotes(workspace.root, path.join('src', 'sample.ts'));

    assert.strictEqual(notes.length, 1);
    assert.strictEqual(notes[0]?.note.content, 'service note');
    assert.strictEqual(notes[0]?.note.anchor.type, 'Line');
  });

  test('FrilVault Notes provider reads the active editor file', async () => {
    const workspace = createTestWorkspace();
    writeNotesState(workspace, [
      createLineNoteView('src/sample.ts', 7, 2, 'first file note'),
      createLineNoteView('src/other.ts', 2, 1, 'second file note'),
    ]);

    await configureExtension(workspace);
    await openFile(workspace.sourceFile);

    const cliClient = new CliClient(() => workspace.cliPath);
    const provider = new FrilVaultNotesProvider(new NotesPanelService(cliClient), () => workspace.root);
    const firstChildren = await provider.getChildren();

    assert.strictEqual(firstChildren.length, 1);
    assert.strictEqual(firstChildren[0]?.label, path.join('src', 'sample.ts'));
    assert.strictEqual(firstChildren[0]?.description, '1 note');
    const firstNotes = await provider.getChildren(firstChildren[0]);
    assert.strictEqual(firstNotes[0]?.label, 'first file note');
    assert.strictEqual(firstNotes[0]?.description, 'L7');

    await openFile(workspace.secondSourceFile);

    const secondChildren = await provider.getChildren();

    assert.strictEqual(secondChildren.length, 1);
    assert.strictEqual(secondChildren[0]?.label, path.join('src', 'other.ts'));
    const secondNotes = await provider.getChildren(secondChildren[0]);
    assert.strictEqual(secondNotes[0]?.label, 'second file note');
    assert.strictEqual(secondNotes[0]?.description, 'L2');
  });

  test('Add Note command executes flvt add with relative file path and refreshes', async () => {
    const workspace = createTestWorkspace();
    await configureExtension(workspace);

    const editor = await openFile(workspace.sourceFile);
    editor.selection = new vscode.Selection(new vscode.Position(1, 4), new vscode.Position(1, 4));

    let treeRefreshCount = 0;
    let decorationRefreshCount = 0;
    let successMessage = '';
    let errorMessage = '';
    const cliClient = new CliClient(() => workspace.cliPath);

    const command = createAddNoteCommand({
      getWorkspaceRoot: () => workspace.root,
      service: new AddNoteService(cliClient),
      refreshNotesPanel: () => {
          treeRefreshCount += 1;
      },
      refreshDecorations: async () => {
          decorationRefreshCount += 1;
      },
      promptNoteContent: async () => 'added from command test',
      showInformationMessage: async (message) => {
        successMessage = message;
        return undefined;
      },
      showErrorMessage: async (message) => {
        errorMessage = message;
        return undefined;
      },
    });

    await command();

    const addLog = JSON.parse(fs.readFileSync(workspace.addLogFile, 'utf8')) as {
      file: string;
      line: number;
      column: number;
      content: string;
    };

    assert.deepStrictEqual(addLog, {
      file: path.join('src', 'sample.ts'),
      line: 2,
      column: 5,
      content: 'added from command test',
    });
    assert.strictEqual(treeRefreshCount, 1);
    assert.strictEqual(decorationRefreshCount, 1);
    assert.match(successMessage, /FrilVault note added at 2:5\./);
    assert.strictEqual(errorMessage, '');
  });

  test('Show Notes For Current File command focuses the panel and reports empty files', async () => {
    const workspace = createTestWorkspace();
    await configureExtension(workspace);
    await openFile(workspace.sourceFile);

    let treeRefreshCount = 0;
    let focusedCommand = '';
    let infoMessage = '';
    let errorMessage = '';
    const cliClient = new CliClient(() => workspace.cliPath);

    const command = createShowNotesForCurrentFileCommand({
      getWorkspaceRoot: () => workspace.root,
      service: new NotesPanelService(cliClient),
      refreshNotesPanel: () => {
        treeRefreshCount += 1;
      },
      executeCommand: async (commandId) => {
        focusedCommand = commandId;
      },
      showInformationMessage: async (message) => {
        infoMessage = message;
        return undefined;
      },
      showErrorMessage: async (message) => {
        errorMessage = message;
        return undefined;
      },
    });

    await command();

    assert.strictEqual(treeRefreshCount, 1);
    assert.strictEqual(focusedCommand, 'frilvault.notes.focus');
    assert.strictEqual(infoMessage, `No notes for ${path.join('src', 'sample.ts')}.`);
    assert.strictEqual(errorMessage, '');
  });

  test('Show Notes For Current File command focuses the panel when notes exist', async () => {
    const workspace = createTestWorkspace();
    writeNotesState(workspace, [
      createLineNoteView('src/sample.ts', 1, 1, 'existing note'),
    ]);
    await configureExtension(workspace);
    await openFile(workspace.sourceFile);

    let treeRefreshCount = 0;
    let focusedCommand = '';
    let infoMessage = '';
    const cliClient = new CliClient(() => workspace.cliPath);

    const command = createShowNotesForCurrentFileCommand({
      getWorkspaceRoot: () => workspace.root,
      service: new NotesPanelService(cliClient),
      refreshNotesPanel: () => {
        treeRefreshCount += 1;
      },
      executeCommand: async (commandId) => {
        focusedCommand = commandId;
      },
      showInformationMessage: async (message) => {
        infoMessage = message;
        return undefined;
      },
    });

    await command();

    assert.strictEqual(treeRefreshCount, 1);
    assert.strictEqual(focusedCommand, 'frilvault.notes.focus');
    assert.strictEqual(infoMessage, '');
  });
});

function createTestWorkspace(): TestWorkspace {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), 'frilvault-vscode-test-'));
  createdWorkspaces.push(root);

  const srcDir = path.join(root, 'src');
  fs.mkdirSync(srcDir, { recursive: true });

  const sourceFile = path.join(srcDir, 'sample.ts');
  const secondSourceFile = path.join(srcDir, 'other.ts');
  fs.writeFileSync(sourceFile, 'const sample = 1;\nconst next = 2;\n');
  fs.writeFileSync(secondSourceFile, 'export const other = true;\n');

  const cliPath = path.join(root, 'fake-flvt');
  const stateFile = path.join(root, '.frilvault-cli-state.json');
  const addLogFile = path.join(root, '.frilvault-add-log.json');

  fs.writeFileSync(stateFile, JSON.stringify({ notes: [] }, null, 2));
  fs.writeFileSync(
    cliPath,
    `#!/usr/bin/env node
const fs = require('fs');
const path = require('path');

const args = process.argv.slice(2);
const command = args[0];
const cwd = process.cwd();
const stateFile = path.join(cwd, '.frilvault-cli-state.json');
const addLogFile = path.join(cwd, '.frilvault-add-log.json');
const state = fs.existsSync(stateFile)
  ? JSON.parse(fs.readFileSync(stateFile, 'utf8'))
  : { notes: [] };

function valueOf(flag) {
  const index = args.indexOf(flag);
  return index >= 0 ? args[index + 1] : undefined;
}

if (command === 'list') {
  const file = valueOf('--file');
  const notes = state.notes.filter((note) => note.source_file === file);
  process.stdout.write(JSON.stringify(notes));
  process.exit(0);
}

if (command === 'add') {
  const file = valueOf('--file');
  const line = Number(valueOf('--line'));
  const column = Number(valueOf('--column'));
  const content = valueOf('--content');

  fs.writeFileSync(addLogFile, JSON.stringify({ file, line, column, content }, null, 2));

  state.notes.push({
    source_file: file,
    note: {
      id: 'test-note-id',
      anchor: { type: 'Line', line, column },
      content,
      created_at: '2026-06-09T00:00:00Z',
      updated_at: '2026-06-09T00:00:00Z'
    }
  });

  fs.writeFileSync(stateFile, JSON.stringify(state, null, 2));
  process.exit(0);
}

process.stderr.write('Unsupported fake flvt command');
process.exit(1);
`,
    { mode: 0o755 },
  );

  return {
    root,
    cliPath,
    sourceFile,
    secondSourceFile,
    stateFile,
    addLogFile,
  };
}

function createLineNoteView(sourceFile: string, line: number, column: number, content: string) {
  return {
    source_file: sourceFile,
    note: {
      id: `${sourceFile}-${line}-${column}`,
      anchor: {
        type: 'Line' as const,
        line,
        column,
      },
      content,
      created_at: '2026-06-09T00:00:00Z',
      updated_at: '2026-06-09T00:00:00Z',
    },
  };
}

function writeNotesState(
  workspace: TestWorkspace,
  notes: Array<ReturnType<typeof createLineNoteView>>,
): void {
  fs.writeFileSync(workspace.stateFile, JSON.stringify({ notes }, null, 2));
}

async function configureExtension(workspace: TestWorkspace): Promise<void> {
  await vscode.workspace
    .getConfiguration('frilvault')
    .update('workspaceRoot', workspace.root, vscode.ConfigurationTarget.Global);
  await vscode.workspace
    .getConfiguration('frilvault')
    .update('cliPath', workspace.cliPath, vscode.ConfigurationTarget.Global);
}

async function openFile(filePath: string): Promise<vscode.TextEditor> {
  const document = await vscode.workspace.openTextDocument(vscode.Uri.file(filePath));
  return vscode.window.showTextDocument(document);
}

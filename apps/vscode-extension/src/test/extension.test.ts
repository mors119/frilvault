import * as assert from 'node:assert';
import * as fs from 'node:fs';
import * as os from 'node:os';
import * as path from 'node:path';

import { suite, test, teardown } from 'mocha';
import * as vscode from 'vscode';

import { CliClient } from '../core/cliClient';
import { CurrentFileNotesStore } from '../features/current-file/store';
import { createAddNoteCommand } from '../features/add-note/command';
import { AddNoteService } from '../features/add-note/service';
import {
  GITIGNORE_PROMPT_DISABLED_KEY,
  maybePromptForGitignore,
} from '../features/gitignore/prompt';
import {
  FRILVAULT_ENABLED_KEY,
  isFrilVaultEnabled,
  setFrilVaultEnabled,
} from '../features/enablement/state';
import { isTrackedSourceRename } from '../features/workspace/rename';
import {
  isTrackedSourcePath,
  isTrackedVaultPath,
} from '../features/workspace/watcher';
import { createShowNotesForCurrentFileCommand } from '../features/notes-panel/command';
import { FrilVaultNotesProvider } from '../features/notes-panel/provider';
import { NotesPanelService } from '../features/notes-panel/service';
import { NotesPanelItem } from '../features/notes-panel/view';
import type { NoteView } from '../types';
import { revealNote } from '../utils/file';

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

  test('FrilVault Notes provider returns empty tree when extension is disabled', async () => {
    const workspace = createTestWorkspace();
    writeNotesState(workspace, [
      createLineNoteView('src/sample.ts', 7, 2, 'first file note'),
    ]);

    await configureExtension(workspace);
    await openFile(workspace.sourceFile);

    const cliClient = new CliClient(() => workspace.cliPath);
    const store = new CurrentFileNotesStore(cliClient, () => false, () => workspace.root);
    const provider = new FrilVaultNotesProvider(store, () => workspace.root, () => false);
    const children = await provider.getChildren();

    assert.strictEqual(children.length, 0);
  });

  test('Enablement state defaults to disabled and persists per workspace', async () => {
    const workspace = createTestWorkspace();
    const workspaceState = createMockWorkspaceState();

    assert.strictEqual(isFrilVaultEnabled(workspaceState, workspace.root), false);

    await setFrilVaultEnabled(workspaceState, workspace.root, true);

    assert.strictEqual(
      workspaceState.get<Record<string, boolean>>(FRILVAULT_ENABLED_KEY)?.[workspace.root],
      true,
    );
    assert.strictEqual(isFrilVaultEnabled(workspaceState, workspace.root), true);

    await setFrilVaultEnabled(workspaceState, workspace.root, false);

    assert.strictEqual(isFrilVaultEnabled(workspaceState, workspace.root), false);
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
    const store = new CurrentFileNotesStore(cliClient, () => true, () => workspace.root);
    await store.syncActiveEditor(vscode.window.activeTextEditor);
    const provider = new FrilVaultNotesProvider(store, () => workspace.root);
    const firstChildren = await provider.getChildren();

    assert.strictEqual(firstChildren.length, 1);
    assert.strictEqual(firstChildren[0]?.label, 'Line Notes');
    assert.strictEqual(firstChildren[0]?.description, '1');
    const firstNotes = await provider.getChildren(firstChildren[0]);
    assert.strictEqual(firstNotes[0]?.label, 'first file note');
    assert.strictEqual(firstNotes[0]?.description, 'L7');

    await openFile(workspace.secondSourceFile);
    await store.syncActiveEditor(vscode.window.activeTextEditor);

    const secondChildren = await provider.getChildren();

    assert.strictEqual(secondChildren.length, 1);
    assert.strictEqual(secondChildren[0]?.label, 'Line Notes');
    const secondNotes = await provider.getChildren(secondChildren[0]);
    assert.strictEqual(secondNotes[0]?.label, 'second file note');
    assert.strictEqual(secondNotes[0]?.description, 'L2');
  });

  test('FrilVault Notes provider groups line and symbol notes separately', async () => {
    const workspace = createTestWorkspace();
    writeNotesState(workspace, [
      createSymbolNoteView('src/sample.ts', 'myFn', 12, 'symbol note'),
      createLineNoteView('src/sample.ts', 3, 1, 'line note'),
    ]);

    await configureExtension(workspace);
    await openFile(workspace.sourceFile);

    const cliClient = new CliClient(() => workspace.cliPath);
    const store = new CurrentFileNotesStore(cliClient, () => true, () => workspace.root);
    await store.syncActiveEditor(vscode.window.activeTextEditor);
    const provider = new FrilVaultNotesProvider(store, () => workspace.root);
    const groups = await provider.getChildren();

    assert.strictEqual(groups.length, 2);
    assert.strictEqual(groups[0]?.label, 'Line Notes');
    assert.strictEqual(groups[1]?.label, 'Symbol Notes');

    const lineNotes = await provider.getChildren(groups[0]);
    assert.strictEqual(lineNotes.length, 1);
    assert.strictEqual(lineNotes[0]?.label, 'line note');

    const symbolNotes = await provider.getChildren(groups[1]);
    assert.strictEqual(symbolNotes.length, 1);
    assert.strictEqual(symbolNotes[0]?.label, 'symbol note');
    assert.strictEqual(symbolNotes[0]?.description, 'L12 myFn');
  });

  test('Symbol note reveal prefers resolved coordinates', async () => {
    const workspace = createTestWorkspace();
    const noteView = createSymbolNoteView('src/sample.ts', 'myFn', 1, 'symbol note', {
      line: 8,
      column: 4,
    });
    const item = new NotesPanelItem(noteView, workspace.root);

    assert.strictEqual(item.description, 'L8 myFn');

    await configureExtension(workspace);
    await openFile(workspace.sourceFile);

    await revealNote(noteView, workspace.root);

    const editor = vscode.window.activeTextEditor;
    assert.ok(editor);
    assert.strictEqual(editor.selection.active.line, 7);
    assert.strictEqual(editor.selection.active.character, 3);
  });

  test('Add Note command executes flvt add with relative file path and refreshes', async () => {
    const workspace = createTestWorkspace();
    await configureExtension(workspace);

    const editor = await openFile(workspace.sourceFile);
    editor.selection = new vscode.Selection(new vscode.Position(1, 4), new vscode.Position(1, 4));

    let invalidateCount = 0;
    let successMessage = '';
    let errorMessage = '';
    const cliClient = new CliClient(() => workspace.cliPath);

    const command = createAddNoteCommand({
      getWorkspaceRoot: () => workspace.root,
      service: new AddNoteService(cliClient),
      invalidateViews: async () => {
        invalidateCount += 1;
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
    assert.strictEqual(invalidateCount, 1);
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

  test('Gitignore prompt appends entry when user accepts', async () => {
    const workspace = createTestWorkspace();
    let addCalled = false;
    let infoMessage = '';

    const workspaceState = createMockWorkspaceState();

    await maybePromptForGitignore({
      getWorkspaceRoot: () => workspace.root,
      cliClient: {
        checkGitignore: async () => ({ ignored: false }),
        addGitignoreEntry: async () => {
          addCalled = true;
        },
      } as unknown as CliClient,
      workspaceState,
      showWarningMessage: async (_message, _options, ...items) => items[0],
      showInformationMessage: async (message) => {
        infoMessage = message;
        return undefined;
      },
    });

    assert.strictEqual(addCalled, true);
    assert.strictEqual(infoMessage, 'Added `.vault/` to `.gitignore`.');
  });

  test('Gitignore prompt respects never ask again preference', async () => {
    const workspace = createTestWorkspace();
    let checkCount = 0;

    const workspaceState = createMockWorkspaceState();
    await workspaceState.update(GITIGNORE_PROMPT_DISABLED_KEY, {
      [workspace.root]: true,
    });

    await maybePromptForGitignore({
      getWorkspaceRoot: () => workspace.root,
      cliClient: {
        checkGitignore: async () => {
          checkCount += 1;
          return { ignored: false };
        },
        addGitignoreEntry: async () => {
          throw new Error('should not append when prompt is disabled');
        },
      } as unknown as CliClient,
      workspaceState,
      showWarningMessage: async () => 'Add to .gitignore',
    });

    assert.strictEqual(checkCount, 0);
  });

  test('Source rename handler ignores vault paths and outside workspace renames', () => {
    const workspace = createTestWorkspace();

    assert.strictEqual(
      isTrackedSourceRename(
        workspace.root,
        vscode.Uri.file(path.join(workspace.root, 'src/sample.ts')),
        vscode.Uri.file(path.join(workspace.root, 'src/sample_renamed.ts')),
      ),
      true,
    );

    assert.strictEqual(
      isTrackedSourceRename(
        workspace.root,
        vscode.Uri.file(path.join(workspace.root, '.vault/notes/src/sample.ts.json')),
        vscode.Uri.file(path.join(workspace.root, '.vault/notes/src/sample_renamed.ts.json')),
      ),
      false,
    );

    assert.strictEqual(
      isTrackedSourceRename(
        workspace.root,
        vscode.Uri.file('/tmp/outside.ts'),
        vscode.Uri.file('/tmp/outside_renamed.ts'),
      ),
      false,
    );
  });

  test('Workspace watcher helpers distinguish vault notes and source paths', () => {
    const workspace = createTestWorkspace();

    assert.strictEqual(
      isTrackedVaultPath(
        workspace.root,
        vscode.Uri.file(path.join(workspace.root, '.vault/notes/src/sample.ts.json')),
      ),
      true,
    );

    assert.strictEqual(
      isTrackedSourcePath(
        workspace.root,
        vscode.Uri.file(path.join(workspace.root, 'src/sample.ts')),
      ),
      true,
    );

    assert.strictEqual(
      isTrackedSourcePath(
        workspace.root,
        vscode.Uri.file(path.join(workspace.root, '.vault/notes/src/sample.ts.json')),
      ),
      false,
    );
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
  const tags = [];
  for (let index = 0; index < args.length; index += 1) {
    if (args[index] === '--tag') {
      tags.push(args[index + 1]);
    }
  }

  fs.writeFileSync(addLogFile, JSON.stringify({ file, line, column, content }, null, 2));

  const noteView = {
    source_file: file,
    note: {
      id: 'test-note-id',
      anchor: { type: 'Line', line, column },
      content,
      tags,
      created_at: '2026-06-09T00:00:00Z',
      updated_at: '2026-06-09T00:00:00Z'
    }
  };

  state.notes.push(noteView);
  fs.writeFileSync(stateFile, JSON.stringify(state, null, 2));

  if (valueOf('--format') === 'json') {
    process.stdout.write(JSON.stringify(noteView));
  }

  process.exit(0);
}

if (command === 'update') {
  const file = valueOf('--file');
  const id = valueOf('--id');
  const content = valueOf('--content');
  const tags = [];
  for (let index = 0; index < args.length; index += 1) {
    if (args[index] === '--tag') {
      tags.push(args[index + 1]);
    }
  }

  const noteView = state.notes.find((note) => note.source_file === file && note.note.id === id);
  if (!noteView) {
    process.stderr.write('note not found');
    process.exit(1);
  }

  if (valueOf('--expected-updated-at') && valueOf('--expected-updated-at') !== noteView.note.updated_at) {
    process.stderr.write('concurrent modification for note: ' + id);
    process.exit(1);
  }

  noteView.note.content = content;
  noteView.note.tags = tags;
  noteView.note.updated_at = '2026-06-10T00:00:00Z';
  fs.writeFileSync(stateFile, JSON.stringify(state, null, 2));

  if (valueOf('--format') === 'json') {
    process.stdout.write(JSON.stringify(noteView));
  }

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

function createLineNoteView(
  sourceFile: string,
  line: number,
  column: number,
  content: string,
): NoteView {
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

function createSymbolNoteView(
  sourceFile: string,
  name: string,
  lineHint: number,
  content: string,
  resolved?: { line: number; column: number },
): NoteView {
  return {
    source_file: sourceFile,
    note: {
      id: `${sourceFile}-${name}`,
      anchor: {
        type: 'Symbol' as const,
        name,
        kind: 'Function',
        line_hint: lineHint,
      },
      content,
      created_at: '2026-06-09T00:00:00Z',
      updated_at: '2026-06-09T00:00:00Z',
    },
    resolved,
  };
}

function writeNotesState(workspace: TestWorkspace, notes: NoteView[]): void {
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

function createMockWorkspaceState(): vscode.Memento {
  const storage = new Map<string, unknown>();

  return {
    keys: () => [...storage.keys()],
    get: <T>(key: string) => storage.get(key) as T | undefined,
    update: async (key: string, value: unknown) => {
      storage.set(key, value);
    },
  };
}

import * as path from 'path';
import * as vscode from 'vscode';

import { createAddNoteCommand } from './features/add-note/command';
import { DecorationsProvider } from './features/decorations/provider';
import {
  FrilVaultNoteTreeItem,
  FrilVaultNotesProvider,
} from './features/notes-panel/provider';
import { FrilVaultNativeClient } from './native';
import type { NoteView, RepairSuggestion, WorkspaceHealth, WorkspaceStats } from './types';

function getWorkspaceRoot(): string {
  const configured = vscode.workspace
    .getConfiguration('frilvault')
    .get<string>('workspaceRoot', '')
    .trim();

  if (configured.length > 0) {
    return configured;
  }

  const folder = vscode.workspace.workspaceFolders?.[0];
  if (!folder) {
    throw new Error('FrilVault requires an open workspace folder.');
  }

  return folder.uri.fsPath;
}

function getActiveEditorOrThrow(): vscode.TextEditor {
  const editor = vscode.window.activeTextEditor;
  if (!editor) {
    throw new Error('No active editor.');
  }

  return editor;
}

function getActiveFilePathOrThrow(): string {
  const editor = getActiveEditorOrThrow();

  if (editor.document.uri.scheme !== 'file') {
    throw new Error('FrilVault only supports files on disk.');
  }

  return editor.document.uri.fsPath;
}

function formatStats(stats: WorkspaceStats): string {
  return [
    `files: ${stats.file_count}`,
    `notes: ${stats.total_notes}`,
    `existing files: ${stats.existing_files}`,
    `missing files: ${stats.missing_files}`,
    `line notes: ${stats.line_notes}`,
    `symbol notes: ${stats.symbol_notes}`,
  ].join('\n');
}

function formatHealth(health: WorkspaceHealth): string {
  if (health.missing_source_files.length === 0) {
    return 'No missing source files.';
  }

  return ['Missing source files:', ...health.missing_source_files].join('\n');
}

function formatRepairSuggestions(suggestions: RepairSuggestion[]): string {
  if (suggestions.length === 0) {
    return 'No repair suggestions.';
  }

  return suggestions
    .map((suggestion) => {
      const candidates =
        suggestion.candidates.length > 0
          ? suggestion.candidates.join(', ')
          : 'no candidates';

      return `${suggestion.missing_file} -> ${candidates}`;
    })
    .join('\n');
}

async function revealNote(note: NoteView, workspaceRoot: string): Promise<void> {
  const sourcePath = path.join(workspaceRoot, note.source_file);
  const document = await vscode.workspace.openTextDocument(sourcePath);
  const editor = await vscode.window.showTextDocument(document);

  const line =
    note.note.anchor.type === 'Line'
      ? Math.max(note.note.anchor.line - 1, 0)
      : Math.max((note.note.anchor.line_hint ?? 1) - 1, 0);
  const column = note.note.anchor.type === 'Line' ? Math.max(note.note.anchor.column - 1, 0) : 0;
  const position = new vscode.Position(line, column);
  editor.selection = new vscode.Selection(position, position);
  editor.revealRange(new vscode.Range(position, position));
}

async function promptNoteContent(initialValue = ''): Promise<string | undefined> {
  const content = await vscode.window.showInputBox({
    prompt: 'Enter a FrilVault note',
    value: initialValue,
    ignoreFocusOut: true,
    validateInput(value) {
      return value.trim().length === 0 ? 'Note content is required.' : undefined;
    },
  });

  return content?.trim();
}

async function editNote(
  client: FrilVaultNativeClient,
  provider: FrilVaultNotesProvider,
  decorationsProvider: DecorationsProvider,
  item?: FrilVaultNoteTreeItem,
): Promise<void> {
  const target = item ?? (await pickCurrentFileNote(client));
  if (!target) {
    return;
  }

  const workspaceRoot = getWorkspaceRoot();
  const sourceFile = path.join(workspaceRoot, target.noteView.source_file);
  const content = await promptNoteContent(target.noteView.note.content);

  if (!content) {
    return;
  }

  client.updateNote(workspaceRoot, sourceFile, target.noteView.note.id, content);
  provider.refresh();
  await decorationsProvider.refresh();
}

async function deleteNote(
  client: FrilVaultNativeClient,
  provider: FrilVaultNotesProvider,
  decorationsProvider: DecorationsProvider,
  item?: FrilVaultNoteTreeItem,
): Promise<void> {
  const target = item ?? (await pickCurrentFileNote(client));
  if (!target) {
    return;
  }

  const confirmed = await vscode.window.showWarningMessage(
    'Delete this FrilVault note?',
    { modal: true },
    'Delete',
  );

  if (confirmed !== 'Delete') {
    return;
  }

  const workspaceRoot = getWorkspaceRoot();
  const sourceFile = path.join(workspaceRoot, target.noteView.source_file);
  client.deleteNote(workspaceRoot, sourceFile, target.noteView.note.id);
  provider.refresh();
  await decorationsProvider.refresh();
}

async function pickCurrentFileNote(
  client: FrilVaultNativeClient,
): Promise<FrilVaultNoteTreeItem | undefined> {
  const workspaceRoot = getWorkspaceRoot();
  const sourceFile = getActiveFilePathOrThrow();
  const notes = client.listNotes(workspaceRoot, sourceFile);

  if (notes.length === 0) {
    vscode.window.showInformationMessage('No FrilVault notes found for the current file.');
    return undefined;
  }

  const picked = await vscode.window.showQuickPick(
    notes.map((noteView) => ({
      label: noteView.note.content,
      description:
        noteView.note.anchor.type === 'Line'
          ? `L${noteView.note.anchor.line}:C${noteView.note.anchor.column}`
          : `${noteView.note.anchor.kind}: ${noteView.note.anchor.name}`,
      noteView,
    })),
    { placeHolder: 'Select a FrilVault note' },
  );

  return picked ? new FrilVaultNoteTreeItem(picked.noteView, sourceFile) : undefined;
}

async function searchNotes(client: FrilVaultNativeClient): Promise<void> {
  const workspaceRoot = getWorkspaceRoot();
  const keyword = await vscode.window.showInputBox({
    prompt: 'Search FrilVault notes',
    ignoreFocusOut: true,
  });

  if (!keyword || keyword.trim().length === 0) {
    return;
  }

  const results = client.searchNotes(workspaceRoot, keyword.trim());
  if (results.length === 0) {
    vscode.window.showInformationMessage(`No notes found for "${keyword}".`);
    return;
  }

  const picked = await vscode.window.showQuickPick(
    results.map((note) => ({
      label: note.note.content,
      description: note.source_file,
      detail:
        note.note.anchor.type === 'Line'
          ? `Line ${note.note.anchor.line}, Column ${note.note.anchor.column}`
          : `${note.note.anchor.kind}: ${note.note.anchor.name}`,
      note,
    })),
    { placeHolder: `Found ${results.length} note(s)` },
  );

  if (picked) {
    await revealNote(picked.note, workspaceRoot);
  }
}

function showTextDocument(title: string, body: string): void {
  const channel = vscode.window.createOutputChannel(title);
  channel.clear();
  channel.appendLine(body);
  channel.show(true);
}

export function activate(context: vscode.ExtensionContext): void {
  const client = new FrilVaultNativeClient(context);
  const provider = new FrilVaultNotesProvider(getWorkspaceRoot);
  const decorationsProvider = new DecorationsProvider(context.extensionPath, getWorkspaceRoot);
  const addNoteCommand = createAddNoteCommand({
    getWorkspaceRoot,
    noteTreeDataProvider: provider,
    decorationsProvider,
  });

  context.subscriptions.push(
    decorationsProvider,
    vscode.window.registerTreeDataProvider('frilvault.notes', provider),
    vscode.commands.registerCommand('frilvault.notesPanel.openNote', async (noteView: NoteView) => {
      await revealNote(noteView, getWorkspaceRoot());
    }),
    vscode.commands.registerCommand('frilvault.refresh', () => provider.refresh()),
    vscode.commands.registerCommand('frilvault.addNote', addNoteCommand),
    vscode.commands.registerCommand('frilvault.editNote', (item?: FrilVaultNoteTreeItem) =>
      editNote(client, provider, decorationsProvider, item),
    ),
    vscode.commands.registerCommand('frilvault.deleteNote', (item?: FrilVaultNoteTreeItem) =>
      deleteNote(client, provider, decorationsProvider, item),
    ),
    vscode.commands.registerCommand('frilvault.searchNotes', () => searchNotes(client)),
    vscode.commands.registerCommand('frilvault.showStats', () => {
      const stats = client.workspaceStats(getWorkspaceRoot());
      showTextDocument('FrilVault Stats', formatStats(stats));
    }),
    vscode.commands.registerCommand('frilvault.showHealth', () => {
      const health = client.workspaceHealth(getWorkspaceRoot());
      const suggestions = client.repairSuggestions(getWorkspaceRoot());
      showTextDocument(
        'FrilVault Health',
        `${formatHealth(health)}\n\n${formatRepairSuggestions(suggestions)}`,
      );
    }),
    vscode.commands.registerCommand('frilvault.applyRepairs', async () => {
      const repaired = client.applyRepairs(getWorkspaceRoot());
      provider.refresh();
      await decorationsProvider.refresh();
      vscode.window.showInformationMessage(`FrilVault repaired ${repaired} file(s).`);
    }),
    vscode.window.onDidChangeActiveTextEditor(async (editor) => {
      provider.refresh();
      await decorationsProvider.refresh(editor);
    }),
    vscode.workspace.onDidSaveTextDocument(async () => {
      provider.refresh();
      await decorationsProvider.refresh();
    }),
  );

  void decorationsProvider.refresh();
}

export function deactivate(): void {}

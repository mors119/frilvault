import * as vscode from 'vscode';

import { CliClient } from './core/cliClient';
import { createAddNoteCommand } from './features/add-note/command';
import { AddNoteService } from './features/add-note/service';
import { FrilVaultDecorator } from './features/decorations/decorator';
import { FrilVaultHoverProvider } from './features/hover/hoverProvider';
import { FrilVaultNotesProvider } from './features/notes-panel/provider';
import { NotesPanelService } from './features/notes-panel/service';
import { createSearchCommand } from './features/search/command';
import { createApplyRepairsCommand, createShowHealthCommand } from './features/workspace/health';
import { createShowStatsCommand } from './features/workspace/stats';
import type { NoteView } from './types';
import { getWorkspaceRoot, revealNote } from './utils/file';

export function activate(context: vscode.ExtensionContext): void {
  const cliClient = new CliClient();
  const addNoteService = new AddNoteService(cliClient);
  const notesPanelService = new NotesPanelService(cliClient);
  const notesProvider = new FrilVaultNotesProvider(notesPanelService, getWorkspaceRoot);
  const decorator = new FrilVaultDecorator(context.extensionPath, cliClient, getWorkspaceRoot);
  const hoverProvider = new FrilVaultHoverProvider(cliClient, getWorkspaceRoot);

  context.subscriptions.push(
    decorator,
    vscode.window.registerTreeDataProvider('frilvault.notes', notesProvider),
    vscode.languages.registerHoverProvider({ scheme: 'file' }, hoverProvider),
    vscode.commands.registerCommand('frilvault.notesPanel.openNote', async (noteView: NoteView) => {
      await revealNote(noteView, getWorkspaceRoot());
    }),
    vscode.commands.registerCommand(
      'frilvault.addNote',
      createAddNoteCommand({
        getWorkspaceRoot,
        service: addNoteService,
        refreshNotesPanel: () => notesProvider.refresh(),
        refreshDecorations: async (editor) => decorator.refresh(editor),
      }),
    ),
    vscode.commands.registerCommand(
      'frilvault.searchNotes',
      createSearchCommand(cliClient, getWorkspaceRoot),
    ),
    vscode.commands.registerCommand(
      'frilvault.showStats',
      createShowStatsCommand(cliClient, getWorkspaceRoot),
    ),
    vscode.commands.registerCommand(
      'frilvault.showHealth',
      createShowHealthCommand(cliClient, getWorkspaceRoot),
    ),
    vscode.commands.registerCommand(
      'frilvault.applyRepairs',
      createApplyRepairsCommand(
        cliClient,
        getWorkspaceRoot,
        () => notesProvider.refresh(),
        async () => decorator.refresh(),
      ),
    ),
    vscode.commands.registerCommand('frilvault.refresh', async () => {
      notesProvider.refresh();
      await decorator.refresh();
    }),
    vscode.window.onDidChangeActiveTextEditor(async (editor) => {
      notesProvider.refresh();
      await decorator.refresh(editor);
    }),
    vscode.workspace.onDidSaveTextDocument(async () => {
      notesProvider.refresh();
      await decorator.refresh();
    }),
  );

  void decorator.refresh();
}

export function deactivate(): void {}

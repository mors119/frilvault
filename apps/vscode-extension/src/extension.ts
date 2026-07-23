import * as vscode from 'vscode';

import { CliClient } from './core/cliClient';
import { createAddNoteCommand } from './features/add-note/command';
import { AddNoteService } from './features/add-note/service';
import { createDisableCommand, createEnableCommand } from './features/enablement/command';
import { isFrilVaultEnabled, syncEnabledContext } from './features/enablement/state';
import { maybePromptForGitignore } from './features/gitignore/prompt';
import { FrilVaultDecorator } from './features/decorations/decorator';
import { FrilVaultHoverProvider } from './features/hover/hoverProvider';
import { createShowNotesForCurrentFileCommand } from './features/notes-panel/command';
import { FrilVaultNotesProvider } from './features/notes-panel/provider';
import { NotesPanelService } from './features/notes-panel/service';
import { createSearchCommand } from './features/search/command';
import { createApplyRepairsCommand, createShowHealthCommand } from './features/workspace/health';
import { registerSourceRenameHandler } from './features/workspace/rename';
import { registerWorkspaceWatcher } from './features/workspace/watcher';
import { createShowStatsCommand } from './features/workspace/stats';
import type { NoteView } from './types';
import { getWorkspaceRoot, revealNote } from './utils/file';

export function activate(context: vscode.ExtensionContext): void {
  const cliClient = new CliClient();
  const addNoteService = new AddNoteService(cliClient);
  const notesPanelService = new NotesPanelService(cliClient);

  const isEnabled = () => isFrilVaultEnabled(context.workspaceState, getWorkspaceRoot());

  const notesProvider = new FrilVaultNotesProvider(
    notesPanelService,
    getWorkspaceRoot,
    isEnabled,
  );
  const decorator = new FrilVaultDecorator(
    context.extensionPath,
    cliClient,
    getWorkspaceRoot,
    isEnabled,
  );
  const hoverProvider = new FrilVaultHoverProvider(cliClient, getWorkspaceRoot, isEnabled);

  const refreshUi = async (editor?: vscode.TextEditor) => {
    notesProvider.refresh();
    await decorator.refresh(editor);
  };

  const clearUi = () => {
    decorator.clear();
    notesProvider.refresh();
  };

  const runWhenEnabled = <T extends unknown[]>(
    handler: (...args: T) => void | Promise<void>,
  ) => {
    return async (...args: T) => {
      if (!isEnabled()) {
        void vscode.window.showInformationMessage(
          'FrilVault is disabled for this workspace. Turn it on from the FrilVault Notes view.',
        );
        return;
      }

      await handler(...args);
    };
  };

  context.subscriptions.push(
    decorator,
    vscode.window.registerTreeDataProvider('frilvault.notes', notesProvider),
    vscode.languages.registerHoverProvider({ scheme: 'file' }, hoverProvider),
    vscode.commands.registerCommand('frilvault.notesPanel.openNote', async (noteView: NoteView) => {
      if (!isEnabled()) {
        return;
      }

      await revealNote(noteView, getWorkspaceRoot());
    }),
    vscode.commands.registerCommand(
      'frilvault.enable',
      createEnableCommand({
        getWorkspaceRoot,
        workspaceState: context.workspaceState,
        refreshUi,
        clearUi,
      }),
    ),
    vscode.commands.registerCommand(
      'frilvault.disable',
      createDisableCommand({
        getWorkspaceRoot,
        workspaceState: context.workspaceState,
        refreshUi,
        clearUi,
      }),
    ),
    vscode.commands.registerCommand(
      'frilvault.addNote',
      runWhenEnabled(
        createAddNoteCommand({
          getWorkspaceRoot,
          service: addNoteService,
          refreshNotesPanel: () => notesProvider.refresh(),
          refreshDecorations: async (editor) => decorator.refresh(editor),
          onNoteAdded: async () => {
            await maybePromptForGitignore({
              getWorkspaceRoot,
              cliClient,
              workspaceState: context.workspaceState,
            });
          },
        }),
      ),
    ),
    vscode.commands.registerCommand(
      'frilvault.searchNotes',
      runWhenEnabled(createSearchCommand(cliClient, getWorkspaceRoot)),
    ),
    vscode.commands.registerCommand(
      'frilvault.showNotesForCurrentFile',
      runWhenEnabled(
        createShowNotesForCurrentFileCommand({
          getWorkspaceRoot,
          service: notesPanelService,
          refreshNotesPanel: () => notesProvider.refresh(),
        }),
      ),
    ),
    vscode.commands.registerCommand(
      'frilvault.showStats',
      runWhenEnabled(createShowStatsCommand(cliClient, getWorkspaceRoot)),
    ),
    vscode.commands.registerCommand(
      'frilvault.showHealth',
      runWhenEnabled(createShowHealthCommand(cliClient, getWorkspaceRoot)),
    ),
    vscode.commands.registerCommand(
      'frilvault.applyRepairs',
      runWhenEnabled(
        createApplyRepairsCommand(
          cliClient,
          getWorkspaceRoot,
          () => notesProvider.refresh(),
          async () => decorator.refresh(),
        ),
      ),
    ),
    vscode.commands.registerCommand(
      'frilvault.refresh',
      runWhenEnabled(async () => {
        await refreshUi();
      }),
    ),
    vscode.window.onDidChangeActiveTextEditor(async (editor) => {
      await refreshUi(editor);
    }),
    vscode.workspace.onDidSaveTextDocument(async () => {
      await refreshUi();
    }),
  );

  registerSourceRenameHandler(
    context,
    cliClient,
    getWorkspaceRoot,
    isEnabled,
    () => notesProvider.refresh(),
    async () => decorator.refresh(),
  );

  registerWorkspaceWatcher(
    context,
    cliClient,
    getWorkspaceRoot,
    isEnabled,
    () => notesProvider.refresh(),
    async () => decorator.refresh(),
  );

  void syncEnabledContext(isEnabled()).then(() => {
    if (isEnabled()) {
      void refreshUi();
      return;
    }

    clearUi();
  });
}

export function deactivate(): void {}

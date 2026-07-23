import * as vscode from 'vscode';

import { CliClient } from './core/cliClient';
import { createAddNoteCommand } from './features/add-note/command';
import { AddNoteService } from './features/add-note/service';
import { CurrentFileNotesStore } from './features/current-file/store';
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
import { getWorkspaceRoot, revealNote, tryGetWorkspaceRoot } from './utils/file';

let activeDecorator: FrilVaultDecorator | undefined;
let activeStore: CurrentFileNotesStore | undefined;

export function activate(context: vscode.ExtensionContext): void {
  const cliClient = new CliClient();
  const addNoteService = new AddNoteService(cliClient);
  const notesPanelService = new NotesPanelService(cliClient);

  const isEnabled = () => {
    const workspaceRoot = tryGetWorkspaceRoot();

    if (!workspaceRoot) {
      return false;
    }

    return isFrilVaultEnabled(context.workspaceState, workspaceRoot);
  };

  const store = new CurrentFileNotesStore(cliClient, isEnabled);
  activeStore = store;

  const notesProvider = new FrilVaultNotesProvider(store, getWorkspaceRoot, isEnabled);
  const decorator = new FrilVaultDecorator(
    context.extensionPath,
    store,
    getWorkspaceRoot,
    isEnabled,
  );
  activeDecorator = decorator;
  const hoverProvider = new FrilVaultHoverProvider(store, getWorkspaceRoot, isEnabled);

  const invalidateViews = async (editor?: vscode.TextEditor) => {
    await store.syncActiveEditor(editor ?? vscode.window.activeTextEditor);
  };

  const refreshUi = async (editor?: vscode.TextEditor) => {
    await invalidateViews(editor);
  };

  const clearUi = () => {
    store.clear();
    decorator.clear();
    notesProvider.refresh();
  };

  const onStoreChanged = () => {
    notesProvider.refresh();
    void decorator.refresh();
  };

  store.onDidChange(onStoreChanged, undefined, context.subscriptions);

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
    store,
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
          invalidateViews,
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
      runWhenEnabled(createApplyRepairsCommand(cliClient, getWorkspaceRoot, invalidateViews)),
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

  registerSourceRenameHandler(context, cliClient, isEnabled, invalidateViews);
  registerWorkspaceWatcher(context, cliClient, isEnabled, invalidateViews);

  void syncEnabledContext(isEnabled()).then(async () => {
    try {
      if (isEnabled()) {
        await refreshUi();
        return;
      }

      clearUi();
    } catch {
      clearUi();
    }
  });
}

export function deactivate(): void {
  activeDecorator?.clear();
  activeStore?.clear();
  activeDecorator = undefined;
  activeStore = undefined;
}

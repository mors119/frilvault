import * as vscode from 'vscode';

import { CliClient } from './core/cliClient';
import { CurrentFileNotesStore } from './features/current-file/store';
import { createDisableCommand, createEnableCommand } from './features/enablement/command';
import { isFrilVaultEnabled, syncEnabledContext } from './features/enablement/state';
import { maybePromptForGitignore } from './features/gitignore/prompt';
import { FrilVaultDecorator } from './features/decorations/decorator';
import { GutterNoteActions } from './features/decorations/gutterActions';
import { registerGutterCommands } from './features/decorations/gutterCommands';
import { GutterNoteRegistry } from './features/decorations/registry';
import { FrilVaultHoverProvider } from './features/hover/hoverProvider';
import { registerInlineNoteCodeLensProvider } from './features/inline-editor/codelens';
import {
  createCreateNoteHereCommand,
  createEditNoteCommand,
} from './features/inline-editor/command';
import { createInlineNoteEditor } from './features/inline-editor/editor';
import { createShowNotesForCurrentFileCommand } from './features/notes-panel/command';
import { FrilVaultNotesProvider } from './features/notes-panel/provider';
import { NotesPanelService } from './features/notes-panel/service';
import { createSearchCommand } from './features/search/command';
import { createApplyRepairsCommand, createShowHealthCommand } from './features/workspace/health';
import { registerSourceRenameHandler } from './features/workspace/rename';
import { registerNoteUriHandler } from './features/uri/handler';
import { registerWorkspaceWatcher } from './features/workspace/watcher';
import { createShowStatsCommand } from './features/workspace/stats';
import type { NoteView } from './types';
import { getWorkspaceRoot, revealNote, tryGetWorkspaceRoot } from './utils/file';

let activeDecorator: FrilVaultDecorator | undefined;
let activeStore: CurrentFileNotesStore | undefined;
let activeRegistry: GutterNoteRegistry | undefined;
const codeLensRefreshEmitter = new vscode.EventEmitter<void>();

export function activate(context: vscode.ExtensionContext): void {
  const cliClient = new CliClient();
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

  const gutterRegistry = new GutterNoteRegistry();
  activeRegistry = gutterRegistry;

  const notesProvider = new FrilVaultNotesProvider(store, getWorkspaceRoot, isEnabled);
  const decorator = new FrilVaultDecorator(
    context.extensionPath,
    store,
    gutterRegistry,
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

  const inlineNoteEditor = createInlineNoteEditor({
    cliClient,
    getWorkspaceRoot,
    invalidateViews: async () => {
      await refreshUi();
      await maybePromptForGitignore({
        getWorkspaceRoot,
        cliClient,
        workspaceState: context.workspaceState,
      });
    },
  });
  inlineNoteEditor.register(context);

  const gutterActions = new GutterNoteActions({
    cliClient,
    registry: gutterRegistry,
    getWorkspaceRoot,
    invalidateViews,
    openInlineEditor: (noteView) => inlineNoteEditor.openEdit(noteView),
  });

  const clearUi = () => {
    store.clear();
    gutterRegistry.clear();
    decorator.clear();
    notesProvider.refresh();
  };

  registerGutterCommands(context, gutterActions);

  const onStoreChanged = () => {
    notesProvider.refresh();
    void decorator.refresh();
    codeLensRefreshEmitter.fire();
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
      runWhenEnabled(createCreateNoteHereCommand(inlineNoteEditor)),
    ),
    vscode.commands.registerCommand(
      'frilvault.createNoteHere',
      runWhenEnabled(createCreateNoteHereCommand(inlineNoteEditor)),
    ),
    vscode.commands.registerCommand(
      'frilvault.editNote',
      runWhenEnabled(createEditNoteCommand(inlineNoteEditor)),
    ),
    vscode.commands.registerCommand(
      'frilvault.notesPanel.editNote',
      runWhenEnabled((item: { noteView?: NoteView }) => {
        if (!item?.noteView) {
          return;
        }

        inlineNoteEditor.openEdit(item.noteView);
      }),
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
  registerNoteUriHandler(context, { cliClient, isEnabled });
  registerInlineNoteCodeLensProvider(
    context,
    store,
    getWorkspaceRoot,
    isEnabled,
    codeLensRefreshEmitter.event,
  );

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
  activeRegistry?.clear();
  activeDecorator = undefined;
  activeStore = undefined;
  activeRegistry = undefined;
}

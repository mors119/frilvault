import * as vscode from 'vscode';

import { notesViewFocusCommand } from '../../constants/ids';
import { getActiveEditorOrThrow, getRelativeFilePath } from '../../utils/file';
import { NotesPanelService } from './service';

export interface ShowNotesForCurrentFileDependencies {
  getWorkspaceRoot: () => string;
  service: NotesPanelService;
  refreshNotesPanel: () => void;
  executeCommand?: (command: string) => Thenable<unknown>;
  showInformationMessage?: (message: string) => Thenable<string | undefined>;
  showErrorMessage?: (message: string) => Thenable<string | undefined>;
}

export function createShowNotesForCurrentFileCommand(
  dependencies: ShowNotesForCurrentFileDependencies,
): () => Promise<void> {
  const executeCommand = dependencies.executeCommand ?? vscode.commands.executeCommand;
  const showInformationMessage =
    dependencies.showInformationMessage ?? vscode.window.showInformationMessage;
  const showErrorMessage = dependencies.showErrorMessage ?? vscode.window.showErrorMessage;

  return async () => {
    try {
      const editor = getActiveEditorOrThrow();
      const workspaceRoot = dependencies.getWorkspaceRoot();
      const sourceFile = getRelativeFilePath(workspaceRoot, editor.document.uri.fsPath);
      const notes = await dependencies.service.listNotes(workspaceRoot, sourceFile);

      dependencies.refreshNotesPanel();
      await executeCommand(notesViewFocusCommand());

      if (notes.length === 0) {
        await showInformationMessage(`No notes for ${sourceFile}.`);
      }
    } catch (error) {
      const message =
        error instanceof Error ? error.message : 'Failed to show notes for the current file.';

      await showErrorMessage(`FrilVault: ${message}`);
    }
  };
}

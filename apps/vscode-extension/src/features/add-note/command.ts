import * as vscode from 'vscode';

import { AddNoteService } from './service';
import { getConfiguredCliPath } from './cli';

export interface AddNoteCommandDependencies {
  getWorkspaceRoot: () => string;
  noteTreeDataProvider: { refresh(): void };
  decorationsProvider: { refresh(editor?: vscode.TextEditor): Promise<void> };
  promptNoteContent?: () => Promise<string | undefined>;
  showInformationMessage?: (message: string) => Thenable<string | undefined>;
  showErrorMessage?: (message: string) => Thenable<string | undefined>;
}

async function promptNoteContent(): Promise<string | undefined> {
  const content = await vscode.window.showInputBox({
    prompt: 'Enter a FrilVault note',
    ignoreFocusOut: true,
    validateInput(value) {
      return value.trim().length === 0 ? 'Note content is required.' : undefined;
    },
  });

  return content?.trim();
}

export function createAddNoteCommand(
  dependencies: AddNoteCommandDependencies,
): () => Promise<void> {
  const service = new AddNoteService();
  const getNoteContent = dependencies.promptNoteContent ?? promptNoteContent;
  const showInformationMessage =
    dependencies.showInformationMessage ?? vscode.window.showInformationMessage;
  const showErrorMessage = dependencies.showErrorMessage ?? vscode.window.showErrorMessage;

  return async () => {
    try {
      const editor = vscode.window.activeTextEditor;
      if (!editor) {
        throw new Error('No active editor.');
      }

      if (editor.document.uri.scheme !== 'file') {
        throw new Error('FrilVault only supports files on disk.');
      }

      const content = await getNoteContent();
      if (!content) {
        return;
      }

      const line = editor.selection.active.line + 1;
      const column = editor.selection.active.character + 1;
      const workspaceRoot = dependencies.getWorkspaceRoot();

      await service.execute({
        cliPath: getConfiguredCliPath(),
        workspaceRoot,
        sourceFile: editor.document.uri.fsPath,
        line,
        column,
        content,
      });

      dependencies.noteTreeDataProvider.refresh();
      await dependencies.decorationsProvider.refresh(editor);

      await showInformationMessage(`FrilVault note added at ${line}:${column}.`);
    } catch (error) {
      const message =
        error instanceof Error ? error.message : 'Failed to add FrilVault note.';

      await showErrorMessage(`FrilVault: ${message}`);
    }
  };
}

import * as vscode from 'vscode';

import { getActiveEditorOrThrow } from '../../utils/file';
import { AddNoteService } from './service';

export interface AddNoteCommandDependencies {
  getWorkspaceRoot: () => string;
  service: AddNoteService;
  refreshNotesPanel: () => void;
  refreshDecorations: (editor?: vscode.TextEditor) => Promise<void>;
  onNoteAdded?: () => Promise<void>;
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
  const getNoteContent = dependencies.promptNoteContent ?? promptNoteContent;
  const showInformationMessage =
    dependencies.showInformationMessage ?? vscode.window.showInformationMessage;
  const showErrorMessage = dependencies.showErrorMessage ?? vscode.window.showErrorMessage;

  return async () => {
    try {
      const editor = getActiveEditorOrThrow();
      const content = await getNoteContent();

      if (!content) {
        return;
      }

      const line = editor.selection.active.line + 1;
      const column = editor.selection.active.character + 1;

      await dependencies.service.execute({
        workspaceRoot: dependencies.getWorkspaceRoot(),
        sourceFile: editor.document.uri.fsPath,
        line,
        column,
        content,
      });

      dependencies.refreshNotesPanel();
      await dependencies.refreshDecorations(editor);
      await showInformationMessage(`FrilVault note added at ${line}:${column}.`);

      if (dependencies.onNoteAdded) {
        await dependencies.onNoteAdded();
      }
    } catch (error) {
      const message =
        error instanceof Error ? error.message : 'Failed to add FrilVault note.';

      await showErrorMessage(`FrilVault: ${message}`);
    }
  };
}

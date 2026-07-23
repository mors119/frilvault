import * as vscode from 'vscode';

import type { NoteView } from '../../types';
import type { CurrentFileNotesStore } from '../current-file/store';
import { resolveNoteLine as resolvePresentationNoteLine } from '../presentation/editorNoteView';

export function registerInlineNoteCodeLensProvider(
  context: vscode.ExtensionContext,
  store: CurrentFileNotesStore,
  getWorkspaceRoot: () => string,
  isEnabled: () => boolean,
  onRefresh: vscode.Event<void>,
): void {
  const provider: vscode.CodeLensProvider = {
    onDidChangeCodeLenses: onRefresh,
    provideCodeLenses(document) {
      if (!isEnabled()) {
        return [];
      }

      const relativePath = relativePathForDocument(document, getWorkspaceRoot());
      const notes = store.getSnapshot().notes.filter((note) => note.source_file === relativePath);
      const lenses: vscode.CodeLens[] = [];

      for (const note of notes) {
        const lineNumber = resolvePresentationNoteLine(note);

        if (lineNumber === undefined) {
          continue;
        }

        const line = lineNumber - 1;
        lenses.push(
          new vscode.CodeLens(new vscode.Range(line, 0, line, 0), {
            title: 'Edit FrilVault Note',
            command: 'frilvault.editNote',
            arguments: [note.note.id, note.source_file, note],
          }),
        );
      }

      const activeEditor = vscode.window.activeTextEditor;
      if (activeEditor && activeEditor.document.uri.toString() === document.uri.toString()) {
        const activeLine = activeEditor.selection.active.line;
        const hasNoteOnActiveLine = notes.some((note) => {
          const lineNumber = resolvePresentationNoteLine(note);

          return lineNumber !== undefined && lineNumber - 1 === activeLine;
        });

        if (!hasNoteOnActiveLine) {
          lenses.push(
            new vscode.CodeLens(new vscode.Range(activeLine, 0, activeLine, 0), {
              title: 'Create FrilVault Note Here',
              command: 'frilvault.createNoteHere',
            }),
          );
        }
      }

      return lenses;
    },
  };

  context.subscriptions.push(
    vscode.languages.registerCodeLensProvider({ scheme: 'file' }, provider),
  );
}

function relativePathForDocument(document: vscode.TextDocument, workspaceRoot: string): string {
  const configuredRoot = workspaceRoot.endsWith('/') ? workspaceRoot.slice(0, -1) : workspaceRoot;
  const filePath = document.uri.fsPath;

  if (filePath.startsWith(`${configuredRoot}/`)) {
    return filePath.slice(configuredRoot.length + 1);
  }

  return vscode.workspace.asRelativePath(document.uri, false);
}

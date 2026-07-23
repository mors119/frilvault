import * as vscode from 'vscode';

import type { GutterNoteActions } from './gutterActions';

export function registerGutterCommands(
  context: vscode.ExtensionContext,
  actions: GutterNoteActions,
): void {
  context.subscriptions.push(
    vscode.commands.registerCommand(
      'frilvault.gutter.showActions',
      async (line: number, sourceFile: string) => {
        await actions.showActions(line, sourceFile);
      },
    ),
    vscode.commands.registerCommand(
      'frilvault.gutter.viewNote',
      async (noteId: string, sourceFile: string) => {
        await actions.viewNote(noteId, sourceFile);
      },
    ),
    vscode.commands.registerCommand(
      'frilvault.gutter.editNote',
      async (noteId: string, sourceFile: string) => {
        await actions.editNote(noteId, sourceFile);
      },
    ),
    vscode.commands.registerCommand(
      'frilvault.gutter.deleteNote',
      async (noteId: string, sourceFile: string) => {
        await actions.deleteNote(noteId, sourceFile);
      },
    ),
    vscode.commands.registerCommand(
      'frilvault.gutter.copyLink',
      async (noteId: string, sourceFile: string) => {
        await actions.copyLink(noteId, sourceFile);
      },
    ),
  );
}

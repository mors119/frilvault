import * as vscode from 'vscode';

import type { CliClient } from '../../core/cliClient';
import { getRelativeFilePath } from '../../utils/file';
import { createLineNoteDecorationType, createSymbolNoteDecorationType } from './gutter';

export class FrilVaultDecorator implements vscode.Disposable {
  private readonly lineDecorationType: vscode.TextEditorDecorationType;

  private readonly symbolDecorationType: vscode.TextEditorDecorationType;

  private previousEditor: vscode.TextEditor | undefined;

  public constructor(
    extensionPath: string,
    private readonly cliClient: CliClient,
    private readonly getWorkspaceRoot: () => string,
  ) {
    this.lineDecorationType = createLineNoteDecorationType(extensionPath);
    this.symbolDecorationType = createSymbolNoteDecorationType(extensionPath);
  }

  public async refresh(editor = vscode.window.activeTextEditor): Promise<void> {
    if (this.previousEditor && this.previousEditor !== editor) {
      this.clear(this.previousEditor);
    }

    if (!editor || editor.document.uri.scheme !== 'file') {
      this.previousEditor = editor;
      return;
    }

    const notes = await this.cliClient.listNotes(
      this.getWorkspaceRoot(),
      getRelativeFilePath(this.getWorkspaceRoot(), editor.document.uri.fsPath),
    );

    const lineDecorations: vscode.DecorationOptions[] = [];
    const symbolDecorations: vscode.DecorationOptions[] = [];

    for (const note of notes) {
      const line =
        note.note.anchor.type === 'Line'
          ? (note.note.anchor.line ?? 1) - 1
          : (note.note.anchor.line_hint ?? 1) - 1;

      if (line < 0 || line >= editor.document.lineCount) {
        continue;
      }

      const decoration = {
        range: editor.document.lineAt(line).range,
        hoverMessage: new vscode.MarkdownString(note.note.content),
      };

      if (note.note.anchor.type === 'Line') {
        lineDecorations.push(decoration);
      } else {
        symbolDecorations.push(decoration);
      }
    }

    editor.setDecorations(this.lineDecorationType, lineDecorations);
    editor.setDecorations(this.symbolDecorationType, symbolDecorations);
    this.previousEditor = editor;
  }

  public clear(editor = vscode.window.activeTextEditor): void {
    editor?.setDecorations(this.lineDecorationType, []);
    editor?.setDecorations(this.symbolDecorationType, []);
  }

  public dispose(): void {
    this.lineDecorationType.dispose();
    this.symbolDecorationType.dispose();
  }
}

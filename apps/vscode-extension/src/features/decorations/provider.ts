import * as path from 'node:path';
import * as vscode from 'vscode';

import { DecorationsService } from './service';

export class DecorationsProvider implements vscode.Disposable {
  private readonly decorationType: vscode.TextEditorDecorationType;

  private readonly service = new DecorationsService();

  private previousEditor: vscode.TextEditor | undefined;

  public constructor(
    private readonly extensionPath: string,
    private readonly getWorkspaceRoot: () => string,
  ) {
    this.decorationType = vscode.window.createTextEditorDecorationType({
      gutterIconPath: vscode.Uri.file(
        path.join(extensionPath, 'media', 'frilvault-note-gutter.svg'),
      ),
      gutterIconSize: 'contain',
      overviewRulerLane: vscode.OverviewRulerLane.Right,
      overviewRulerColor: new vscode.ThemeColor('editorInfo.foreground'),
    });
  }

  public async refresh(editor = vscode.window.activeTextEditor): Promise<void> {
    if (this.previousEditor && this.previousEditor !== editor) {
      this.clear(this.previousEditor);
    }

    if (!editor || editor.document.uri.scheme !== 'file') {
      this.previousEditor = editor;
      return;
    }

    const workspaceRoot = this.getWorkspaceRoot();
    const notes = await this.service.listNotes({
      workspaceRoot,
      sourceFile: editor.document.uri.fsPath,
    });

    const decorations = notes.flatMap((noteView) => {
      if (noteView.note.anchor.type !== 'Line') {
        return [];
      }

      const line = Math.max(noteView.note.anchor.line - 1, 0);
      if (line >= editor.document.lineCount) {
        return [];
      }

      return [
        {
          range: editor.document.lineAt(line).range,
          hoverMessage: new vscode.MarkdownString(noteView.note.content),
        },
      ];
    });

    editor.setDecorations(this.decorationType, decorations);
    this.previousEditor = editor;
  }

  public clear(editor = vscode.window.activeTextEditor): void {
    editor?.setDecorations(this.decorationType, []);
  }

  public dispose(): void {
    this.decorationType.dispose();
  }
}

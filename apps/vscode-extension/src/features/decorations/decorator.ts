import * as vscode from 'vscode';

import type { CurrentFileNotesStore } from '../current-file/store';
import { formatNoteHover } from '../../utils/noteMarkdown';
import { createLineNoteDecorationType, createSymbolNoteDecorationType } from './gutter';

export class FrilVaultDecorator implements vscode.Disposable {
  private readonly lineDecorationType: vscode.TextEditorDecorationType;

  private readonly symbolDecorationType: vscode.TextEditorDecorationType;

  private previousEditor: vscode.TextEditor | undefined;

  private pendingEditorUri: string | undefined;

  public constructor(
    extensionPath: string,
    private readonly store: CurrentFileNotesStore,
    private readonly getWorkspaceRoot: () => string,
    private readonly isEnabled: () => boolean = () => true,
  ) {
    this.lineDecorationType = createLineNoteDecorationType(extensionPath);
    this.symbolDecorationType = createSymbolNoteDecorationType(extensionPath);
  }

  public async refresh(editor = vscode.window.activeTextEditor): Promise<void> {
    if (!this.isEnabled()) {
      this.clear(editor);
      this.previousEditor = editor;
      this.pendingEditorUri = undefined;
      return;
    }

    if (this.previousEditor && this.previousEditor !== editor) {
      this.clear(this.previousEditor);
    }

    if (!editor || editor.document.uri.scheme !== 'file') {
      this.previousEditor = editor;
      this.pendingEditorUri = undefined;
      return;
    }

    const editorUri = editor.document.uri.toString();
    this.pendingEditorUri = editorUri;

    const snapshot = this.store.getSnapshot();
    if (snapshot.loading || snapshot.editorDocumentUri !== editorUri) {
      this.clear(editor);
      return;
    }

    this.renderNotes(editor, snapshot.notes);
  }

  public clear(editor = vscode.window.activeTextEditor): void {
    editor?.setDecorations(this.lineDecorationType, []);
    editor?.setDecorations(this.symbolDecorationType, []);
  }

  public dispose(): void {
    this.lineDecorationType.dispose();
    this.symbolDecorationType.dispose();
  }

  private renderNotes(editor: vscode.TextEditor, notes: ReturnType<CurrentFileNotesStore['getSnapshot']>['notes']): void {
    if (this.pendingEditorUri !== editor.document.uri.toString()) {
      return;
    }

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
        hoverMessage: formatNoteHover(note, this.getWorkspaceRoot()),
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
    this.pendingEditorUri = undefined;
  }
}

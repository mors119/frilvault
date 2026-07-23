import * as vscode from 'vscode';

import type { CurrentFileNotesStore } from '../current-file/store';
import { aggregateNotesByLine } from './aggregate';
import { formatGutterHoverSummary } from './gutterHover';
import {
  createMarkerDecorationType,
  getConfiguredMarkerStyle,
  markerRenderOptions,
  type GutterMarkerStyle,
} from './markerStyle';
import type { GutterNoteRegistry } from './registry';

export class FrilVaultDecorator implements vscode.Disposable {
  private decorationType: vscode.TextEditorDecorationType;

  private markerStyle: GutterMarkerStyle;

  private previousEditor: vscode.TextEditor | undefined;

  private pendingEditorUri: string | undefined;

  private readonly configListener: vscode.Disposable;

  public constructor(
    private readonly extensionPath: string,
    private readonly store: CurrentFileNotesStore,
    private readonly registry: GutterNoteRegistry,
    private readonly getWorkspaceRoot: () => string,
    private readonly isEnabled: () => boolean = () => true,
  ) {
    this.markerStyle = getConfiguredMarkerStyle();
    this.decorationType = createMarkerDecorationType(this.extensionPath, this.markerStyle);
    this.configListener = vscode.workspace.onDidChangeConfiguration((event) => {
      if (!event.affectsConfiguration('frilvault.gutterMarkerStyle')) {
        return;
      }

      this.recreateDecorationType();
      void this.refresh();
    });
  }

  public async refresh(editor = vscode.window.activeTextEditor): Promise<void> {
    if (!this.isEnabled()) {
      this.clear(editor);
      this.previousEditor = editor;
      this.pendingEditorUri = undefined;
      this.registry.clear(editor?.document.uri.toString());
      return;
    }

    if (this.previousEditor && this.previousEditor !== editor) {
      this.clear(this.previousEditor);
      this.registry.clear(this.previousEditor.document.uri.toString());
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

    this.renderNotes(editor, snapshot.notes, snapshot.sourceFile ?? '');
  }

  public clear(editor = vscode.window.activeTextEditor): void {
    editor?.setDecorations(this.decorationType, []);
  }

  public dispose(): void {
    this.configListener.dispose();
    this.decorationType.dispose();
  }

  private renderNotes(
    editor: vscode.TextEditor,
    notes: ReturnType<CurrentFileNotesStore['getSnapshot']>['notes'],
    sourceFile: string,
  ): void {
    if (this.pendingEditorUri !== editor.document.uri.toString()) {
      return;
    }

    const groups = aggregateNotesByLine(notes, editor.document.lineCount);
    const lineNotes = new Map<number, ReturnType<typeof aggregateNotesByLine>[number]['notes']>();

    const decorations: vscode.DecorationOptions[] = groups.map((group) => {
      lineNotes.set(group.line, group.notes);

      return {
        range: editor.document.lineAt(group.line).range,
        hoverMessage: formatGutterHoverSummary(group.notes, sourceFile),
        renderOptions: markerRenderOptions(this.markerStyle, group.notes.length),
      };
    });

    this.registry.set(editor.document.uri.toString(), lineNotes);
    editor.setDecorations(this.decorationType, decorations);
    this.previousEditor = editor;
    this.pendingEditorUri = undefined;
  }

  private recreateDecorationType(): void {
    this.decorationType.dispose();
    this.markerStyle = getConfiguredMarkerStyle();
    this.decorationType = createMarkerDecorationType(this.extensionPath, this.markerStyle);
  }
}

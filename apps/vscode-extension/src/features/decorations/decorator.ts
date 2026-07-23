import * as vscode from 'vscode';

import type { NoteView } from '../../types';
import {
  formatInlineNotesPreview,
  getInlineNotesMaxLength,
  resolveNoteLine,
  resolveNoteRange,
  showInlineLineNotes,
  showInlineSymbolNotes,
} from '../presentation/editorNoteView';
import { buildEditorNotesHoverParts } from '../presentation/noteHover';
import { getConfiguredPreviewLength } from '../hover/richHover';
import { aggregateNotesByLine } from './aggregate';
import { createSymbolNoteDecorationType } from './gutter';
import {
  createMarkerDecorationType,
  getConfiguredMarkerStyle,
  markerRenderOptions,
  type GutterMarkerStyle,
} from './markerStyle';
import type { GutterNoteRegistry } from './registry';

export class FrilVaultDecorator implements vscode.Disposable {
  private gutterDecorationType: vscode.TextEditorDecorationType;

  private inlinePreviewDecorationType: vscode.TextEditorDecorationType;

  private symbolDecorationType: vscode.TextEditorDecorationType;

  private markerStyle: GutterMarkerStyle;

  private previousEditor: vscode.TextEditor | undefined;

  private pendingEditorUri: string | undefined;

  private readonly configListener: vscode.Disposable;

  public constructor(
    private readonly extensionPath: string,
    private readonly store: import('../current-file/store').CurrentFileNotesStore,
    private readonly registry: GutterNoteRegistry,
    private readonly getWorkspaceRoot: () => string,
    private readonly isEnabled: () => boolean = () => true,
  ) {
    this.markerStyle = getConfiguredMarkerStyle();
    this.gutterDecorationType = createMarkerDecorationType(this.extensionPath, this.markerStyle);
    this.inlinePreviewDecorationType = vscode.window.createTextEditorDecorationType({
      after: {
        margin: '0 0 0 1em',
        color: new vscode.ThemeColor('editorCodeLens.foreground'),
        fontStyle: 'italic',
      },
    });
    this.symbolDecorationType = createSymbolNoteDecorationType(this.extensionPath);
    this.configListener = vscode.workspace.onDidChangeConfiguration((event) => {
      if (
        !event.affectsConfiguration('frilvault.gutterMarkerStyle') &&
        !event.affectsConfiguration('frilvault.inlineNotes') &&
        !event.affectsConfiguration('frilvault.inlineLineNotes')
      ) {
        return;
      }

      if (event.affectsConfiguration('frilvault.gutterMarkerStyle')) {
        this.recreateGutterDecorationType();
      }

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
    editor?.setDecorations(this.gutterDecorationType, []);
    editor?.setDecorations(this.inlinePreviewDecorationType, []);
    editor?.setDecorations(this.symbolDecorationType, []);
  }

  public dispose(): void {
    this.configListener.dispose();
    this.gutterDecorationType.dispose();
    this.inlinePreviewDecorationType.dispose();
    this.symbolDecorationType.dispose();
  }

  private renderNotes(
    editor: vscode.TextEditor,
    notes: NoteView[],
    sourceFile: string,
  ): void {
    if (this.pendingEditorUri !== editor.document.uri.toString()) {
      return;
    }

    const workspaceRoot = this.getWorkspaceRoot();
    const groups = aggregateNotesByLine(notes, editor.document.lineCount);
    const lineNotes = new Map<number, NoteView[]>();
    const gutterDecorations: vscode.DecorationOptions[] = groups.map((group) => {
      lineNotes.set(group.line, group.notes);

      return {
        range: editor.document.lineAt(group.line).range,
        hoverMessage: buildEditorNotesHoverParts(
          group.notes,
          workspaceRoot,
          sourceFile,
          getConfiguredPreviewLength(),
        ).contents,
        renderOptions: markerRenderOptions(this.markerStyle, group.notes.length),
      };
    });

    this.registry.set(editor.document.uri.toString(), lineNotes);
    editor.setDecorations(this.gutterDecorationType, gutterDecorations);
    editor.setDecorations(
      this.inlinePreviewDecorationType,
      this.buildInlinePreviewDecorations(editor, notes, sourceFile, workspaceRoot),
    );
    editor.setDecorations(
      this.symbolDecorationType,
      this.buildSymbolGutterDecorations(editor, notes),
    );
    this.previousEditor = editor;
    this.pendingEditorUri = undefined;
  }

  private buildInlinePreviewDecorations(
    editor: vscode.TextEditor,
    notes: NoteView[],
    sourceFile: string,
    workspaceRoot: string,
  ): vscode.DecorationOptions[] {
    const maxLength = getInlineNotesMaxLength();
    const byLine = new Map<number, NoteView[]>();

    for (const note of notes) {
      const isLineNote = note.note.anchor.type === 'Line' && showInlineLineNotes();
      const isSymbolNote =
        note.note.anchor.type === 'Symbol' && note.resolved && showInlineSymbolNotes();

      if (!isLineNote && !isSymbolNote) {
        continue;
      }

      const lineNumber = resolveNoteLine(note);

      if (lineNumber === undefined) {
        continue;
      }

      const zeroBasedLine = lineNumber - 1;

      if (zeroBasedLine < 0 || zeroBasedLine >= editor.document.lineCount) {
        continue;
      }

      const group = byLine.get(zeroBasedLine) ?? [];
      group.push(note);
      byLine.set(zeroBasedLine, group);
    }

    return [...byLine.entries()].map(([line, groupedNotes]) => ({
      range: new vscode.Range(line, Number.MAX_SAFE_INTEGER, line, Number.MAX_SAFE_INTEGER),
      hoverMessage: buildEditorNotesHoverParts(
        groupedNotes,
        workspaceRoot,
        sourceFile,
        getConfiguredPreviewLength(),
      ).contents,
      renderOptions: {
        after: {
          contentText: formatInlineNotesPreview(groupedNotes, maxLength),
          color: new vscode.ThemeColor('editorInfo.foreground'),
          fontStyle: 'italic',
          textDecoration: 'none',
        },
      },
    }));
  }

  private buildSymbolGutterDecorations(
    editor: vscode.TextEditor,
    notes: NoteView[],
  ): vscode.DecorationOptions[] {
    const decorations: vscode.DecorationOptions[] = [];

    for (const note of notes) {
      if (note.note.anchor.type !== 'Symbol' || !note.resolved) {
        continue;
      }

      const range = resolveNoteRange(note, editor.document.lineCount);

      if (!range) {
        continue;
      }

      decorations.push({ range });
    }

    return decorations;
  }

  private recreateGutterDecorationType(): void {
    this.gutterDecorationType.dispose();
    this.markerStyle = getConfiguredMarkerStyle();
    this.gutterDecorationType = createMarkerDecorationType(this.extensionPath, this.markerStyle);
  }
}

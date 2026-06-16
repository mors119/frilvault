import * as path from 'node:path';
import * as vscode from 'vscode';

import type { NoteView } from '../../types';

export class NotesFileGroupItem extends vscode.TreeItem {
  public constructor(
    public readonly sourceFile: string,
    public readonly notes: NoteView[],
  ) {
    super(sourceFile, vscode.TreeItemCollapsibleState.Expanded);
    this.description = `${notes.length} note${notes.length === 1 ? '' : 's'}`;
    this.iconPath = new vscode.ThemeIcon('file');
    this.contextValue = 'frilvault.notesFileGroup';
  }
}

export class NotesPanelItem extends vscode.TreeItem {
  public constructor(
    public readonly noteView: NoteView,
    public readonly workspaceRoot: string,
  ) {
    super(createPreview(noteView), vscode.TreeItemCollapsibleState.None);

    this.description = createDescription(noteView);
    this.tooltip = new vscode.MarkdownString(
      `**${path.basename(noteView.source_file)}**\n\n${noteView.note.content}`,
    );
    this.contextValue = 'frilvault.note';
    this.iconPath = new vscode.ThemeIcon('note');
    this.command = {
      command: 'frilvault.notesPanel.openNote',
      title: 'Open FrilVault Note',
      arguments: [noteView],
    };
  }
}

function createPreview(noteView: NoteView): string {
  return noteView.note.content.length > 60
    ? `${noteView.note.content.slice(0, 57)}...`
    : noteView.note.content;
}

function createDescription(noteView: NoteView): string {
  if (noteView.note.anchor.type === 'Line') {
    return `L${noteView.note.anchor.line ?? 1}`;
  }

  const lineHint =
    typeof noteView.note.anchor.line_hint === 'number'
      ? `L${noteView.note.anchor.line_hint}`
      : 'Symbol';
  return `${lineHint} ${noteView.note.anchor.name ?? ''}`.trim();
}

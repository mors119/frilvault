import * as vscode from 'vscode';

import type { NoteView } from '../../types';
import { formatNoteHover } from '../../utils/noteMarkdown';

export type AnchorGroupKind = 'Line' | 'Symbol';

export class NotesAnchorGroupItem extends vscode.TreeItem {
  public constructor(
    public readonly kind: AnchorGroupKind,
    public readonly notes: NoteView[],
  ) {
    const label = kind === 'Line' ? 'Line Notes' : 'Symbol Notes';

    super(label, vscode.TreeItemCollapsibleState.Expanded);
    this.description = `${notes.length}`;
    this.iconPath = new vscode.ThemeIcon(
      kind === 'Line' ? 'list-unordered' : 'symbol-method',
    );
    this.contextValue =
      kind === 'Line' ? 'frilvault.notesLineGroup' : 'frilvault.notesSymbolGroup';
  }
}

export class NotesPanelItem extends vscode.TreeItem {
  public constructor(
    public readonly noteView: NoteView,
    public readonly workspaceRoot: string,
  ) {
    super(createPreview(noteView), vscode.TreeItemCollapsibleState.None);

    this.description = createDescription(noteView);
    this.tooltip = formatNoteHover(noteView, workspaceRoot);
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

  const resolvedLine = noteView.resolved?.line ?? noteView.note.anchor.line_hint;
  const lineHint =
    typeof resolvedLine === 'number' ? `L${resolvedLine}` : 'Symbol';

  return `${lineHint} ${noteView.note.anchor.name ?? ''}`.trim();
}

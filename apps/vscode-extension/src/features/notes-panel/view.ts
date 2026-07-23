import * as vscode from 'vscode';

import { COMMAND_IDS, VIEW_ITEM_CONTEXT } from '../../constants/ids';
import type { NoteView } from '../../types';
import { formatNoteHover } from '../../utils/noteMarkdown';

export type AnchorGroupKind = 'Line' | 'Symbol' | 'Unresolved';

export class NotesFileHeaderItem extends vscode.TreeItem {
  public constructor(sourceFile: string) {
    super(sourceFile, vscode.TreeItemCollapsibleState.None);
    this.description = 'Active file';
    this.iconPath = new vscode.ThemeIcon('file');
    this.contextValue = VIEW_ITEM_CONTEXT.notesFileHeader;
  }
}

export class NotesStatusItem extends vscode.TreeItem {
  public constructor(message: string, icon: string, commandId?: string) {
    super(message, vscode.TreeItemCollapsibleState.None);
    this.iconPath = new vscode.ThemeIcon(icon);
    this.contextValue = VIEW_ITEM_CONTEXT.notesStatus;

    if (commandId) {
      this.command = {
        command: commandId,
        title: message,
      };
    }
  }
}

export class NotesSymbolGroupItem extends vscode.TreeItem {
  public constructor(
    public readonly symbolName: string,
    public readonly notes: NoteView[],
  ) {
    super(`Symbol: ${symbolName}`, vscode.TreeItemCollapsibleState.Expanded);
    this.description = `${notes.length}`;
    this.iconPath = new vscode.ThemeIcon('symbol-method');
    this.contextValue = VIEW_ITEM_CONTEXT.notesSymbolGroup;
  }
}

export class NotesAnchorGroupItem extends vscode.TreeItem {
  public constructor(
    public readonly kind: AnchorGroupKind,
    public readonly notes: NoteView[],
  ) {
    const label =
      kind === 'Line'
        ? 'Line Notes'
        : kind === 'Unresolved'
          ? 'Unresolved Anchors'
          : 'Symbol Notes';

    super(label, vscode.TreeItemCollapsibleState.Expanded);
    this.description = `${notes.length}`;
    this.iconPath = new vscode.ThemeIcon(
      kind === 'Line'
        ? 'list-unordered'
        : kind === 'Unresolved'
          ? 'warning'
          : 'symbol-method',
    );
    this.contextValue =
      kind === 'Line'
        ? VIEW_ITEM_CONTEXT.notesLineGroup
        : kind === 'Unresolved'
          ? VIEW_ITEM_CONTEXT.notesUnresolvedGroup
          : VIEW_ITEM_CONTEXT.notesSymbolGroup;
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
    this.contextValue = VIEW_ITEM_CONTEXT.note;
    this.iconPath = new vscode.ThemeIcon('note');
    this.command = {
      command: COMMAND_IDS.notesPanelOpenNote,
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
    typeof resolvedLine === 'number' ? `L${resolvedLine}` : 'Unresolved';

  return `${lineHint} ${noteView.note.anchor.name ?? ''}`.trim();
}

import * as vscode from 'vscode';

import type { NoteView } from '../../types';
import { getRelativeFilePath } from '../../utils/file';
import { NotesPanelService } from './service';
import { NotesAnchorGroupItem, NotesPanelItem } from './view';

type TreeNode = NotesAnchorGroupItem | NotesPanelItem;

export class FrilVaultNotesProvider implements vscode.TreeDataProvider<TreeNode> {
  private readonly onDidChangeTreeDataEmitter = new vscode.EventEmitter<void>();

  public readonly onDidChangeTreeData = this.onDidChangeTreeDataEmitter.event;

  public constructor(
    private readonly service: NotesPanelService,
    private readonly getWorkspaceRoot: () => string,
  ) {}

  public refresh(): void {
    this.onDidChangeTreeDataEmitter.fire();
  }

  public getTreeItem(element: TreeNode): vscode.TreeItem {
    return element;
  }

  public async getChildren(element?: TreeNode): Promise<TreeNode[]> {
    if (element instanceof NotesAnchorGroupItem) {
      return element.notes.map((note) => new NotesPanelItem(note, this.getWorkspaceRoot()));
    }

    const editor = vscode.window.activeTextEditor;

    if (!editor || editor.document.uri.scheme !== 'file') {
      return [];
    }

    const workspaceRoot = this.getWorkspaceRoot();
    const sourceFile = getRelativeFilePath(workspaceRoot, editor.document.uri.fsPath);
    const notes = await this.service.listNotes(workspaceRoot, sourceFile);

    return partitionNotesByAnchor(notes);
  }
}

function partitionNotesByAnchor(notes: NoteView[]): NotesAnchorGroupItem[] {
  const lineNotes = notes
    .filter((note) => note.note.anchor.type === 'Line')
    .sort(
      (left, right) =>
        (left.note.anchor.line ?? 0) - (right.note.anchor.line ?? 0),
    );
  const symbolNotes = notes
    .filter((note) => note.note.anchor.type === 'Symbol')
    .sort((left, right) =>
      (left.note.anchor.name ?? '').localeCompare(right.note.anchor.name ?? ''),
    );

  const groups: NotesAnchorGroupItem[] = [];

  if (lineNotes.length > 0) {
    groups.push(new NotesAnchorGroupItem('Line', lineNotes));
  }

  if (symbolNotes.length > 0) {
    groups.push(new NotesAnchorGroupItem('Symbol', symbolNotes));
  }

  return groups;
}

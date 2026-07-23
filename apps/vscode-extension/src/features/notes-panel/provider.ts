import * as vscode from 'vscode';

import {
  CurrentFileNotesStore,
  partitionNotesByAnchor,
} from '../current-file/store';
import { NotesAnchorGroupItem, NotesPanelItem } from './view';

type TreeNode = NotesAnchorGroupItem | NotesPanelItem;

export class FrilVaultNotesProvider implements vscode.TreeDataProvider<TreeNode> {
  private readonly onDidChangeTreeDataEmitter = new vscode.EventEmitter<void>();

  public readonly onDidChangeTreeData = this.onDidChangeTreeDataEmitter.event;

  public constructor(
    private readonly store: CurrentFileNotesStore,
    private readonly getWorkspaceRoot: () => string,
    private readonly isEnabled: () => boolean = () => true,
  ) {}

  public refresh(): void {
    this.onDidChangeTreeDataEmitter.fire();
  }

  public getTreeItem(element: TreeNode): vscode.TreeItem {
    return element;
  }

  public async getChildren(element?: TreeNode): Promise<TreeNode[]> {
    if (!this.isEnabled()) {
      return [];
    }

    if (element instanceof NotesAnchorGroupItem) {
      return element.notes.map((note) => new NotesPanelItem(note, this.getWorkspaceRoot()));
    }

    const { lineNotes, symbolNotes } = partitionNotesByAnchor(this.store.getSnapshot().notes);
    const groups: NotesAnchorGroupItem[] = [];

    if (lineNotes.length > 0) {
      groups.push(new NotesAnchorGroupItem('Line', lineNotes));
    }

    if (symbolNotes.length > 0) {
      groups.push(new NotesAnchorGroupItem('Symbol', symbolNotes));
    }

    return groups;
  }
}

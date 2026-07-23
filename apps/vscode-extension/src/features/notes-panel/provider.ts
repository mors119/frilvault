import * as vscode from 'vscode';

import { COMMAND_IDS } from '../../constants/ids';
import {
  CurrentFileNotesStore,
} from '../current-file/store';
import { groupNotesByAnchor } from './presentation';
import {
  NotesAnchorGroupItem,
  NotesFileHeaderItem,
  NotesPanelItem,
  NotesStatusItem,
  NotesSymbolGroupItem,
} from './view';

type TreeNode =
  | NotesFileHeaderItem
  | NotesStatusItem
  | NotesSymbolGroupItem
  | NotesAnchorGroupItem
  | NotesPanelItem;

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
      return [new NotesStatusItem('FrilVault is disabled for this workspace.', 'debug-pause')];
    }

    const snapshot = this.store.getSnapshot();

    if (element instanceof NotesSymbolGroupItem || element instanceof NotesAnchorGroupItem) {
      return element.notes.map((note) => new NotesPanelItem(note, this.getWorkspaceRoot()));
    }

    if (element) {
      return [];
    }

    if (snapshot.loading) {
      return [new NotesStatusItem('Loading notes for the active file...', 'loading~spin')];
    }

    if (snapshot.error) {
      return [new NotesStatusItem(snapshot.error, 'error')];
    }

    if (!snapshot.sourceFile) {
      return [
        new NotesStatusItem('Open a workspace file to view its notes.', 'info'),
      ];
    }

    if (snapshot.notes.length === 0) {
      return [
        new NotesFileHeaderItem(snapshot.sourceFile),
        new NotesStatusItem(
          'No FrilVault notes are attached to this file.',
          'note',
          COMMAND_IDS.addNote,
        ),
      ];
    }

    const groups = groupNotesByAnchor(snapshot.notes);
    const children: TreeNode[] = [new NotesFileHeaderItem(snapshot.sourceFile)];

    for (const group of groups.symbolGroups) {
      children.push(new NotesSymbolGroupItem(group.name, group.notes));
    }

    if (groups.lineNotes.length > 0) {
      children.push(new NotesAnchorGroupItem('Line', groups.lineNotes));
    }

    if (groups.unresolvedNotes.length > 0) {
      children.push(new NotesAnchorGroupItem('Unresolved', groups.unresolvedNotes));
    }

    return children;
  }
}

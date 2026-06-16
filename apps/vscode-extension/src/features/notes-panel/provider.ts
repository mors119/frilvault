import * as vscode from 'vscode';

import { getRelativeFilePath } from '../../utils/file';
import { NotesPanelService } from './service';
import { NotesFileGroupItem, NotesPanelItem } from './view';

type TreeNode = NotesFileGroupItem | NotesPanelItem;

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
    if (element instanceof NotesFileGroupItem) {
      return element.notes.map((note) => new NotesPanelItem(note, this.getWorkspaceRoot()));
    }

    const editor = vscode.window.activeTextEditor;

    if (!editor || editor.document.uri.scheme !== 'file') {
      return [];
    }

    const workspaceRoot = this.getWorkspaceRoot();
    const sourceFile = getRelativeFilePath(workspaceRoot, editor.document.uri.fsPath);
    const notes = await this.service.listNotes(workspaceRoot, sourceFile);

    if (notes.length === 0) {
      return [];
    }

    return [new NotesFileGroupItem(sourceFile, notes)];
  }
}

import * as path from 'node:path';
import * as vscode from 'vscode';

import { getConfiguredCliPath } from '../add-note/cli';
import type { NoteView } from '../../types';
import { NotesPanelService } from './service';

export class FrilVaultNoteTreeItem extends vscode.TreeItem {
  public constructor(
    public readonly noteView: NoteView,
    public readonly sourceFile: string,
  ) {
    super(createTreeLabel(noteView), vscode.TreeItemCollapsibleState.None);

    this.description = createDescription(noteView);
    this.tooltip = createTooltip(noteView, sourceFile);
    this.contextValue = 'frilvault.note';
    this.iconPath = new vscode.ThemeIcon('note');
    this.command = {
      command: 'frilvault.notesPanel.openNote',
      title: 'Open FrilVault Note',
      arguments: [noteView],
    };
  }
}

export class FrilVaultNotesProvider
  implements vscode.TreeDataProvider<FrilVaultNoteTreeItem>
{
  private readonly onDidChangeTreeDataEmitter = new vscode.EventEmitter<void>();

  private readonly service = new NotesPanelService();

  public readonly onDidChangeTreeData = this.onDidChangeTreeDataEmitter.event;

  public constructor(private readonly getWorkspaceRoot: () => string) {}

  public refresh(): void {
    this.onDidChangeTreeDataEmitter.fire();
  }

  public getTreeItem(element: FrilVaultNoteTreeItem): vscode.TreeItem {
    return element;
  }

  public async getChildren(): Promise<FrilVaultNoteTreeItem[]> {
    const editor = vscode.window.activeTextEditor;

    if (!editor || editor.document.uri.scheme !== 'file') {
      return [];
    }

    const workspaceRoot = this.getWorkspaceRoot();
    const sourceFile = editor.document.uri.fsPath;

    try {
      const notes = await this.service.listNotes({
        cliPath: getConfiguredCliPath(),
        workspaceRoot,
        sourceFile,
      });

      return notes.map((noteView) => new FrilVaultNoteTreeItem(noteView, sourceFile));
    } catch (error) {
      const message =
        error instanceof Error ? error.message : 'Failed to load FrilVault notes.';

      void vscode.window.showErrorMessage(`FrilVault: ${message}`);

      return [];
    }
  }
}

function createTreeLabel(noteView: NoteView): string {
  return noteView.note.content.length > 60
    ? `${noteView.note.content.slice(0, 57)}...`
    : noteView.note.content;
}

function createDescription(noteView: NoteView): string {
  if (noteView.note.anchor.type === 'Line') {
    return `L${noteView.note.anchor.line}`;
  }

  const symbolName = noteView.note.anchor.name;
  const lineHint =
    typeof noteView.note.anchor.line_hint === 'number'
      ? `L${noteView.note.anchor.line_hint}`
      : noteView.note.anchor.kind;

  return `${lineHint} ${symbolName}`;
}

function createTooltip(noteView: NoteView, sourceFile: string): vscode.MarkdownString {
  const anchorDescription =
    noteView.note.anchor.type === 'Line'
      ? `Line ${noteView.note.anchor.line}, Column ${noteView.note.anchor.column}`
      : `${noteView.note.anchor.kind}: ${noteView.note.anchor.name}`;

  return new vscode.MarkdownString(
    `**${path.basename(sourceFile)}**\n\n${anchorDescription}\n\n${noteView.note.content}`,
  );
}

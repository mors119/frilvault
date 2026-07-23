import * as vscode from 'vscode';

import { notesAtLine, type CurrentFileNotesStore } from '../current-file/store';
import { formatNoteHover } from '../../utils/noteMarkdown';

export class FrilVaultHoverProvider implements vscode.HoverProvider {
  public constructor(
    private readonly store: CurrentFileNotesStore,
    private readonly getWorkspaceRoot: () => string,
    private readonly isEnabled: () => boolean = () => true,
  ) {}

  public provideHover(
    document: vscode.TextDocument,
    position: vscode.Position,
  ): vscode.Hover | undefined {
    if (!this.isEnabled() || document.uri.scheme !== 'file') {
      return undefined;
    }

    const matched = notesAtLine(this.store.notesForDocument(document), position.line);

    if (matched.length === 0) {
      return undefined;
    }

    if (matched.length === 1) {
      return new vscode.Hover(formatNoteHover(matched[0], this.getWorkspaceRoot()));
    }

    const markdown = new vscode.MarkdownString();

    for (const [index, note] of matched.entries()) {
      if (index > 0) {
        markdown.appendMarkdown('\n\n---\n\n');
      }

      markdown.appendMarkdown(formatNoteHover(note, this.getWorkspaceRoot()).value);
    }

    return new vscode.Hover(markdown);
  }
}

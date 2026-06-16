import * as vscode from 'vscode';

import type { CliClient } from '../../core/cliClient';
import { getRelativeFilePath } from '../../utils/file';

export class FrilVaultHoverProvider implements vscode.HoverProvider {
  public constructor(
    private readonly cliClient: CliClient,
    private readonly getWorkspaceRoot: () => string,
  ) {}

  public async provideHover(
    document: vscode.TextDocument,
    position: vscode.Position,
  ): Promise<vscode.Hover | undefined> {
    if (document.uri.scheme !== 'file') {
      return undefined;
    }

    const notes = await this.cliClient.searchNotes({
      workspaceRoot: this.getWorkspaceRoot(),
      sourceFile: getRelativeFilePath(this.getWorkspaceRoot(), document.uri.fsPath),
    });

    const matched = notes.filter((note) => {
      if (note.note.anchor.type === 'Line') {
        return (note.note.anchor.line ?? 1) - 1 === position.line;
      }

      return (note.note.anchor.line_hint ?? 1) - 1 === position.line;
    });

    if (matched.length === 0) {
      return undefined;
    }

    const markdown = matched
      .map((note) => {
        const type = note.note.anchor.type;
        return `**FrilVault Note**\n---\nType: ${type}\nContent: ${note.note.content}`;
      })
      .join('\n\n');

    return new vscode.Hover(new vscode.MarkdownString(markdown));
  }
}

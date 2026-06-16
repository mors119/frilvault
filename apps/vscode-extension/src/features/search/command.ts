import * as vscode from 'vscode';

import type { CliClient } from '../../core/cliClient';
import { revealNote } from '../../utils/file';

export function createSearchCommand(
  cliClient: CliClient,
  getWorkspaceRoot: () => string,
): () => Promise<void> {
  return async () => {
    const keyword = await vscode.window.showInputBox({
      prompt: 'Search FrilVault notes',
      ignoreFocusOut: true,
    });

    if (!keyword || keyword.trim().length === 0) {
      return;
    }

    const workspaceRoot = getWorkspaceRoot();
    const results = await cliClient.searchNotes({
      workspaceRoot,
      keyword: keyword.trim(),
    });

    if (results.length === 0) {
      await vscode.window.showInformationMessage(`No notes found for "${keyword}".`);
      return;
    }

    const picked = await vscode.window.showQuickPick(
      results.map((note) => ({
        label: note.note.content,
        description: note.source_file,
        detail:
          note.note.anchor.type === 'Line'
            ? `Line ${note.note.anchor.line ?? 1}, Column ${note.note.anchor.column ?? 1}`
            : `${note.note.anchor.name ?? 'Symbol'} `,
        note,
      })),
      { placeHolder: `Found ${results.length} note(s)` },
    );

    if (picked) {
      await revealNote(picked.note, workspaceRoot);
    }
  };
}

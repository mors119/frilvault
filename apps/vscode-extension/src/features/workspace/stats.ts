import * as vscode from 'vscode';

import type { CliClient } from '../../core/cliClient';

export function createShowStatsCommand(
  cliClient: CliClient,
  getWorkspaceRoot: () => string,
): () => Promise<void> {
  return async () => {
    const stats = await cliClient.workspaceStats(getWorkspaceRoot());
    const channel = vscode.window.createOutputChannel('FrilVault Stats');
    channel.clear();
    channel.appendLine(`files: ${stats.file_count}`);
    channel.appendLine(`notes: ${stats.total_notes}`);
    channel.appendLine(`existing files: ${stats.existing_files}`);
    channel.appendLine(`missing files: ${stats.missing_files}`);
    channel.appendLine(`line notes: ${stats.line_notes}`);
    channel.appendLine(`symbol notes: ${stats.symbol_notes}`);
    channel.show(true);
  };
}

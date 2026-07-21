import * as vscode from 'vscode';

import type { CliClient } from '../../core/cliClient';

export function createShowHealthCommand(
  cliClient: CliClient,
  getWorkspaceRoot: () => string,
): () => Promise<void> {
  return async () => {
    const workspaceRoot = getWorkspaceRoot();
    const health = await cliClient.workspaceHealth(workspaceRoot);
    const suggestions = await cliClient.repairSuggestions(workspaceRoot);
    const channel = vscode.window.createOutputChannel('FrilVault Health');

    channel.clear();

    if (health.missing_source_files.length === 0) {
      channel.appendLine('No missing source files.');
    } else {
      channel.appendLine('Missing source files:');
      for (const file of health.missing_source_files) {
        channel.appendLine(file);
      }
    }

    if (suggestions.length > 0) {
      channel.appendLine('');
      channel.appendLine('Repair suggestions:');
      for (const suggestion of suggestions) {
        channel.appendLine(
          `${suggestion.missing_file} -> ${
            suggestion.candidates.length > 0 ? suggestion.candidates.join(', ') : 'no candidates'
          }`,
        );
      }
    }

    channel.show(true);
  };
}

export function createApplyRepairsCommand(
  cliClient: CliClient,
  getWorkspaceRoot: () => string,
  refreshNotesPanel: () => void,
  refreshDecorations: () => Promise<void>,
): () => Promise<void> {
  return async () => {
    const repaired = await cliClient.applyRepairs(getWorkspaceRoot());
    refreshNotesPanel();
    await refreshDecorations();
    await vscode.window.showInformationMessage(`FrilVault repaired ${repaired} file(s).`);
  };
}

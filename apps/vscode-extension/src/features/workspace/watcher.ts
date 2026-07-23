import * as path from 'node:path';

import * as vscode from 'vscode';

import type { CliClient } from '../../core/cliClient';

const SYNC_DEBOUNCE_MS = 300;

export function isTrackedVaultPath(workspaceRoot: string, uri: vscode.Uri): boolean {
  const relative = path.relative(workspaceRoot, uri.fsPath);

  if (relative.startsWith('..')) {
    return false;
  }

  return (
    relative === `.vault${path.sep}notes` ||
    relative.startsWith(`.vault${path.sep}notes${path.sep}`) ||
    relative === `.vault${path.sep}images` ||
    relative.startsWith(`.vault${path.sep}images${path.sep}`)
  );
}

export function isTrackedSourcePath(workspaceRoot: string, uri: vscode.Uri): boolean {
  const relative = path.relative(workspaceRoot, uri.fsPath);

  if (relative.startsWith('..')) {
    return false;
  }

  if (relative.startsWith(`.vault${path.sep}`)) {
    return false;
  }

  return true;
}

export function registerWorkspaceWatcher(
  context: vscode.ExtensionContext,
  cliClient: CliClient,
  getWorkspaceRoot: () => string,
  refreshNotesPanel: () => void,
  refreshDecorations: () => Promise<void>,
): void {
  let debounceTimer: NodeJS.Timeout | undefined;

  const scheduleSync = () => {
    if (debounceTimer) {
      clearTimeout(debounceTimer);
    }

    debounceTimer = setTimeout(async () => {
      const workspaceRoot = getWorkspaceRoot();

      try {
        await cliClient.sync(workspaceRoot);
        refreshNotesPanel();
        await refreshDecorations();
      } catch (error) {
        const message =
          error instanceof Error ? error.message : 'Failed to sync workspace changes.';

        void vscode.window.showWarningMessage(message);
      }
    }, SYNC_DEBOUNCE_MS);
  };

  const notesWatcher = vscode.workspace.createFileSystemWatcher(
    new vscode.RelativePattern(getWorkspaceRoot(), '.vault/notes/**'),
  );

  notesWatcher.onDidCreate(scheduleSync);
  notesWatcher.onDidChange(scheduleSync);
  notesWatcher.onDidDelete(scheduleSync);

  const imagesWatcher = vscode.workspace.createFileSystemWatcher(
    new vscode.RelativePattern(getWorkspaceRoot(), '.vault/images/**'),
  );

  imagesWatcher.onDidCreate(scheduleSync);
  imagesWatcher.onDidChange(scheduleSync);
  imagesWatcher.onDidDelete(scheduleSync);

  const sourceWatcher = vscode.workspace.createFileSystemWatcher(
    new vscode.RelativePattern(getWorkspaceRoot(), '**/*'),
    false,
    true,
    false,
  );

  sourceWatcher.onDidCreate((uri) => {
    if (isTrackedSourcePath(getWorkspaceRoot(), uri)) {
      scheduleSync();
    }
  });

  sourceWatcher.onDidDelete((uri) => {
    if (isTrackedSourcePath(getWorkspaceRoot(), uri)) {
      scheduleSync();
    }
  });

  context.subscriptions.push(notesWatcher, imagesWatcher, sourceWatcher, {
    dispose: () => {
      if (debounceTimer) {
        clearTimeout(debounceTimer);
      }
    },
  });
}

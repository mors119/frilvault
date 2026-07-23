import * as path from 'node:path';

import * as vscode from 'vscode';

import type { CliClient } from '../../core/cliClient';
import { revealNote, tryGetWorkspaceRoot } from '../../utils/file';

export interface NoteUriHandlerDependencies {
  cliClient: CliClient;
  isEnabled: () => boolean;
}

export function registerNoteUriHandler(
  context: vscode.ExtensionContext,
  dependencies: NoteUriHandlerDependencies,
): void {
  const handler: vscode.UriHandler = {
    handleUri: async (uri: vscode.Uri) => {
      if (uri.scheme !== 'frilvault') {
        return;
      }

      if (!dependencies.isEnabled()) {
        void vscode.window.showInformationMessage(
          'FrilVault is disabled for this workspace. Turn it on from the FrilVault Notes view.',
        );
        return;
      }

      const noteUri = uri.toString(true);
      const workspaceRoot = resolveWorkspaceRoot(uri);

      if (!workspaceRoot) {
        void vscode.window.showErrorMessage(
          'FrilVault could not determine which workspace folder to open this note in.',
        );
        return;
      }

      try {
        const noteView = await dependencies.cliClient.resolveNoteUri(workspaceRoot, noteUri);
        await revealNote(noteView, workspaceRoot);
      } catch (error) {
        void vscode.window.showErrorMessage(formatResolveError(error));
      }
    },
  };

  context.subscriptions.push(vscode.window.registerUriHandler(handler));
}

function resolveWorkspaceRoot(uri: vscode.Uri): string | undefined {
  const configured = tryGetWorkspaceRoot();
  const queryWorkspace = uri.query
    .split('&')
    .map((segment) => segment.split('='))
    .find(([key]) => key === 'workspace')?.[1];

  if (!queryWorkspace) {
    return configured;
  }

  const decodedWorkspace = decodeURIComponent(queryWorkspace);
  const folders = vscode.workspace.workspaceFolders ?? [];

  const matchingFolder = folders.find(
    (folder) => path.resolve(folder.uri.fsPath) === path.resolve(decodedWorkspace),
  );

  if (matchingFolder) {
    return matchingFolder.uri.fsPath;
  }

  if (configured && path.resolve(configured) === path.resolve(decodedWorkspace)) {
    return configured;
  }

  return undefined;
}

function formatResolveError(error: unknown): string {
  const message = error instanceof Error ? error.message : 'Failed to resolve FrilVault note URI.';

  if (message.includes('unknown workspace')) {
    return 'FrilVault note URI targets a different workspace than the one currently open.';
  }

  if (message.includes('stale note')) {
    return 'FrilVault note URI points to a note whose source file is missing or out of date.';
  }

  if (message.includes('note not found')) {
    return 'FrilVault note URI points to a note that no longer exists.';
  }

  if (message.includes('unresolved anchor')) {
    return 'FrilVault note URI points to a symbol note that could not be resolved in the source file.';
  }

  if (message.includes('malformed note uri')) {
    return 'FrilVault note URI is malformed.';
  }

  return `FrilVault: ${message}`;
}

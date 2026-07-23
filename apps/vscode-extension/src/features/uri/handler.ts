import * as path from 'node:path';

import * as vscode from 'vscode';

import type { CliClient } from '../../core/cliClient';
import { revealNote, tryGetWorkspaceRoot } from '../../utils/file';
import { InvalidFrilVaultUriError, parseWorkspaceQuery } from './parse';

export interface NoteUriHandlerDependencies {
  cliClient: CliClient;
  isEnabled: () => boolean;
}

/**
 * Registers the `frilvault://` URI handler used by copied note links.
 *
 * The handler resolves the note through the CLI and reveals it in the editor.
 * It never reads or writes vault JSON directly from TypeScript.
 *
 * 복사된 note link가 사용하는 `frilvault://` URI handler를 등록합니다.
 *
 * handler는 CLI로 note를 해석한 뒤 editor에서 표시하며, TypeScript에서 vault
 * JSON을 직접 읽거나 쓰지 않습니다.
 */
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
      let workspaceRoot: string | undefined;

      try {
        workspaceRoot = resolveWorkspaceRoot(uri);
      } catch (error) {
        void vscode.window.showErrorMessage(formatResolveError(error));
        return;
      }

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
  const decodedWorkspace = parseWorkspaceQuery(uri);

  if (!decodedWorkspace) {
    return configured;
  }

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
  if (error instanceof InvalidFrilVaultUriError) {
    return 'FrilVault note URI is malformed.';
  }

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

import * as vscode from 'vscode';

import {
  isFrilVaultEnabled,
  setFrilVaultEnabled,
  syncEnabledContext,
} from './state';

export interface EnablementCommandDependencies {
  getWorkspaceRoot: () => string;
  workspaceState: vscode.Memento;
  refreshUi: () => Promise<void>;
  clearUi: () => void;
  showInformationMessage?: (message: string) => Thenable<string | undefined>;
}

export function createEnableCommand(
  dependencies: EnablementCommandDependencies,
): () => Promise<void> {
  return async () => {
    const workspaceRoot = dependencies.getWorkspaceRoot();

    if (isFrilVaultEnabled(dependencies.workspaceState, workspaceRoot)) {
      return;
    }

    await setFrilVaultEnabled(dependencies.workspaceState, workspaceRoot, true);
    await syncEnabledContext(true);
    await dependencies.refreshUi();

    const showInformationMessage =
      dependencies.showInformationMessage ?? vscode.window.showInformationMessage;

    await showInformationMessage('FrilVault enabled for this workspace.');
  };
}

export function createDisableCommand(
  dependencies: EnablementCommandDependencies,
): () => Promise<void> {
  return async () => {
    const workspaceRoot = dependencies.getWorkspaceRoot();

    if (!isFrilVaultEnabled(dependencies.workspaceState, workspaceRoot)) {
      return;
    }

    await setFrilVaultEnabled(dependencies.workspaceState, workspaceRoot, false);
    await syncEnabledContext(false);
    dependencies.clearUi();

    const showInformationMessage =
      dependencies.showInformationMessage ?? vscode.window.showInformationMessage;

    await showInformationMessage('FrilVault disabled for this workspace.');
  };
}

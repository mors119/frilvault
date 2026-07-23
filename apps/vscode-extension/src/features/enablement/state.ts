import * as vscode from 'vscode';

export const FRILVAULT_ENABLED_KEY = 'frilvaultEnabled';

export function isFrilVaultEnabled(
  workspaceState: vscode.Memento,
  workspaceRoot: string,
): boolean {
  const enabled =
    workspaceState.get<Record<string, boolean>>(FRILVAULT_ENABLED_KEY) ?? {};

  return enabled[workspaceRoot] !== false;
}

export async function setFrilVaultEnabled(
  workspaceState: vscode.Memento,
  workspaceRoot: string,
  enabled: boolean,
): Promise<void> {
  const current =
    workspaceState.get<Record<string, boolean>>(FRILVAULT_ENABLED_KEY) ?? {};

  await workspaceState.update(FRILVAULT_ENABLED_KEY, {
    ...current,
    [workspaceRoot]: enabled,
  });
}

export async function syncEnabledContext(enabled: boolean): Promise<void> {
  await vscode.commands.executeCommand('setContext', 'frilvault.enabled', enabled);
}

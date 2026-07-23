import * as vscode from 'vscode';

let hoverProviderRegistration: vscode.Disposable | undefined;

/**
 * Registers the FrilVault hover provider exactly once per extension host.
 *
 * VS Code merges hover content from every active provider and decoration
 * hover message. This helper is idempotent so repeated activation attempts
 * reuse the existing registration instead of stacking duplicate hovers.
 */
export function registerFrilVaultHoverProvider(
  context: vscode.ExtensionContext,
  provider: vscode.HoverProvider,
): vscode.Disposable {
  if (hoverProviderRegistration) {
    return hoverProviderRegistration;
  }

  hoverProviderRegistration = vscode.languages.registerHoverProvider(
    { scheme: 'file' },
    provider,
  );

  context.subscriptions.push({
    dispose: () => {
      disposeFrilVaultHoverProvider();
    },
  });

  return hoverProviderRegistration;
}

/** Disposes the hover provider registration and clears module state. */
export function disposeFrilVaultHoverProvider(): void {
  hoverProviderRegistration?.dispose();
  hoverProviderRegistration = undefined;
}

/** Returns whether the hover provider is currently registered. */
export function isFrilVaultHoverProviderRegistered(): boolean {
  return hoverProviderRegistration !== undefined;
}

/** Clears registration state between unit tests. */
export function resetHoverProviderRegistrationForTests(): void {
  disposeFrilVaultHoverProvider();
}

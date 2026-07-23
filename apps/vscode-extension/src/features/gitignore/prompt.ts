import * as vscode from 'vscode';

import type { CliClient } from '../../core/cliClient';

export const GITIGNORE_PROMPT_DISABLED_KEY = 'gitignorePromptDisabled';

export interface GitignorePromptDependencies {
  getWorkspaceRoot: () => string;
  cliClient: CliClient;
  workspaceState: vscode.Memento;
  showWarningMessage?: (
    message: string,
    options: { modal: boolean },
    ...items: string[]
  ) => Thenable<string | undefined>;
  showInformationMessage?: (message: string) => Thenable<string | undefined>;
}

function isPromptDisabled(workspaceState: vscode.Memento, workspaceRoot: string): boolean {
  const disabled =
    workspaceState.get<Record<string, boolean>>(GITIGNORE_PROMPT_DISABLED_KEY) ?? {};

  return disabled[workspaceRoot] === true;
}

async function disablePrompt(
  workspaceState: vscode.Memento,
  workspaceRoot: string,
): Promise<void> {
  const disabled =
    workspaceState.get<Record<string, boolean>>(GITIGNORE_PROMPT_DISABLED_KEY) ?? {};

  await workspaceState.update(GITIGNORE_PROMPT_DISABLED_KEY, {
    ...disabled,
    [workspaceRoot]: true,
  });
}

export async function maybePromptForGitignore(
  dependencies: GitignorePromptDependencies,
): Promise<void> {
  const workspaceRoot = dependencies.getWorkspaceRoot();
  const showWarningMessage =
    dependencies.showWarningMessage ?? vscode.window.showWarningMessage;
  const showInformationMessage =
    dependencies.showInformationMessage ?? vscode.window.showInformationMessage;

  if (isPromptDisabled(dependencies.workspaceState, workspaceRoot)) {
    return;
  }

  const status = await dependencies.cliClient.checkGitignore(workspaceRoot);

  if (status.ignored) {
    return;
  }

  const choice = await showWarningMessage(
    'FrilVault stores notes in `.vault/`, which is not listed in `.gitignore`. Add it to avoid committing personal notes?',
    { modal: true },
    'Add to .gitignore',
    'Skip',
    'Never Ask Again',
  );

  if (choice === 'Add to .gitignore') {
    await dependencies.cliClient.addGitignoreEntry(workspaceRoot);
    await showInformationMessage('Added `.vault/` to `.gitignore`.');
    return;
  }

  if (choice === 'Never Ask Again') {
    await disablePrompt(dependencies.workspaceState, workspaceRoot);
  }
}

import * as vscode from 'vscode';

import type { GitignorePromptDependencies } from '../gitignore/prompt';
import { maybePromptForGitignore } from '../gitignore/prompt';

export function formatOptionalPostSaveFailure(action: string, error: unknown): string {
  const detail = error instanceof Error ? error.message : String(error);
  return `FrilVault note saved, but ${action} failed: ${detail}`;
}

export async function runOptionalPostSaveTasks(
  dependencies: GitignorePromptDependencies,
  showWarningMessage: (
    message: string,
  ) => Thenable<string | undefined> = vscode.window.showWarningMessage,
): Promise<void> {
  try {
    await maybePromptForGitignore(dependencies);
  } catch (error) {
    await showWarningMessage(formatOptionalPostSaveFailure('the .gitignore prompt', error));
  }
}

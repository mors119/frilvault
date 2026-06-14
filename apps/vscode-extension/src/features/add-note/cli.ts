import { execFile } from 'node:child_process';
import { promisify } from 'node:util';

import * as vscode from 'vscode';

const execFileAsync = promisify(execFile);

export interface AddNoteCliInput {
  cliPath: string;
  workspaceRoot: string;
  relativeFilePath: string;
  line: number;
  column: number;
  content: string;
}

export async function executeAddNoteCli(input: AddNoteCliInput): Promise<void> {
  const args = [
    'add',
    '--file',
    input.relativeFilePath,
    '--line',
    String(input.line),
    '--column',
    String(input.column),
    '--content',
    input.content,
  ];

  try {
    await execFileAsync(input.cliPath, args, {
      cwd: input.workspaceRoot,
    });
  } catch (error) {
    const message =
      error instanceof Error ? error.message : 'Failed to execute FrilVault CLI.';

    throw new Error(message);
  }
}

export function getConfiguredCliPath(): string {
  return vscode.workspace
    .getConfiguration('frilvault')
    .get<string>('cliPath', 'flvt')
    .trim();
}

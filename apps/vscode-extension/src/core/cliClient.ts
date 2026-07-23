import { execFile } from 'node:child_process';
import { promisify } from 'node:util';

import * as vscode from 'vscode';

import type { NoteView, RepairSuggestion, SyncResult, WorkspaceHealth, WorkspaceStats } from '../types';
import { parseJson } from '../utils/parser';

const execFileAsync = promisify(execFile);

export function getConfiguredCliPath(): string {
  return vscode.workspace
    .getConfiguration('frilvault')
    .get<string>('cliPath', 'flvt')
    .trim();
}

export interface AddLineNoteInput {
  workspaceRoot: string;
  sourceFile: string;
  line: number;
  column: number;
  content: string;
}

export interface SearchNotesInput {
  workspaceRoot: string;
  keyword?: string;
  sourceFile?: string;
}

export class CliClient {
  public constructor(private readonly getCliPath = getConfiguredCliPath) {}

  public async addLineNote(input: AddLineNoteInput): Promise<void> {
    await this.execInWorkspace(input.workspaceRoot, [
      'add',
      '--file',
      input.sourceFile,
      '--line',
      String(input.line),
      '--column',
      String(input.column),
      '--content',
      input.content,
    ]);
  }

  public async listNotes(workspaceRoot: string, sourceFile: string): Promise<NoteView[]> {
    const stdout = await this.execInWorkspace(workspaceRoot, [
      'list',
      '--file',
      sourceFile,
      '--format',
      'json',
    ]);

    return parseJson<NoteView[]>(stdout);
  }

  public async searchNotes(input: SearchNotesInput): Promise<NoteView[]> {
    const args = ['search'];

    if (input.keyword) {
      args.push(input.keyword);
    }

    if (input.sourceFile) {
      args.push('--file', input.sourceFile);
    }

    args.push('--format', 'json');

    const stdout = await this.execInWorkspace(input.workspaceRoot, args);

    return parseJson<NoteView[]>(stdout);
  }

  public async updateNote(
    workspaceRoot: string,
    sourceFile: string,
    noteId: string,
    content: string,
  ): Promise<void> {
    await this.execInWorkspace(workspaceRoot, [
      'update',
      '--file',
      sourceFile,
      '--id',
      noteId,
      '--content',
      content,
    ]);
  }

  public async deleteNote(
    workspaceRoot: string,
    sourceFile: string,
    noteId: string,
  ): Promise<void> {
    await this.execInWorkspace(workspaceRoot, [
      'delete',
      '--file',
      sourceFile,
      '--id',
      noteId,
    ]);
  }

  public async stats(workspaceRoot: string): Promise<string> {
    return this.execInWorkspace(workspaceRoot, ['stats']);
  }

  public async health(workspaceRoot: string): Promise<string> {
    return this.execInWorkspace(workspaceRoot, ['health']);
  }

  public async repair(workspaceRoot: string, apply = false): Promise<string> {
    return this.execInWorkspace(workspaceRoot, apply ? ['repair', '--apply'] : ['repair']);
  }

  public async workspaceStats(workspaceRoot: string): Promise<WorkspaceStats> {
    const stdout = await this.execInWorkspace(workspaceRoot, ['stats', '--format', 'json']);
    return parseJson<WorkspaceStats>(stdout);
  }

  public async workspaceHealth(workspaceRoot: string): Promise<WorkspaceHealth> {
    const stdout = await this.execInWorkspace(workspaceRoot, ['health', '--format', 'json']);
    return parseJson<WorkspaceHealth>(stdout);
  }

  public async repairSuggestions(workspaceRoot: string): Promise<RepairSuggestion[]> {
    const stdout = await this.execInWorkspace(workspaceRoot, ['repair', '--format', 'json']);
    return parseJson<RepairSuggestion[]>(stdout);
  }

  public async applyRepairs(workspaceRoot: string): Promise<number> {
    const stdout = await this.execInWorkspace(workspaceRoot, [
      'repair',
      '--apply',
      '--format',
      'json',
    ]);
    return parseJson<number>(stdout);
  }

  public async sync(workspaceRoot: string): Promise<SyncResult> {
    const stdout = await this.execInWorkspace(workspaceRoot, ['sync', '--format', 'json']);
    return parseJson<SyncResult>(stdout);
  }

  public async checkGitignore(workspaceRoot: string): Promise<{ ignored: boolean }> {
    const stdout = await this.execInWorkspace(workspaceRoot, [
      'gitignore',
      'check',
      '--format',
      'json',
    ]);

    return parseJson<{ ignored: boolean }>(stdout);
  }

  public async addGitignoreEntry(workspaceRoot: string): Promise<void> {
    await this.execInWorkspace(workspaceRoot, ['gitignore', 'add']);
  }

  public async resolveNoteUri(workspaceRoot: string, uri: string): Promise<NoteView> {
    const stdout = await this.execInWorkspace(workspaceRoot, [
      'resolve-uri',
      '--uri',
      uri,
      '--format',
      'json',
    ]);

    return parseJson<NoteView>(stdout);
  }

  private async execInWorkspace(workspaceRoot: string, args: string[]): Promise<string> {
    try {
      const { stdout } = await execFileAsync(this.getCliPath(), args, {
        cwd: workspaceRoot,
      });

      return stdout.trim();
    } catch (error) {
      const message =
        error instanceof Error ? error.message : 'Failed to execute FrilVault CLI.';

      throw new Error(message);
    }
  }
}

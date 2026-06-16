import * as path from 'node:path';
import * as vscode from 'vscode';

import type { MutationResult, RepairSuggestion, NoteView, WorkspaceHealth, WorkspaceStats } from '../types';
import { parseJson } from '../utils/parser';

interface NativeBinding {
  addLineNote(
    workspaceRoot: string,
    sourceFile: string,
    line: number,
    column: number,
    content: string,
  ): string;
  listNotes(workspaceRoot: string, sourceFile: string): string;
  updateNote(workspaceRoot: string, sourceFile: string, noteId: string, content: string): string;
  deleteNote(workspaceRoot: string, sourceFile: string, noteId: string): void;
  searchNotes(workspaceRoot: string, keyword: string): string;
  workspaceStats(workspaceRoot: string): string;
  workspaceHealth(workspaceRoot: string): string;
  repairSuggestions(workspaceRoot: string): string;
  applyRepairs(workspaceRoot: string): number;
}

export class NodeBridge {
  private readonly binding: NativeBinding;

  public constructor(context: vscode.ExtensionContext) {
    this.binding = require(path.join(context.extensionPath, 'dist', 'frilvault.node')) as NativeBinding;
  }

  public addLineNote(
    workspaceRoot: string,
    sourceFile: string,
    line: number,
    column: number,
    content: string,
  ): MutationResult {
    return parseJson<MutationResult>(
      this.binding.addLineNote(workspaceRoot, sourceFile, line, column, content),
    );
  }

  public listNotes(workspaceRoot: string, sourceFile: string): NoteView[] {
    return parseJson<NoteView[]>(this.binding.listNotes(workspaceRoot, sourceFile));
  }

  public updateNote(
    workspaceRoot: string,
    sourceFile: string,
    noteId: string,
    content: string,
  ): MutationResult {
    return parseJson<MutationResult>(
      this.binding.updateNote(workspaceRoot, sourceFile, noteId, content),
    );
  }

  public deleteNote(workspaceRoot: string, sourceFile: string, noteId: string): void {
    this.binding.deleteNote(workspaceRoot, sourceFile, noteId);
  }

  public searchNotes(workspaceRoot: string, keyword: string): NoteView[] {
    return parseJson<NoteView[]>(this.binding.searchNotes(workspaceRoot, keyword));
  }

  public workspaceStats(workspaceRoot: string): WorkspaceStats {
    return parseJson<WorkspaceStats>(this.binding.workspaceStats(workspaceRoot));
  }

  public workspaceHealth(workspaceRoot: string): WorkspaceHealth {
    return parseJson<WorkspaceHealth>(this.binding.workspaceHealth(workspaceRoot));
  }

  public repairSuggestions(workspaceRoot: string): RepairSuggestion[] {
    return parseJson<RepairSuggestion[]>(this.binding.repairSuggestions(workspaceRoot));
  }

  public applyRepairs(workspaceRoot: string): number {
    return this.binding.applyRepairs(workspaceRoot);
  }
}

import * as vscode from 'vscode';

import type { CliClient } from '../../core/cliClient';
import type { NoteView } from '../../types';
import { tryGetRelativeFilePath, tryGetWorkspaceRoot } from '../../utils/file';

export interface CurrentFileNotesSnapshot {
  workspaceRoot: string | undefined;
  sourceFile: string | undefined;
  editorDocumentUri: string | undefined;
  notes: NoteView[];
  error: string | undefined;
  loading: boolean;
}

const EMPTY_SNAPSHOT: CurrentFileNotesSnapshot = {
  workspaceRoot: undefined,
  sourceFile: undefined,
  editorDocumentUri: undefined,
  notes: [],
  error: undefined,
  loading: false,
};

/**
 * Cached note list for the active editor file.
 *
 * The store prevents stale async list responses from overwriting newer editor
 * state by tracking a monotonically increasing load generation.
 *
 * 활성 편집기 파일의 note list cache입니다.
 *
 * load generation을 증가시키며 stale async list 응답이 더 새로운 editor
 * state를 덮어쓰지 않도록 막습니다.
 */
export class CurrentFileNotesStore implements vscode.Disposable {
  private snapshot: CurrentFileNotesSnapshot = { ...EMPTY_SNAPSHOT };

  private readonly onDidChangeEmitter = new vscode.EventEmitter<void>();

  public readonly onDidChange = this.onDidChangeEmitter.event;

  private loadGeneration = 0;

  public constructor(
    private readonly cliClient: CliClient,
    private readonly isEnabled: () => boolean,
    private readonly getWorkspaceRoot: () => string | undefined = tryGetWorkspaceRoot,
  ) {}

  public getSnapshot(): CurrentFileNotesSnapshot {
    return this.snapshot;
  }

  public clear(): void {
    this.loadGeneration += 1;
    this.snapshot = { ...EMPTY_SNAPSHOT };
    this.onDidChangeEmitter.fire();
  }

  public async syncActiveEditor(editor = vscode.window.activeTextEditor): Promise<void> {
    if (!this.isEnabled()) {
      this.clear();
      return;
    }

    if (!editor || editor.document.uri.scheme !== 'file') {
      this.clear();
      return;
    }

    const workspaceRoot = this.getWorkspaceRoot();
    if (!workspaceRoot) {
      this.setSnapshot({
        ...EMPTY_SNAPSHOT,
        editorDocumentUri: editor.document.uri.toString(),
        error: 'FrilVault requires an open workspace folder.',
      });
      return;
    }

    const sourceFile = tryGetRelativeFilePath(workspaceRoot, editor.document.uri.fsPath);
    if (!sourceFile) {
      this.setSnapshot({
        workspaceRoot,
        sourceFile: undefined,
        editorDocumentUri: editor.document.uri.toString(),
        notes: [],
        error: undefined,
        loading: false,
      });
      return;
    }

    const generation = ++this.loadGeneration;

    this.setSnapshot({
      workspaceRoot,
      sourceFile,
      editorDocumentUri: editor.document.uri.toString(),
      notes: this.snapshot.editorDocumentUri === editor.document.uri.toString()
        ? this.snapshot.notes
        : [],
      error: undefined,
      loading: true,
    });

    try {
      const notes = await this.cliClient.listNotes(workspaceRoot, sourceFile);

      // Ignore late responses from a previous editor or disabled state.
      // 이전 editor 또는 disabled state에서 온 늦은 응답은 무시합니다.
      if (generation !== this.loadGeneration) {
        return;
      }

      this.setSnapshot({
        workspaceRoot,
        sourceFile,
        editorDocumentUri: editor.document.uri.toString(),
        notes,
        error: undefined,
        loading: false,
      });
    } catch (error) {
      if (generation !== this.loadGeneration) {
        return;
      }

      const message =
        error instanceof Error ? error.message : 'Failed to load notes for the current file.';

      this.setSnapshot({
        workspaceRoot,
        sourceFile,
        editorDocumentUri: editor.document.uri.toString(),
        notes: [],
        error: message,
        loading: false,
      });
    }
  }

  public async invalidateAfterMutation(): Promise<void> {
    await this.syncActiveEditor(vscode.window.activeTextEditor);
  }

  public notesForDocument(document: vscode.TextDocument): NoteView[] {
    const snapshot = this.snapshot;

    if (snapshot.loading || snapshot.editorDocumentUri !== document.uri.toString()) {
      return [];
    }

    return snapshot.notes;
  }

  public dispose(): void {
    this.onDidChangeEmitter.dispose();
  }

  private setSnapshot(snapshot: CurrentFileNotesSnapshot): void {
    this.snapshot = snapshot;
    this.onDidChangeEmitter.fire();
  }
}

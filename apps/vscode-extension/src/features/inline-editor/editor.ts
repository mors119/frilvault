import * as vscode from 'vscode';

import type { CliClient } from '../../core/cliClient';
import { getInlineNotesDebounceMs } from '../presentation/inlinePreviewSettings';
import type { NoteView } from '../../types';
import {
  getActiveEditorOrThrow,
  getRelativeFilePath,
  getWorkspaceRoot,
} from '../../utils/file';
import {
  findSymbolAtPosition,
  mapDocumentSymbolKind,
  readSymbolSignature,
} from '../../utils/symbols';
import {
  AutoSaveStatus,
  DebouncedAutoSave,
  draftFingerprint,
} from './autoSave';
import {
  applyFormInput,
  createEditDraft,
  revisionFromDraft,
  validateInlineNoteForm,
  type InlineNoteDraft,
} from './draft';
import { InlineNotePanel, type InlineNotePanelMessage } from './panel';
import { InlineNoteEditorService } from './service';

export interface InlineNoteEditorDependencies {
  cliClient: CliClient;
  getWorkspaceRoot?: () => string;
  refreshNoteState: () => Promise<void>;
  runOptionalPostSaveTasks?: () => Promise<void>;
  showErrorMessage?: (message: string) => Thenable<string | undefined>;
  showInformationMessage?: (message: string) => Thenable<string | undefined>;
  showWarningMessage?: (message: string) => Thenable<string | undefined>;
}

/**
 * Webview-based inline note editor with debounced auto-save.
 *
 * webview 기반 inline note editor이며 debounced auto-save를 사용합니다.
 */
export class InlineNoteEditor {
  private readonly panel = new InlineNotePanel();
  private readonly service: InlineNoteEditorService;
  private readonly autoSave: DebouncedAutoSave;
  private draft: InlineNoteDraft | undefined;
  private context: vscode.ExtensionContext | undefined;
  private saveStatus: AutoSaveStatus = 'saved';
  private conflictDraft: InlineNoteDraft | undefined;

  public constructor(private readonly dependencies: InlineNoteEditorDependencies) {
    this.service = new InlineNoteEditorService(dependencies.cliClient);
    this.autoSave = new DebouncedAutoSave(
      getInlineNotesDebounceMs(),
      (status) => this.handleSaveStatus(status),
      async (generation) => {
        await this.persistDraft(generation);
      },
    );
  }

  public register(context: vscode.ExtensionContext): void {
    this.context = context;
  }

  public async openCreateHere(): Promise<void> {
    const editor = getActiveEditorOrThrow();
    const workspaceRoot = this.workspaceRoot();
    const sourceFile = getRelativeFilePath(workspaceRoot, editor.document.uri.fsPath);
    const position = editor.selection.active;
    const line = position.line + 1;
    const column = position.character + 1;

    const symbol = await findSymbolAtPosition(editor.document, position);
    const draft = this.service.buildCreateDraftForEditor({
      workspaceRoot,
      sourceFile,
      line,
      column,
      symbol: symbol
        ? {
            name: symbol.name,
            kind: mapDocumentSymbolKind(symbol.kind),
            signature: readSymbolSignature(editor.document, symbol),
            lineHint: symbol.range.start.line + 1,
          }
        : undefined,
    });

    this.openDraft(draft);
  }

  public openEdit(noteView: NoteView): void {
    this.openDraft(createEditDraft(noteView, this.workspaceRoot()));
  }

  public openEditById(noteId: string, sourceFile: string, noteView?: NoteView): void {
    if (noteView) {
      this.openEdit(noteView);
      return;
    }

    this.openEdit({
      source_file: sourceFile,
      note: {
        id: noteId,
        content: '',
        anchor: { type: 'Line', line: 1, column: 1 },
      },
    });
  }

  private openDraft(draft: InlineNoteDraft): void {
    if (!this.context) {
      throw new Error('Inline note editor is not registered.');
    }

    this.draft = draft;
    this.conflictDraft = undefined;
    this.autoSave.setPersistedFingerprint(draftFingerprint(draft.content, draft.tagsText));
    this.handleSaveStatus('saved');

    this.panel.open(
      this.context,
      draft,
      async (message) => {
        await this.handlePanelMessage(message);
      },
      async () => {
        await this.handlePanelClose();
      },
    );
  }

  private async handlePanelMessage(message: InlineNotePanelMessage): Promise<void> {
    if (!this.draft) {
      return;
    }

    switch (message.type) {
      case 'change':
        await this.handleChange(message.content, message.tagsText);
        break;
      case 'close':
        await this.handlePanelClose(true);
        break;
      case 'delete':
        await this.handleDelete();
        break;
      case 'retry':
        await this.autoSave.flush();
        break;
      case 'keepLocal':
        await this.handleKeepLocalVersion();
        break;
      case 'loadExternal':
        await this.handleLoadExternalVersion();
        break;
    }
  }

  private async handleChange(content: string, tagsText: string): Promise<void> {
    if (!this.draft || this.saveStatus === 'conflict') {
      return;
    }

    this.draft = applyFormInput(this.draft, { content, tagsText });
    this.autoSave.schedule(draftFingerprint(content, tagsText));
  }

  private async handlePanelClose(forceClose = false): Promise<void> {
    if (!this.draft) {
      this.panel.close();
      return;
    }

    await this.autoSave.flush();

    if (this.saveStatus === 'failed' && !forceClose) {
      this.panel.updateDraft(this.draft, {
        errorMessage: 'Save failed. Retry or keep editing before closing.',
        status: this.saveStatus,
      });
      return;
    }

    this.autoSave.cancel();
    this.panel.close();
    this.draft = undefined;
    this.conflictDraft = undefined;
  }

  private async handleDelete(): Promise<void> {
    const noteId = this.draft?.noteId;

    if (!this.draft || !noteId) {
      return;
    }

    const draft = this.draft;

    const showWarningMessage =
      this.dependencies.showWarningMessage ?? vscode.window.showWarningMessage;
    const confirmed = await showWarningMessage(
      'Delete this FrilVault note?',
      { modal: true },
      'Delete',
    );

    if (confirmed !== 'Delete') {
      return;
    }

    try {
      await this.dependencies.cliClient.deleteNote(
        draft.workspaceRoot,
        draft.sourceFile,
        noteId,
      );
      await this.dependencies.refreshNoteState();
      this.autoSave.cancel();
      this.panel.close();
      this.draft = undefined;
    } catch (error) {
      this.panel.updateDraft(draft, {
        errorMessage: formatError(error, 'Failed to delete note.'),
        status: this.saveStatus,
      });
    }
  }

  private async persistDraft(generation: number): Promise<void> {
    if (!this.draft || !this.autoSave.isLatestGeneration(generation)) {
      return;
    }

    const validationError = validateInlineNoteForm({
      content: this.draft.content,
      tagsText: this.draft.tagsText,
    });

    if (validationError) {
      this.panel.updateDraft(this.draft, {
        errorMessage: validationError,
        status: 'failed',
      });
      throw new Error(validationError);
    }

    try {
      const undoSnapshot = this.draft.undoSnapshot ?? revisionFromDraft(this.draft);
      const saved = await this.service.saveDraft(this.draft);

      if (!this.autoSave.isLatestGeneration(generation)) {
        return;
      }

      const persisted = this.service.applySavedRevision(this.draft, saved, undoSnapshot);
      this.draft = persisted;
      this.autoSave.setPersistedFingerprint(
        draftFingerprint(persisted.content, persisted.tagsText),
      );
      this.panel.updateDraft(persisted, { status: 'saved', canDelete: true });

      try {
        await this.dependencies.refreshNoteState();
      } catch (error) {
        await this.reportOptionalFailure('refreshing note views', error);
      }

      void this.dependencies.runOptionalPostSaveTasks?.().catch(async (error) => {
        await this.reportOptionalFailure('running post-save tasks', error);
      });
    } catch (error) {
      if (!this.autoSave.isLatestGeneration(generation)) {
        return;
      }

      if (isConcurrentModificationError(error)) {
        this.conflictDraft = this.draft;
        this.handleSaveStatus('conflict');
        this.panel.updateDraft(this.draft, {
          errorMessage: 'This note was changed elsewhere.',
          status: 'conflict',
        });
        throw error;
      }

      this.panel.updateDraft(this.draft, {
        errorMessage: formatError(error, 'Failed to save note.'),
        status: 'failed',
      });
      throw error;
    }
  }

  private async handleKeepLocalVersion(): Promise<void> {
    if (!this.conflictDraft) {
      return;
    }

    this.draft = this.conflictDraft;
    this.conflictDraft = undefined;
    this.handleSaveStatus('editing');
    await this.autoSave.flush();
  }

  private async handleLoadExternalVersion(): Promise<void> {
    if (!this.draft?.noteId) {
      return;
    }

    try {
      const notes = await this.dependencies.cliClient.listNotes(
        this.draft.workspaceRoot,
        this.draft.sourceFile,
      );
      const latest = notes.find((note) => note.note.id === this.draft?.noteId);

      if (!latest) {
        throw new Error('Note no longer exists.');
      }

      this.draft = createEditDraft(latest, this.draft.workspaceRoot);
      this.conflictDraft = undefined;
      this.autoSave.setPersistedFingerprint(
        draftFingerprint(this.draft.content, this.draft.tagsText),
      );
      this.handleSaveStatus('saved');
      this.panel.updateDraft(this.draft, { status: 'saved' });
    } catch (error) {
      this.panel.updateDraft(this.draft, {
        errorMessage: formatError(error, 'Failed to load the external version.'),
        status: 'conflict',
      });
    }
  }

  private handleSaveStatus(status: AutoSaveStatus): void {
    this.saveStatus = status;

    if (this.draft) {
      this.panel.updateDraft(this.draft, { status });
    }
  }

  private workspaceRoot(): string {
    return this.dependencies.getWorkspaceRoot?.() ?? getWorkspaceRoot();
  }

  private async reportOptionalFailure(action: string, error: unknown): Promise<void> {
    const showWarningMessage =
      this.dependencies.showWarningMessage ?? vscode.window.showWarningMessage;
    const detail = error instanceof Error ? error.message : 'Unknown error';

    await showWarningMessage(`FrilVault note saved, but ${action} failed: ${detail}`);
  }
}

function isConcurrentModificationError(error: unknown): boolean {
  const message = error instanceof Error ? error.message.toLowerCase() : '';
  return message.includes('concurrent modification');
}

function formatError(error: unknown, fallback: string): string {
  return error instanceof Error ? error.message : fallback;
}

export function createInlineNoteEditor(
  dependencies: InlineNoteEditorDependencies,
): InlineNoteEditor {
  return new InlineNoteEditor(dependencies);
}

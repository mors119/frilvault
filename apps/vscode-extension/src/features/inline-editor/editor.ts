import * as vscode from 'vscode';

import type { CliClient } from '../../core/cliClient';
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
  invalidateViews: () => Promise<void>;
  showErrorMessage?: (message: string) => Thenable<string | undefined>;
  showInformationMessage?: (message: string) => Thenable<string | undefined>;
  showWarningMessage?: (
    message: string,
    options: vscode.MessageOptions,
    ...items: string[]
  ) => Thenable<string | undefined>;
}

export class InlineNoteEditor {
  private readonly panel = new InlineNotePanel();
  private readonly service: InlineNoteEditorService;
  private draft: InlineNoteDraft | undefined;
  private lastSavedSnapshot: InlineNoteDraft['undoSnapshot'];
  private context: vscode.ExtensionContext | undefined;

  public constructor(private readonly dependencies: InlineNoteEditorDependencies) {
    this.service = new InlineNoteEditorService(dependencies.cliClient);
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
    const draft = createEditDraft(noteView, this.workspaceRoot());
    this.openDraft(draft);
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
    this.lastSavedSnapshot = draft.undoSnapshot;

    this.panel.open(this.context, draft, async (message) => {
      await this.handlePanelMessage(message);
    });
  }

  private async handlePanelMessage(message: InlineNotePanelMessage): Promise<void> {
    if (!this.draft) {
      return;
    }

    if (message.type === 'cancel') {
      this.panel.close();
      this.draft = undefined;
      return;
    }

    if (message.type === 'undo') {
      await this.undoLastSave();
      return;
    }

    if (message.type !== 'save') {
      return;
    }

    const nextDraft = applyFormInput(this.draft, {
      content: message.content ?? this.draft.content,
      tagsText: message.tagsText ?? this.draft.tagsText,
    });

    const validationError = validateInlineNoteForm({
      content: nextDraft.content,
      tagsText: nextDraft.tagsText,
    });

    if (validationError) {
      this.draft = nextDraft;
      this.panel.updateDraft(nextDraft, { errorMessage: validationError });
      return;
    }

    try {
      const undoSnapshot = this.draft.undoSnapshot ?? revisionFromDraft(this.draft);
      const saved = await this.service.saveDraft(nextDraft);
      const persisted = this.service.applySavedRevision(nextDraft, saved, undoSnapshot);
      this.draft = persisted;
      this.lastSavedSnapshot = undoSnapshot;
      await this.dependencies.invalidateViews();
      this.panel.updateDraft(persisted);
      await this.showInfo('FrilVault note saved. Undo remains available in the editor.');
    } catch (error) {
      if (isConcurrentModificationError(error)) {
        await this.handleConcurrentModification(nextDraft);
        return;
      }

      this.draft = nextDraft;
      this.panel.updateDraft(nextDraft, {
        errorMessage: formatError(error, 'Failed to save note.'),
      });
    }
  }

  private async undoLastSave(): Promise<void> {
    if (!this.draft?.noteId || !this.lastSavedSnapshot) {
      return;
    }

    try {
      const restored = await this.service.undoRevision(this.draft, this.lastSavedSnapshot);
      const nextDraft = this.service.applySavedRevision(
        this.draft,
        restored,
        revisionFromDraft(this.draft),
      );
      this.draft = {
        ...nextDraft,
        undoSnapshot: this.lastSavedSnapshot,
      };
      await this.dependencies.invalidateViews();
      this.panel.updateDraft(this.draft);
      await this.showInfo('FrilVault note restored to the previous revision.');
    } catch (error) {
      if (isConcurrentModificationError(error)) {
        await this.handleConcurrentModification(this.draft);
        return;
      }

      this.panel.updateDraft(this.draft, {
        errorMessage: formatError(error, 'Failed to undo the last save.'),
      });
    }
  }

  private async handleConcurrentModification(draft: InlineNoteDraft): Promise<void> {
    const showWarningMessage =
      this.dependencies.showWarningMessage ?? vscode.window.showWarningMessage;
    const choice = await showWarningMessage(
      'This note was changed elsewhere. Reload the latest version or keep editing your draft.',
      { modal: true },
      'Reload Latest',
      'Keep Draft',
    );

    if (choice !== 'Reload Latest' || !draft.noteId) {
      this.draft = draft;
      this.panel.updateDraft(draft, {
        errorMessage: 'Save blocked because a newer revision exists.',
      });
      return;
    }

    try {
      const notes = await this.dependencies.cliClient.listNotes(
        draft.workspaceRoot,
        draft.sourceFile,
      );
      const latest = notes.find((note) => note.note.id === draft.noteId);

      if (!latest) {
        throw new Error('Note no longer exists.');
      }

      const reloaded = createEditDraft(latest, draft.workspaceRoot);
      this.draft = {
        ...reloaded,
        content: draft.content,
        tagsText: draft.tagsText,
      };
      this.panel.updateDraft(this.draft, {
        errorMessage: 'Loaded the latest saved revision. Review your draft before saving again.',
      });
    } catch (error) {
      this.draft = draft;
      this.panel.updateDraft(draft, {
        errorMessage: formatError(error, 'Failed to reload the latest note revision.'),
      });
    }
  }

  private workspaceRoot(): string {
    return this.dependencies.getWorkspaceRoot?.() ?? getWorkspaceRoot();
  }

  private async showInfo(message: string): Promise<void> {
    const showInformationMessage =
      this.dependencies.showInformationMessage ?? vscode.window.showInformationMessage;
    await showInformationMessage(message);
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

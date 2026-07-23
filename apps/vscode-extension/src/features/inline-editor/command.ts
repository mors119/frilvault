import * as vscode from 'vscode';

import type { NoteView } from '../../types';
import type { InlineNoteEditor } from './editor';

export function createCreateNoteHereCommand(
  editor: InlineNoteEditor,
): () => Promise<void> {
  return async () => {
    await editor.openCreateHere();
  };
}

export function createEditNoteCommand(
  editor: InlineNoteEditor,
): (noteId?: string, sourceFile?: string, noteView?: NoteView) => Promise<void> {
  return async (noteId?: string, sourceFile?: string, noteView?: NoteView) => {
    if (noteView) {
      editor.openEdit(noteView);
      return;
    }

    if (!noteId || !sourceFile) {
      throw new Error('Edit note requires a note id and source file.');
    }

    editor.openEditById(noteId, sourceFile);
  };
}

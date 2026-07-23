import type { NoteView } from '../../types';

/** Deduplicates notes by stable note ID while preserving first occurrence order. */
export function deduplicateNotesById(notes: NoteView[]): NoteView[] {
  const unique = new Map<string, NoteView>();

  for (const note of notes) {
    if (!unique.has(note.note.id)) {
      unique.set(note.note.id, note);
    }
  }

  return [...unique.values()];
}

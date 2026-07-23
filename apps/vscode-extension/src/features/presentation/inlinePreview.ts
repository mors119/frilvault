import type { NoteView } from '../../types';

export const INLINE_NOTE_PREFIX = 'Note: ';

/** Normalizes note content for compact inline display. */
export function normalizeNoteForInlineDisplay(content: string): string {
  return content
    .replace(/```[\s\S]*?```/g, ' ')
    .replace(/`([^`]+)`/g, '$1')
    .replace(/^#{1,6}\s+/gm, '')
    .replace(/\*\*([^*]+)\*\*/g, '$1')
    .replace(/\*([^*]+)\*/g, '$1')
    .replace(/\s+/g, ' ')
    .trim();
}

/** Creates a truncated one-line inline preview with a Unicode ellipsis. */
export function createInlinePreview(content: string, maxLength: number): string {
  const normalized = normalizeNoteForInlineDisplay(content);

  if (normalized.length <= maxLength) {
    return normalized;
  }

  return `${normalized.slice(0, maxLength).trimEnd()}…`;
}

/** Formats one or more notes for inline decoration preview text. */
export function formatInlineNotesPreview(notes: NoteView[], maxLength: number): string {
  if (notes.length === 0) {
    return '';
  }

  const primary = createInlinePreview(notes[0].note.content, maxLength);

  if (notes.length === 1) {
    return `${INLINE_NOTE_PREFIX}${primary}`;
  }

  return `${INLINE_NOTE_PREFIX}${primary} (+${notes.length - 1})`;
}

/** @deprecated Use {@link createInlinePreview} instead. */
export function truncateInlineContent(content: string, maxLength: number): string {
  return createInlinePreview(content, maxLength);
}

/** @deprecated Use {@link normalizeNoteForInlineDisplay} instead. */
export function normalizeInlineContent(content: string): string {
  return normalizeNoteForInlineDisplay(content);
}

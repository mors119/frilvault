import type { NoteView } from '../../types';
import { formatAnchorLabel, toEditorNoteView } from './editorNoteView';
import { toHoverNotePresentation } from './noteHover';

/** Plain note content for clipboard copy commands. */
export function buildNoteContentForClipboard(note: NoteView): string {
  return note.note.content;
}

/** Markdown note payload for clipboard copy commands (no hover action labels). */
export function buildNoteMarkdownForClipboard(note: NoteView, workspaceRoot: string): string {
  const presentation = toHoverNotePresentation(toEditorNoteView(note, workspaceRoot));
  const lines = ['# FrilVault', '', presentation.content, '', formatAnchorLabel(presentation.anchor)];

  if (presentation.tags.length > 0) {
    lines.push(`Tags: ${presentation.tags.join(', ')}`);
  }

  if (presentation.updatedAt) {
    lines.push(`Updated: ${formatUpdatedAt(presentation.updatedAt)}`);
  }

  return lines.join('\n');
}

function formatUpdatedAt(value: string): string {
  const parsed = Date.parse(value);

  if (Number.isNaN(parsed)) {
    return value;
  }

  return new Date(parsed).toLocaleDateString();
}

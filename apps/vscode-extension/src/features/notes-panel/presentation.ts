import type { NoteView } from '../../types';

export interface SymbolNoteGroup {
  name: string;
  notes: NoteView[];
}

export interface NotesAnchorGroups {
  symbolGroups: SymbolNoteGroup[];
  lineNotes: NoteView[];
  unresolvedNotes: NoteView[];
}

/** Groups current-file notes by symbol name, line anchor, and unresolved symbol anchors. */
export function groupNotesByAnchor(notes: NoteView[]): NotesAnchorGroups {
  const lineNotes = notes
    .filter((note) => note.note.anchor.type === 'Line')
    .sort((left, right) => (left.note.anchor.line ?? 0) - (right.note.anchor.line ?? 0));

  const symbolNotes = notes.filter((note) => note.note.anchor.type === 'Symbol');
  const unresolvedNotes = symbolNotes
    .filter((note) => !note.resolved)
    .sort((left, right) =>
      (left.note.anchor.name ?? '').localeCompare(right.note.anchor.name ?? ''),
    );

  const resolvedByName = new Map<string, NoteView[]>();

  for (const note of symbolNotes.filter((entry) => entry.resolved)) {
    const name = note.note.anchor.name ?? 'Symbol';
    const group = resolvedByName.get(name) ?? [];
    group.push(note);
    resolvedByName.set(name, group);
  }

  const symbolGroups = [...resolvedByName.entries()]
    .sort(([left], [right]) => left.localeCompare(right))
    .map(([name, groupNotes]) => ({ name, notes: groupNotes }));

  return { symbolGroups, lineNotes, unresolvedNotes };
}

export function truncateNoteContent(content: string, maxLength = 60): string {
  return content.length > maxLength ? `${content.slice(0, maxLength - 3)}...` : content;
}

export function formatNoteQuickPickDescription(noteView: NoteView): string {
  if (noteView.note.anchor.type === 'Line') {
    return `Line ${noteView.note.anchor.line ?? 1}`;
  }

  const resolvedLine = noteView.resolved?.line ?? noteView.note.anchor.line_hint;
  const lineLabel = typeof resolvedLine === 'number' ? `Line ${resolvedLine}` : 'Unresolved';
  const name = noteView.note.anchor.name ?? 'Symbol';

  return `${lineLabel} · ${name}`;
}

export function formatNoteQuickPickDetail(noteView: NoteView): string | undefined {
  const tags = noteView.note.tags?.filter((tag) => tag.trim().length > 0) ?? [];

  if (tags.length > 0) {
    return tags.join(', ');
  }

  if (noteView.note.updated_at) {
    return `Updated ${noteView.note.updated_at}`;
  }

  return undefined;
}

export function noteQuickPickLabel(noteView: NoteView): string {
  return truncateNoteContent(noteView.note.content);
}

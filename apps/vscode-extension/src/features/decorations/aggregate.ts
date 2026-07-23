import type { NoteView } from '../../types';

export interface LineNoteGroup {
  line: number;
  notes: NoteView[];
}

export function resolveNoteLine(note: NoteView): number {
  if (note.note.anchor.type === 'Line') {
    return note.note.anchor.line ?? 1;
  }

  if (note.resolved?.line) {
    return note.resolved.line;
  }

  return note.note.anchor.line_hint ?? 1;
}

export function sortNotesDeterministic(notes: NoteView[]): NoteView[] {
  return [...notes].sort((left, right) => {
    const kindOrder = anchorKindOrder(left) - anchorKindOrder(right);
    if (kindOrder !== 0) {
      return kindOrder;
    }

    const updatedComparison = compareUpdatedAt(left, right);
    if (updatedComparison !== 0) {
      return updatedComparison;
    }

    return left.note.id.localeCompare(right.note.id);
  });
}

export function aggregateNotesByLine(
  notes: NoteView[],
  lineCount: number,
): LineNoteGroup[] {
  const byLine = new Map<number, NoteView[]>();

  for (const note of notes) {
    const line = resolveNoteLine(note) - 1;

    if (line < 0 || line >= lineCount) {
      continue;
    }

    const group = byLine.get(line) ?? [];
    group.push(note);
    byLine.set(line, group);
  }

  return [...byLine.entries()]
    .sort(([leftLine], [rightLine]) => leftLine - rightLine)
    .map(([line, lineNotes]) => ({
      line,
      notes: sortNotesDeterministic(lineNotes),
    }));
}

function anchorKindOrder(note: NoteView): number {
  return note.note.anchor.type === 'Symbol' ? 0 : 1;
}

function compareUpdatedAt(left: NoteView, right: NoteView): number {
  const leftUpdated = left.note.updated_at ?? '';
  const rightUpdated = right.note.updated_at ?? '';

  return rightUpdated.localeCompare(leftUpdated);
}

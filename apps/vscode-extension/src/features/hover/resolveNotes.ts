import * as vscode from 'vscode';

import type { NoteView } from '../../types';
import { findSymbolAtPosition } from '../../utils/symbols';
import { deduplicateNotesById } from '../presentation/deduplicateNotes';

export function sortNotesForHover(notes: NoteView[]): NoteView[] {
  return [...notes].sort((left, right) => {
    const priorityComparison = comparePriority(left, right);
    if (priorityComparison !== 0) {
      return priorityComparison;
    }

    const kindComparison = anchorKindOrder(left) - anchorKindOrder(right);
    if (kindComparison !== 0) {
      return kindComparison;
    }

    const updatedComparison = compareUpdatedAt(left, right);
    if (updatedComparison !== 0) {
      return updatedComparison;
    }

    return left.note.id.localeCompare(right.note.id);
  });
}

export async function resolveNotesAtPosition(
  notes: NoteView[],
  document: vscode.TextDocument,
  position: vscode.Position,
  token: vscode.CancellationToken,
): Promise<NoteView[]> {
  const symbol = await findSymbolAtPosition(document, position);

  if (token.isCancellationRequested) {
    return [];
  }

  return deduplicateNotesById(
    resolveNotesFromCache(notes, position, symbol?.name),
  );
}

export function resolveNotesFromCache(
  notes: NoteView[],
  position: vscode.Position,
  symbolName?: string,
): NoteView[] {
  if (symbolName) {
    const byName = notes.filter(
      (note) =>
        note.note.anchor.type === 'Symbol' && note.note.anchor.name === symbolName,
    );

    if (byName.length > 0) {
      return deduplicateNotesById(sortNotesForHover(byName));
    }
  }

  const symbolMatches = symbolNotesAtPosition(notes, symbolName, position);
  if (symbolMatches.length > 0) {
    return deduplicateNotesById(symbolMatches);
  }

  return deduplicateNotesById(lineNotesAtPosition(notes, position));
}

function symbolNotesAtPosition(
  notes: NoteView[],
  symbolName: string | undefined,
  position: vscode.Position,
): NoteView[] {
  if (symbolName) {
    return [];
  }

  const symbolNotes = notes.filter(
    (note) => note.note.anchor.type === 'Symbol' && note.resolved,
  );
  const byPosition = symbolNotes.filter((note) => {
    const line = (note.resolved?.line ?? 1) - 1;
    return line === position.line;
  });

  return sortNotesForHover(byPosition);
}

function lineNotesAtPosition(
  notes: NoteView[],
  position: vscode.Position,
): NoteView[] {
  const lineNotes = notes.filter(
    (note) =>
      note.note.anchor.type === 'Line' &&
      (note.note.anchor.line ?? 1) - 1 === position.line,
  );

  return sortNotesForHover(lineNotes);
}

function anchorKindOrder(note: NoteView): number {
  return note.note.anchor.type === 'Symbol' ? 0 : 1;
}

function comparePriority(left: NoteView, right: NoteView): number {
  const leftPriority = left.note.priority ?? 0;
  const rightPriority = right.note.priority ?? 0;

  return rightPriority - leftPriority;
}

function compareUpdatedAt(left: NoteView, right: NoteView): number {
  const leftUpdated = left.note.updated_at ?? '';
  const rightUpdated = right.note.updated_at ?? '';

  return rightUpdated.localeCompare(leftUpdated);
}

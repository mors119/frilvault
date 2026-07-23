import * as vscode from 'vscode';

import type { NoteView } from '../../types';
import { buildNoteUri } from '../decorations/gutterActions';

export type ResolutionStatus = 'resolved' | 'stale' | 'unresolved';

export type ResolvedNoteAnchor =
  | {
      kind: 'symbol';
      symbolName: string;
      symbolKind?: string;
      resolvedRange?: vscode.Range;
      resolutionStatus: ResolutionStatus;
    }
  | {
      kind: 'line';
      line: number;
      column?: number;
      resolvedRange: vscode.Range;
    };

export interface EditorNoteView {
  noteId: string;
  sourceFile: string;
  content: string;
  noteType?: string;
  tags: string[];
  priority?: number;
  updatedAt?: string;
  decisionMetadata?: string;
  anchor: ResolvedNoteAnchor;
  stableUri: string;
}

/** Converts a CLI note view into the shared editor presentation model. */
export function toEditorNoteView(note: NoteView, workspaceRoot: string): EditorNoteView {
  return {
    noteId: note.note.id,
    sourceFile: note.source_file,
    content: note.note.content,
    noteType: inferNoteType(note),
    tags: note.note.tags ?? [],
    priority: note.note.priority,
    updatedAt: note.note.updated_at,
    decisionMetadata: note.note.decision_metadata,
    anchor: resolveNoteAnchor(note),
    stableUri: buildNoteUri(note.note.id, workspaceRoot),
  };
}

/** Returns the 1-based line used for gutter aggregation, or undefined for unresolved symbols. */
export function resolveNoteLine(note: NoteView): number | undefined {
  if (note.note.anchor.type === 'Line') {
    return note.note.anchor.line ?? 1;
  }

  if (!note.resolved) {
    return undefined;
  }

  return note.resolved.line;
}

export function resolveNoteRange(note: NoteView, lineCount: number): vscode.Range | undefined {
  if (note.note.anchor.type === 'Line') {
    const line = Math.max((note.note.anchor.line ?? 1) - 1, 0);

    if (line >= lineCount) {
      return undefined;
    }

    const column = Math.max((note.note.anchor.column ?? 1) - 1, 0);

    return new vscode.Range(line, column, line, column);
  }

  if (!note.resolved) {
    return undefined;
  }

  const line = Math.max(note.resolved.line - 1, 0);

  if (line >= lineCount) {
    return undefined;
  }

  const column = Math.max(note.resolved.column - 1, 0);

  return new vscode.Range(line, column, line, column);
}

export function formatNoteTitle(view: EditorNoteView): string {
  if (view.noteType) {
    return view.noteType;
  }

  if (view.anchor.kind === 'symbol') {
    return view.anchor.symbolName;
  }

  const firstLine = view.content.split('\n').find((line) => line.trim().length > 0);

  return firstLine?.trim() ?? 'FrilVault Note';
}

export function formatAnchorHeading(anchor: ResolvedNoteAnchor): string {
  if (anchor.kind === 'line') {
    const columnSuffix =
      typeof anchor.column === 'number' ? `:${anchor.column}` : '';

    return `Line ${anchor.line}${columnSuffix}`;
  }

  return 'Symbol';
}

export function formatAnchorDetail(anchor: ResolvedNoteAnchor): string | undefined {
  if (anchor.kind === 'line') {
    return undefined;
  }

  const kindLabel = anchor.symbolKind ?? 'Symbol';

  return `${kindLabel}: ${anchor.symbolName}`;
}

export function formatResolutionWarning(anchor: ResolvedNoteAnchor): string | undefined {
  if (anchor.kind !== 'symbol' || anchor.resolutionStatus !== 'unresolved') {
    return undefined;
  }

  return 'Could not locate the current symbol';
}

export function normalizeInlineContent(content: string): string {
  return content.replace(/\s+/g, ' ').trim();
}

export function truncateInlineContent(content: string, maxLength: number): string {
  const normalized = normalizeInlineContent(content);

  if (normalized.length <= maxLength) {
    return normalized;
  }

  return `${normalized.slice(0, Math.max(maxLength - 3, 0))}...`;
}

export function isInlineLineNotesEnabled(): boolean {
  return vscode.workspace.getConfiguration('frilvault').get<boolean>('inlineLineNotes.enabled', true);
}

export function getInlineLineNotesMaxLength(): number {
  const configured = vscode.workspace
    .getConfiguration('frilvault')
    .get<number>('inlineLineNotes.maxLength', 120);

  if (!Number.isFinite(configured) || configured < 20) {
    return 120;
  }

  return Math.floor(configured);
}

function resolveNoteAnchor(note: NoteView): ResolvedNoteAnchor {
  if (note.note.anchor.type === 'Line') {
    const line = note.note.anchor.line ?? 1;
    const column = note.note.anchor.column ?? 1;

    return {
      kind: 'line',
      line,
      column,
      resolvedRange: new vscode.Range(line - 1, column - 1, line - 1, column - 1),
    };
  }

  const symbolName = note.note.anchor.name ?? 'Symbol';
  const resolutionStatus: ResolutionStatus = note.resolved ? 'resolved' : 'unresolved';
  const resolvedRange = note.resolved
    ? new vscode.Range(
        note.resolved.line - 1,
        note.resolved.column - 1,
        note.resolved.line - 1,
        note.resolved.column - 1,
      )
    : undefined;

  return {
    kind: 'symbol',
    symbolName,
    symbolKind: note.note.anchor.kind,
    resolvedRange,
    resolutionStatus,
  };
}

function inferNoteType(note: NoteView): string | undefined {
  if (note.note.title?.trim()) {
    return note.note.title.trim();
  }

  const tag = note.note.tags?.find((entry) => entry.trim().length > 0);

  return tag?.trim();
}

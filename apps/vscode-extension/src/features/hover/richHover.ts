import * as vscode from 'vscode';

import type { NoteView } from '../../types';
import {
  formatEditorNoteHover,
  formatEditorNotesHover,
  NOTE_HOVER_COMMANDS,
} from '../presentation/noteHover';

export const RICH_HOVER_COMMANDS = NOTE_HOVER_COMMANDS;

const DEFAULT_PREVIEW_LENGTH = 800;

export function getConfiguredPreviewLength(): number {
  const configured = vscode.workspace
    .getConfiguration('frilvault')
    .get<number>('hoverPreviewLength', DEFAULT_PREVIEW_LENGTH);

  if (!Number.isFinite(configured) || configured < 1) {
    return DEFAULT_PREVIEW_LENGTH;
  }

  return Math.floor(configured);
}

export function formatRichNoteHover(
  note: NoteView,
  workspaceRoot: string,
  sourceFile: string,
  previewLength = getConfiguredPreviewLength(),
): vscode.MarkdownString {
  return formatEditorNoteHover(note, workspaceRoot, sourceFile, previewLength);
}

export function formatRichNotesHover(
  notes: NoteView[],
  workspaceRoot: string,
  sourceFile: string,
  previewLength = getConfiguredPreviewLength(),
): vscode.MarkdownString {
  return formatEditorNotesHover(notes, workspaceRoot, sourceFile, previewLength);
}

export function truncateMarkdownContent(
  content: string,
  limit: number,
): { preview: string; truncated: boolean } {
  if (content.length <= limit) {
    return { preview: content, truncated: false };
  }

  const fenceStart = content.lastIndexOf('```', limit);
  const fenceEnd = fenceStart >= 0 ? content.indexOf('```', fenceStart + 3) : -1;

  if (fenceStart >= 0 && (fenceEnd < 0 || fenceEnd > limit)) {
    const safeBreak = content.lastIndexOf('\n', Math.max(fenceStart - 1, 0));
    const preview = content.slice(0, safeBreak > 0 ? safeBreak : fenceStart).trimEnd();

    return { preview: `${preview}\n\n...`, truncated: true };
  }

  const lineBreak = content.lastIndexOf('\n', limit);
  const cut = lineBreak > limit * 0.5 ? lineBreak : limit;
  const preview = content.slice(0, cut).trimEnd();

  return { preview: `${preview}\n\n...`, truncated: true };
}

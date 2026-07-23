import * as vscode from 'vscode';

import type { NoteView } from '../../types';
import { buildEditorNotesHoverParts } from '../presentation/noteHover';
import { getConfiguredPreviewLength } from '../hover/richHover';

export function formatGutterHoverSummary(
  notes: NoteView[],
  sourceFile: string,
  workspaceRoot: string,
): vscode.MarkdownString[] {
  return buildEditorNotesHoverParts(
    notes,
    workspaceRoot,
    sourceFile,
    Math.min(getConfiguredPreviewLength(), 240),
  ).contents;
}

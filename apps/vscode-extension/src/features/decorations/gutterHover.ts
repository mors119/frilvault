import * as vscode from 'vscode';

import type { NoteView } from '../../types';
import { formatEditorNotesHover } from '../presentation/noteHover';
import { getConfiguredPreviewLength } from '../hover/richHover';

export function formatGutterHoverSummary(
  notes: NoteView[],
  sourceFile: string,
  workspaceRoot: string,
): vscode.MarkdownString {
  return formatEditorNotesHover(
    notes,
    workspaceRoot,
    sourceFile,
    Math.min(getConfiguredPreviewLength(), 240),
  );
}

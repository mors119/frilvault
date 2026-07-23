import * as path from 'node:path';

import * as vscode from 'vscode';

import { formatEditorNoteHover } from '../features/presentation/noteHover';
import { getConfiguredPreviewLength } from '../features/hover/richHover';
import type { NoteAttachment, NoteView } from '../types';

export function appendNoteAttachments(
  markdown: vscode.MarkdownString,
  note: NoteView,
  workspaceRoot: string,
): void {
  const attachments = note.note.attachments ?? [];

  for (const attachment of attachments) {
    const imagePath = attachmentPath(workspaceRoot, note.note.id, attachment);
    const imageUri = vscode.Uri.file(imagePath).with({ scheme: 'vscode-resource' });

    markdown.appendMarkdown(`\n\n![${attachment.filename}](${imageUri})`);
  }
}

export function attachmentPath(
  workspaceRoot: string,
  noteId: string,
  attachment: NoteAttachment,
): string {
  return path.join(
    workspaceRoot,
    '.vault',
    'images',
    noteId,
    `${attachment.id}.${attachment.extension}`,
  );
}

export function formatNoteHover(
  note: NoteView,
  workspaceRoot: string,
  sourceFile = note.source_file,
): vscode.MarkdownString {
  return formatEditorNoteHover(
    note,
    workspaceRoot,
    sourceFile,
    getConfiguredPreviewLength(),
  );
}

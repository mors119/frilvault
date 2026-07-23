import * as vscode from 'vscode';

import { appendNoteAttachments } from '../../utils/noteMarkdown';
import type { NoteView } from '../../types';

export const RICH_HOVER_COMMANDS = [
  'frilvault.gutter.viewNote',
  'frilvault.gutter.editNote',
  'frilvault.notesPanel.openNote',
] as const;

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
  const markdown = createTrustedMarkdown();
  const title = escapeMarkdownInline(noteTitle(note));

  markdown.appendMarkdown(`### ${title}\n\n`);
  markdown.appendMarkdown(`**Kind:** ${escapeMarkdownInline(noteKindLabel(note))}\n\n`);

  appendAnchorDetails(markdown, note);
  appendTags(markdown, note);
  appendUpdatedTime(markdown, note);
  appendDecisionMetadata(markdown, note);

  const { preview, truncated } = truncateMarkdownContent(note.note.content, previewLength);
  markdown.appendMarkdown(`${preview}\n\n`);
  appendNoteAttachments(markdown, note, workspaceRoot);

  if (truncated) {
    markdown.appendMarkdown(
      `[Open Note](${commandUri('frilvault.gutter.viewNote', [note.note.id, sourceFile])}) | `,
    );
    markdown.appendMarkdown(
      `[Edit](${commandUri('frilvault.gutter.editNote', [note.note.id, sourceFile])})\n\n`,
    );
  } else {
    markdown.appendMarkdown(
      `[Edit](${commandUri('frilvault.gutter.editNote', [note.note.id, sourceFile])})\n\n`,
    );
  }

  return markdown;
}

export function formatRichNotesHover(
  notes: NoteView[],
  workspaceRoot: string,
  sourceFile: string,
  previewLength = getConfiguredPreviewLength(),
): vscode.MarkdownString {
  if (notes.length === 1) {
    return formatRichNoteHover(notes[0], workspaceRoot, sourceFile, previewLength);
  }

  const markdown = createTrustedMarkdown();

  for (const [index, note] of notes.entries()) {
    if (index > 0) {
      markdown.appendMarkdown('\n\n---\n\n');
    }

    markdown.appendMarkdown(formatRichNoteHover(note, workspaceRoot, sourceFile, previewLength).value);
  }

  return markdown;
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

function createTrustedMarkdown(): vscode.MarkdownString {
  const markdown = new vscode.MarkdownString(undefined, true);
  markdown.isTrusted = { enabledCommands: [...RICH_HOVER_COMMANDS] };
  markdown.supportHtml = false;
  return markdown;
}

function appendAnchorDetails(markdown: vscode.MarkdownString, note: NoteView): void {
  if (note.note.anchor.type === 'Line') {
    markdown.appendMarkdown(
      `**Anchor:** Line ${note.note.anchor.line ?? 1}:${note.note.anchor.column ?? 1}\n\n`,
    );
    return;
  }

  const resolvedLine = note.resolved?.line ?? note.note.anchor.line_hint;
  const lineLabel = typeof resolvedLine === 'number' ? `L${resolvedLine}` : 'unknown line';
  const kind = note.note.anchor.kind ?? 'Symbol';
  markdown.appendMarkdown(
    `**Anchor:** ${escapeMarkdownInline(note.note.anchor.name ?? 'Symbol')} (${escapeMarkdownInline(kind)}) at ${lineLabel}\n\n`,
  );

  if (note.note.anchor.signature) {
    markdown.appendMarkdown(
      `\`${escapeMarkdownInline(note.note.anchor.signature)}\`\n\n`,
    );
  }
}

function appendTags(markdown: vscode.MarkdownString, note: NoteView): void {
  const tags = note.note.tags ?? [];

  if (tags.length === 0) {
    return;
  }

  markdown.appendMarkdown(`**Tags:** ${tags.map(escapeMarkdownInline).join(', ')}\n\n`);
}

function appendUpdatedTime(markdown: vscode.MarkdownString, note: NoteView): void {
  if (!note.note.updated_at) {
    return;
  }

  markdown.appendMarkdown(`**Updated:** ${escapeMarkdownInline(formatUpdatedAt(note.note.updated_at))}\n\n`);
}

function appendDecisionMetadata(markdown: vscode.MarkdownString, note: NoteView): void {
  const metadata = note.note.decision_metadata;

  if (!metadata) {
    return;
  }

  markdown.appendMarkdown(`**Decision:** ${escapeMarkdownInline(metadata)}\n\n`);
}

function noteTitle(note: NoteView): string {
  if (note.note.title?.trim()) {
    return note.note.title.trim();
  }

  if (note.note.anchor.type === 'Symbol' && note.note.anchor.name) {
    return note.note.anchor.name;
  }

  const firstLine = note.note.content.split('\n').find((line) => line.trim().length > 0);

  return firstLine?.trim() ?? 'FrilVault Note';
}

function noteKindLabel(note: NoteView): string {
  if (note.note.anchor.type === 'Symbol') {
    return note.note.anchor.kind ?? 'Symbol';
  }

  return 'Line';
}

function formatUpdatedAt(value: string): string {
  const parsed = Date.parse(value);

  if (Number.isNaN(parsed)) {
    return value;
  }

  return new Date(parsed).toLocaleString();
}

function escapeMarkdownInline(value: string): string {
  return value.replace(/([\\`*_[\]<>])/g, '\\$1');
}

function commandUri(command: string, args: unknown[]): string {
  return `command:${command}?${encodeURIComponent(JSON.stringify(args))}`;
}

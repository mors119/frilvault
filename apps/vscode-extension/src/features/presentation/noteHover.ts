import * as vscode from 'vscode';

import { COMMAND_IDS } from '../../constants/ids';
import type { NoteView } from '../../types';
import { appendNoteAttachments } from '../../utils/noteMarkdown';
import {
  formatAnchorDetail,
  formatAnchorHeading,
  formatNoteTitle,
  formatResolutionWarning,
  toEditorNoteView,
  type EditorNoteView,
} from '../presentation/editorNoteView';
import { truncateMarkdownContent } from '../hover/richHover';

export const NOTE_HOVER_COMMANDS = [
  'frilvault.gutter.showActions',
  'frilvault.gutter.viewNote',
  'frilvault.gutter.editNote',
  'frilvault.gutter.deleteNote',
  'frilvault.gutter.copyLink',
  COMMAND_IDS.notesPanelOpenNote,
] as const;

export function formatEditorNoteHover(
  note: NoteView,
  workspaceRoot: string,
  sourceFile: string,
  previewLength: number,
): vscode.MarkdownString {
  const view = toEditorNoteView(note, workspaceRoot);
  const markdown = createTrustedMarkdown();

  markdown.appendMarkdown(`**FrilVault**\n\n`);
  markdown.appendMarkdown(`**${escapeMarkdownInline(formatNoteTitle(view))}**\n\n`);
  appendAnchorSection(markdown, view);
  appendTags(markdown, view);
  appendUpdatedTime(markdown, view);
  appendDecisionMetadata(markdown, view);

  const { preview, truncated } = truncateMarkdownContent(view.content, previewLength);
  markdown.appendMarkdown(`${preview}\n\n`);
  appendNoteAttachments(markdown, note, workspaceRoot);

  appendActionLinks(markdown, note.note.id, sourceFile, truncated);

  return markdown;
}

export function formatEditorNotesHover(
  notes: NoteView[],
  workspaceRoot: string,
  sourceFile: string,
  previewLength: number,
): vscode.MarkdownString {
  if (notes.length === 1) {
    return formatEditorNoteHover(notes[0], workspaceRoot, sourceFile, previewLength);
  }

  const markdown = createTrustedMarkdown();

  markdown.appendMarkdown(`**FrilVault Notes (${notes.length})**\n\n`);

  for (const [index, note] of notes.entries()) {
    const view = toEditorNoteView(note, workspaceRoot);

    markdown.appendMarkdown(`${index + 1}. **${escapeMarkdownInline(formatNoteTitle(view))}**\n`);
    markdown.appendMarkdown(`   ${truncateSingleLine(view.content)}\n\n`);
  }

  appendActionLinks(markdown, notes[0].note.id, sourceFile, false, notes.length > 1);

  return markdown;
}

function appendAnchorSection(markdown: vscode.MarkdownString, view: EditorNoteView): void {
  markdown.appendMarkdown(`**Anchor**\n`);
  markdown.appendMarkdown(`${escapeMarkdownInline(formatAnchorHeading(view.anchor))}\n`);

  const detail = formatAnchorDetail(view.anchor);

  if (detail) {
    markdown.appendMarkdown(`${escapeMarkdownInline(detail)}\n`);
  }

  const warning = formatResolutionWarning(view.anchor);

  if (warning) {
    markdown.appendMarkdown(`Status: ${escapeMarkdownInline(warning)}\n`);
  }

  markdown.appendMarkdown('\n');
}

function appendTags(markdown: vscode.MarkdownString, view: EditorNoteView): void {
  if (view.tags.length === 0) {
    return;
  }

  markdown.appendMarkdown(`**Tags:** ${view.tags.map(escapeMarkdownInline).join(', ')}\n\n`);
}

function appendUpdatedTime(markdown: vscode.MarkdownString, view: EditorNoteView): void {
  if (!view.updatedAt) {
    return;
  }

  markdown.appendMarkdown(`**Updated**\n${escapeMarkdownInline(formatUpdatedAt(view.updatedAt))}\n\n`);
}

function appendDecisionMetadata(markdown: vscode.MarkdownString, view: EditorNoteView): void {
  if (!view.decisionMetadata) {
    return;
  }

  markdown.appendMarkdown(`**Decision:** ${escapeMarkdownInline(view.decisionMetadata)}\n\n`);
}

function appendActionLinks(
  markdown: vscode.MarkdownString,
  noteId: string,
  sourceFile: string,
  _truncated: boolean,
  multipleNotes = false,
): void {
  const links = [
    `[Open Note](${commandUri('frilvault.gutter.viewNote', [noteId, sourceFile])})`,
    `[Edit](${commandUri('frilvault.gutter.editNote', [noteId, sourceFile])})`,
    `[Delete](${commandUri('frilvault.gutter.deleteNote', [noteId, sourceFile])})`,
    `[Copy Link](${commandUri('frilvault.gutter.copyLink', [noteId, sourceFile])})`,
  ];

  if (multipleNotes) {
    links.unshift(
      `[Choose note…](${commandUri('frilvault.gutter.showActions', [0, sourceFile])})`,
    );
  }

  markdown.appendMarkdown(`${links.join(' · ')}\n`);
}

function createTrustedMarkdown(): vscode.MarkdownString {
  const markdown = new vscode.MarkdownString(undefined, true);
  markdown.isTrusted = { enabledCommands: [...NOTE_HOVER_COMMANDS] };
  markdown.supportHtml = false;
  return markdown;
}

function truncateSingleLine(content: string, limit = 80): string {
  const normalized = content.replace(/\s+/g, ' ').trim();

  if (normalized.length <= limit) {
    return normalized;
  }

  return `${normalized.slice(0, limit - 3)}...`;
}

function formatUpdatedAt(value: string): string {
  const parsed = Date.parse(value);

  if (Number.isNaN(parsed)) {
    return value;
  }

  return new Date(parsed).toLocaleDateString();
}

function escapeMarkdownInline(value: string): string {
  return value.replace(/([\\`*_[\]<>])/g, '\\$1');
}

function commandUri(command: string, args: unknown[]): string {
  return `command:${command}?${encodeURIComponent(JSON.stringify(args))}`;
}

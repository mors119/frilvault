import * as vscode from 'vscode';

import type { NoteView } from '../../types';
import { sortNotesDeterministic } from './aggregate';

const SUMMARY_CONTENT_LIMIT = 120;

const GUTTER_COMMANDS = [
  'frilvault.gutter.showActions',
  'frilvault.gutter.viewNote',
  'frilvault.gutter.editNote',
  'frilvault.gutter.deleteNote',
  'frilvault.gutter.copyLink',
] as const;

export function formatGutterHoverSummary(
  notes: NoteView[],
  sourceFile: string,
): vscode.MarkdownString {
  const markdown = new vscode.MarkdownString(undefined, true);
  markdown.isTrusted = { enabledCommands: [...GUTTER_COMMANDS] };
  markdown.supportHtml = false;

  const ordered = sortNotesDeterministic(notes);
  const primary = ordered[0];

  markdown.appendMarkdown(`**${noteKindLabel(primary)}**\n\n`);
  markdown.appendMarkdown(`${truncateContent(primary.note.content)}\n\n`);

  const tags = primary.note.tags ?? [];
  if (tags.length > 0) {
    markdown.appendMarkdown(`Tags: ${tags.join(', ')}\n\n`);
  }

  if (primary.note.updated_at) {
    markdown.appendMarkdown(`Updated: ${formatUpdatedAt(primary.note.updated_at)}\n\n`);
  }

  if (ordered.length > 1) {
    markdown.appendMarkdown(`+ ${ordered.length - 1} more note(s) on this line\n\n`);
  }

  appendActionLinks(markdown, ordered, sourceFile);

  return markdown;
}

function appendActionLinks(
  markdown: vscode.MarkdownString,
  notes: NoteView[],
  sourceFile: string,
): void {
  if (notes.length === 1) {
    const note = notes[0];
    markdown.appendMarkdown(
      `[View](${commandUri('frilvault.gutter.viewNote', [note.note.id, sourceFile])}) | `,
    );
    markdown.appendMarkdown(
      `[Edit](${commandUri('frilvault.gutter.editNote', [note.note.id, sourceFile])}) | `,
    );
    markdown.appendMarkdown(
      `[Delete](${commandUri('frilvault.gutter.deleteNote', [note.note.id, sourceFile])}) | `,
    );
    markdown.appendMarkdown(
      `[Copy Link](${commandUri('frilvault.gutter.copyLink', [note.note.id, sourceFile])})`,
    );
    return;
  }

  markdown.appendMarkdown(
    `[Choose note…](${commandUri('frilvault.gutter.showActions', [resolveNoteLine(notes[0]) - 1, sourceFile])})`,
  );
}

function commandUri(command: string, args: unknown[]): string {
  return `command:${command}?${encodeURIComponent(JSON.stringify(args))}`;
}

function noteKindLabel(note: NoteView): string {
  if (note.note.anchor.type === 'Symbol') {
    return note.note.anchor.name ?? 'Symbol note';
  }

  return 'Line note';
}

function truncateContent(content: string): string {
  if (content.length <= SUMMARY_CONTENT_LIMIT) {
    return content;
  }

  return `${content.slice(0, SUMMARY_CONTENT_LIMIT - 3)}...`;
}

function formatUpdatedAt(value: string): string {
  const parsed = Date.parse(value);

  if (Number.isNaN(parsed)) {
    return value;
  }

  return new Date(parsed).toLocaleString();
}

function resolveNoteLine(note: NoteView): number {
  if (note.note.anchor.type === 'Line') {
    return note.note.anchor.line ?? 1;
  }

  if (note.resolved?.line) {
    return note.resolved.line;
  }

  return note.note.anchor.line_hint ?? 1;
}

import * as vscode from 'vscode';

const DEFAULT_INLINE_MAX_LENGTH = 80;
const DEFAULT_DEBOUNCE_MS = 500;

export function getInlineNotesDebounceMs(): number {
  const configured = vscode.workspace
    .getConfiguration('frilvault')
    .get<number>('inlineEditor.autoSaveDebounceMs', DEFAULT_DEBOUNCE_MS);

  if (!Number.isFinite(configured) || configured < 100) {
    return DEFAULT_DEBOUNCE_MS;
  }

  return Math.floor(configured);
}

export function isInlineNotesEnabled(): boolean {
  const config = vscode.workspace.getConfiguration('frilvault');

  if (config.has('inlineNotes.enabled')) {
    return config.get<boolean>('inlineNotes.enabled', true);
  }

  return config.get<boolean>('inlineLineNotes.enabled', true);
}

export function showInlineLineNotes(): boolean {
  const config = vscode.workspace.getConfiguration('frilvault');

  if (!isInlineNotesEnabled()) {
    return false;
  }

  if (config.has('inlineNotes.showLineNotes')) {
    return config.get<boolean>('inlineNotes.showLineNotes', true);
  }

  return true;
}

export function showInlineSymbolNotes(): boolean {
  const config = vscode.workspace.getConfiguration('frilvault');

  if (!isInlineNotesEnabled()) {
    return false;
  }

  return config.get<boolean>('inlineNotes.showSymbolNotes', true);
}

export function getInlineNotesMaxLength(): number {
  const config = vscode.workspace.getConfiguration('frilvault');
  const configured = config.has('inlineNotes.maxLength')
    ? config.get<number>('inlineNotes.maxLength', DEFAULT_INLINE_MAX_LENGTH)
    : config.get<number>('inlineLineNotes.maxLength', DEFAULT_INLINE_MAX_LENGTH);

  if (!Number.isFinite(configured) || configured < 20) {
    return DEFAULT_INLINE_MAX_LENGTH;
  }

  return Math.floor(configured);
}

/** @deprecated Use {@link isInlineNotesEnabled}. */
export function isInlineLineNotesEnabled(): boolean {
  return showInlineLineNotes();
}

/** @deprecated Use {@link getInlineNotesMaxLength}. */
export function getInlineLineNotesMaxLength(): number {
  return getInlineNotesMaxLength();
}

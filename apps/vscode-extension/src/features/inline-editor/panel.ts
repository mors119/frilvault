import * as vscode from 'vscode';

import type { InlineNoteDraft } from './draft';

export interface InlineNotePanelMessage {
  type: 'save' | 'cancel' | 'undo';
  content?: string;
  tagsText?: string;
}

export class InlineNotePanel {
  private panel: vscode.WebviewPanel | undefined;
  private draft: InlineNoteDraft | undefined;
  private onMessage:
    | ((message: InlineNotePanelMessage) => void | Promise<void>)
    | undefined;

  public open(
    context: vscode.ExtensionContext,
    draft: InlineNoteDraft,
    onMessage: (message: InlineNotePanelMessage) => void | Promise<void>,
  ): void {
    this.draft = draft;
    this.onMessage = onMessage;

    if (!this.panel) {
      this.panel = vscode.window.createWebviewPanel(
        'frilvault.inlineNoteEditor',
        'FrilVault Note',
        vscode.ViewColumn.Beside,
        {
          enableScripts: true,
          retainContextWhenHidden: true,
          localResourceRoots: [],
        },
      );

      this.panel.onDidDispose(() => {
        this.panel = undefined;
        this.draft = undefined;
        this.onMessage = undefined;
      });

      this.panel.webview.onDidReceiveMessage(async (message: InlineNotePanelMessage) => {
        await this.onMessage?.(message);
      });

      context.subscriptions.push(this.panel);
    }

    this.panel.title = draft.mode === 'create' ? 'Create FrilVault Note' : 'Edit FrilVault Note';
    this.panel.webview.html = renderPanelHtml(draft);
    this.panel.reveal(vscode.ViewColumn.Beside, true);
  }

  public updateDraft(draft: InlineNoteDraft, options?: { errorMessage?: string }): void {
    this.draft = draft;

    if (!this.panel) {
      return;
    }

    void this.panel.webview.postMessage({
      type: 'state',
      draft,
      errorMessage: options?.errorMessage,
    });
  }

  public close(): void {
    this.panel?.dispose();
    this.panel = undefined;
    this.draft = undefined;
    this.onMessage = undefined;
  }

  public isOpen(): boolean {
    return this.panel !== undefined;
  }
}

function renderPanelHtml(draft: InlineNoteDraft): string {
  const nonce = String(Date.now());
  const payload = JSON.stringify({
    content: draft.content,
    tagsText: draft.tagsText,
    mode: draft.mode,
    kind: draft.kind,
    anchorSummary: draft.anchorSummary,
    sourceFile: draft.sourceFile,
    canUndo: Boolean(draft.undoSnapshot && draft.mode === 'edit'),
  });

  return `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src 'unsafe-inline'; script-src 'nonce-${nonce}';" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>FrilVault Note Editor</title>
  <style>
    :root {
      color-scheme: light dark;
      font-family: var(--vscode-font-family);
      font-size: var(--vscode-font-size);
      color: var(--vscode-foreground);
      background: var(--vscode-editor-background);
    }
    body {
      margin: 0;
      padding: 16px;
    }
    form {
      display: grid;
      gap: 12px;
      max-width: 720px;
    }
    label {
      display: grid;
      gap: 6px;
      font-weight: 600;
    }
    .meta {
      color: var(--vscode-descriptionForeground);
      font-weight: 400;
    }
    textarea, input {
      width: 100%;
      box-sizing: border-box;
      padding: 8px;
      border: 1px solid var(--vscode-input-border, #888);
      background: var(--vscode-input-background);
      color: var(--vscode-input-foreground);
      border-radius: 4px;
      font: inherit;
    }
    textarea {
      min-height: 220px;
      resize: vertical;
      line-height: 1.4;
    }
    .actions {
      display: flex;
      gap: 8px;
      flex-wrap: wrap;
    }
    button {
      padding: 6px 12px;
      border: 1px solid var(--vscode-button-border, transparent);
      background: var(--vscode-button-background);
      color: var(--vscode-button-foreground);
      border-radius: 4px;
      cursor: pointer;
      font: inherit;
    }
    button.secondary {
      background: var(--vscode-button-secondaryBackground);
      color: var(--vscode-button-secondaryForeground);
    }
    .error {
      color: var(--vscode-errorForeground);
      min-height: 1.2em;
    }
    .hint {
      color: var(--vscode-descriptionForeground);
      font-size: 0.92em;
    }
  </style>
</head>
<body>
  <form id="note-form" aria-label="FrilVault note editor">
    <div>
      <strong id="mode-label">${escapeHtml(draft.mode === 'create' ? 'Create note' : 'Edit note')}</strong>
      <div class="meta" aria-label="Anchor summary">${escapeHtml(draft.anchorSummary)}</div>
      <div class="meta" aria-label="Source file">${escapeHtml(draft.sourceFile)}</div>
      <div class="meta" aria-label="Note kind">Kind: ${escapeHtml(draft.kind)}</div>
    </div>

    <label for="content">
      Markdown content
      <textarea id="content" name="content" aria-label="Markdown content" spellcheck="true">${escapeHtml(draft.content)}</textarea>
    </label>

    <label for="tags">
      Tags
      <input id="tags" name="tags" aria-label="Comma-separated tags" value="${escapeHtml(draft.tagsText)}" />
    </label>

    <div class="hint">Save: Ctrl/Cmd+Enter. Cancel: Escape.</div>
    <div id="error" class="error" role="alert" aria-live="assertive"></div>

    <div class="actions">
      <button type="submit" id="save-button" aria-label="Save note">Save</button>
      <button type="button" class="secondary" id="cancel-button" aria-label="Cancel editing">Cancel</button>
      <button type="button" class="secondary" id="undo-button" aria-label="Undo last save" ${draft.undoSnapshot && draft.mode === 'edit' ? '' : 'hidden'}>Undo</button>
    </div>
  </form>

  <script nonce="${nonce}">
    const vscode = acquireVsCodeApi();
    const initial = ${payload};

    const form = document.getElementById('note-form');
    const contentInput = document.getElementById('content');
    const tagsInput = document.getElementById('tags');
    const errorEl = document.getElementById('error');
    const undoButton = document.getElementById('undo-button');
    const cancelButton = document.getElementById('cancel-button');

    function currentPayload() {
      return {
        content: contentInput.value,
        tagsText: tagsInput.value,
      };
    }

    form.addEventListener('submit', (event) => {
      event.preventDefault();
      const payload = currentPayload();
      vscode.postMessage({ type: 'save', ...payload });
    });

    cancelButton.addEventListener('click', () => {
      vscode.postMessage({ type: 'cancel' });
    });

    undoButton.addEventListener('click', () => {
      vscode.postMessage({ type: 'undo' });
    });

    window.addEventListener('keydown', (event) => {
      if ((event.metaKey || event.ctrlKey) && event.key === 'Enter') {
        event.preventDefault();
        const payload = currentPayload();
        vscode.postMessage({ type: 'save', ...payload });
      }

      if (event.key === 'Escape') {
        event.preventDefault();
        vscode.postMessage({ type: 'cancel' });
      }
    });

    window.addEventListener('message', (event) => {
      const message = event.data;
      if (!message || message.type !== 'state') {
        return;
      }

      contentInput.value = message.draft?.content ?? contentInput.value;
      tagsInput.value = message.draft?.tagsText ?? tagsInput.value;
      errorEl.textContent = message.errorMessage ?? '';

      if (message.draft?.canUndo) {
        undoButton.hidden = false;
      }
    });
  </script>
</body>
</html>`;
}

function escapeHtml(value: string): string {
  return value
    .replaceAll('&', '&amp;')
    .replaceAll('<', '&lt;')
    .replaceAll('>', '&gt;')
    .replaceAll('"', '&quot;');
}

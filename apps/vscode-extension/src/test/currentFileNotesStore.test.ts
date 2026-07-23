import * as assert from 'node:assert';

import { suite, test } from 'mocha';

import { CliClient } from '../core/cliClient';
import { CurrentFileNotesStore } from '../features/current-file/store';
import type { NoteView } from '../types';

suite('CurrentFileNotesStore', () => {
  test('loads notes for the active editor and ignores stale responses', async () => {
    let callCount = 0;
    const cliClient = {
      listNotes: async (_workspaceRoot: string, sourceFile: string) => {
        callCount += 1;

        if (callCount === 1) {
          await new Promise((resolve) => setTimeout(resolve, 20));
          return [createLineNoteView(sourceFile, 1, 1, 'stale note')];
        }

        return [createLineNoteView(sourceFile, 2, 1, 'fresh note')];
      },
    } as unknown as CliClient;

    const store = new CurrentFileNotesStore(cliClient, () => true, () => '/tmp/workspace');
    const editor = createMockEditor('/tmp/workspace/src/sample.ts');

    const firstLoad = store.syncActiveEditor(editor);
    const secondLoad = store.syncActiveEditor(editor);

    await Promise.all([firstLoad, secondLoad]);

    assert.strictEqual(store.getSnapshot().notes.length, 1);
    assert.strictEqual(store.getSnapshot().notes[0]?.note.content, 'fresh note');
  });

  test('returns empty notes when the CLI fails', async () => {
    const cliClient = {
      listNotes: async () => {
        throw new Error('missing vault');
      },
    } as unknown as CliClient;

    const store = new CurrentFileNotesStore(cliClient, () => true, () => '/tmp/workspace');
    await store.syncActiveEditor(createMockEditor('/tmp/workspace/src/sample.ts'));

    assert.deepStrictEqual(store.getSnapshot().notes, []);
    assert.match(store.getSnapshot().error ?? '', /missing vault/);
  });

  test('clears notes when FrilVault is disabled', async () => {
    let enabled = true;
    const cliClient = {
      listNotes: async (_workspaceRoot: string, sourceFile: string) => [
        createLineNoteView(sourceFile, 1, 1, 'enabled note'),
      ],
    } as unknown as CliClient;

    const store = new CurrentFileNotesStore(cliClient, () => enabled, () => '/tmp/workspace');
    await store.syncActiveEditor(createMockEditor('/tmp/workspace/src/sample.ts'));

    assert.strictEqual(store.getSnapshot().notes.length, 1);

    enabled = false;
    store.clear();

    assert.strictEqual(store.getSnapshot().notes.length, 0);
  });
});

function createMockEditor(filePath: string): import('vscode').TextEditor {
  return {
    document: {
      uri: {
        scheme: 'file',
        toString: () => `file://${filePath}`,
        fsPath: filePath,
      },
    },
  } as import('vscode').TextEditor;
}

function createLineNoteView(
  sourceFile: string,
  line: number,
  column: number,
  content: string,
): NoteView {
  return {
    source_file: sourceFile,
    note: {
      id: `${sourceFile}-${line}-${column}`,
      anchor: {
        type: 'Line' as const,
        line,
        column,
      },
      content,
      created_at: '2026-06-09T00:00:00Z',
      updated_at: '2026-06-09T00:00:00Z',
    },
  };
}

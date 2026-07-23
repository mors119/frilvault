import * as assert from 'node:assert';

import { suite, test } from 'mocha';

import { createAddNoteCommand } from '../features/inline-editor/command';
import type { InlineNoteEditor } from '../features/inline-editor/editor';

suite('Add note command', () => {
  test('delegates to the inline editor creation flow', async () => {
    let called = false;
    const editor = {
      openCreateHere: async () => {
        called = true;
      },
    } as unknown as InlineNoteEditor;

    await createAddNoteCommand(editor)();

    assert.strictEqual(called, true);
  });
});

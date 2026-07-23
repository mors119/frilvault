import * as assert from 'node:assert';

import { suite, test } from 'mocha';

import {
  createInlinePreview,
  formatInlineNotesPreview,
  normalizeNoteForInlineDisplay,
} from '../features/presentation/inlinePreview';

suite('Inline note preview', () => {
  test('normalizes markdown and whitespace for inline display', () => {
    const normalized = normalizeNoteForInlineDisplay('# Title\n\n**bold** text');

    assert.strictEqual(normalized, 'Title bold text');
  });

  test('truncates long content with a Unicode ellipsis', () => {
    const preview = createInlinePreview('abcdefghijklmnop', 8);

    assert.strictEqual(preview, 'abcdefgh…');
  });

  test('formats multiple notes with an additional count', () => {
    const preview = formatInlineNotesPreview(
      [
        { source_file: 'src/a.ts', note: { id: 'a', content: 'first note', anchor: { type: 'Line', line: 1 } } },
        { source_file: 'src/a.ts', note: { id: 'b', content: 'second note', anchor: { type: 'Line', line: 1 } } },
      ],
      40,
    );

    assert.match(preview, /^Note: first note \(\+1\)$/);
  });
});

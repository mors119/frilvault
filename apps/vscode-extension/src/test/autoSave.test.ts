import * as assert from 'node:assert';

import { suite, test } from 'mocha';

import {
  DebouncedAutoSave,
  draftFingerprint,
} from '../features/inline-editor/autoSave';

suite('Inline note auto-save', () => {
  test('schedules one debounced save for rapid changes', async () => {
    let saveCount = 0;
    const statuses: string[] = [];
    const autoSave = new DebouncedAutoSave(
      20,
      (status) => {
        statuses.push(status);
      },
      async () => {
        saveCount += 1;
      },
    );

    autoSave.setPersistedFingerprint('initial');
    autoSave.schedule('changed-1');
    autoSave.schedule('changed-2');
    await autoSave.flush();

    assert.strictEqual(saveCount, 1);
    assert.ok(statuses.includes('editing'));
    assert.ok(statuses.includes('saved'));
  });

  test('skips persistence when content fingerprint is unchanged', () => {
    let saveCount = 0;
    const autoSave = new DebouncedAutoSave(
      10,
      () => undefined,
      async () => {
        saveCount += 1;
      },
    );

    const fingerprint = draftFingerprint('same', 'tag');
    autoSave.setPersistedFingerprint(fingerprint);
    autoSave.schedule(fingerprint);

    assert.strictEqual(saveCount, 0);
  });

  test('ignores stale save completions', async () => {
    const appliedGenerations: number[] = [];
    let currentGeneration = 0;
    const autoSave = new DebouncedAutoSave(
      10,
      () => undefined,
      async (generation) => {
        appliedGenerations.push(generation);
        if (generation === 1) {
          await new Promise((resolve) => setTimeout(resolve, 30));
        }
      },
    );

    autoSave.setPersistedFingerprint('initial');
    autoSave.schedule('first');
    currentGeneration += 1;
    const first = autoSave.flush();
    autoSave.schedule('second');
    currentGeneration += 1;
    const second = autoSave.flush();
    await Promise.all([first, second]);

    assert.ok(appliedGenerations.includes(1));
    assert.ok(appliedGenerations.includes(2));
    assert.strictEqual(currentGeneration, 2);
  });
});

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

    autoSave.reset('initial');
    autoSave.schedule('changed-1', 1);
    autoSave.schedule('changed-2', 2);
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
    autoSave.reset(fingerprint);
    autoSave.schedule(fingerprint, 1);

    assert.strictEqual(saveCount, 0);
  });

  test('serializes persistence and queues the latest revision', async () => {
    const appliedGenerations: number[] = [];
    let releaseFirstSave: (() => void) | undefined;
    const autoSave = new DebouncedAutoSave(
      10,
      () => undefined,
      async (revision) => {
        appliedGenerations.push(revision);
        if (revision === 1) {
          await new Promise<void>((resolve) => {
            releaseFirstSave = resolve;
          });
        }
      },
    );

    autoSave.reset('initial');
    autoSave.schedule('first', 1);
    const first = autoSave.flush();
    autoSave.schedule('second', 2);

    assert.deepStrictEqual(appliedGenerations, [1]);

    releaseFirstSave?.();
    await first;

    assert.ok(appliedGenerations.includes(1));
    assert.ok(appliedGenerations.includes(2));
  });

  test('defers persistence until IME composition completes', async () => {
    let saveCount = 0;
    const autoSave = new DebouncedAutoSave(
      10,
      () => undefined,
      async () => {
        saveCount += 1;
      },
    );

    autoSave.reset('initial');
    autoSave.startComposition();
    autoSave.schedule('draft', 1);
    await autoSave.flush();

    assert.strictEqual(saveCount, 0);

    autoSave.endComposition();
    await autoSave.flush();

    assert.strictEqual(saveCount, 1);
  });
});

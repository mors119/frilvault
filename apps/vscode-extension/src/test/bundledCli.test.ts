import * as assert from 'node:assert';

import { suite, test } from 'mocha';

import {
  bundledCliFileName,
  resolveBundledCliPath,
  resolveCliPath,
} from '../core/bundledCli';

suite('bundledCli', () => {
  test('prefers an explicit cliPath override', () => {
    const resolved = resolveCliPath({
      configuredCliPath: '/custom/flvt',
      extensionPath: '/extension',
      existsSync: () => true,
    });

    assert.deepStrictEqual(resolved, {
      cliPath: '/custom/flvt',
      source: 'configured',
    });
  });

  test('resolves the bundled CLI inside the extension bin directory', () => {
    const resolved = resolveCliPath({
      configuredCliPath: '',
      extensionPath: '/extension',
      existsSync: (filePath) => filePath === '/extension/bin/flvt',
    });

    assert.deepStrictEqual(resolved, {
      cliPath: '/extension/bin/flvt',
      source: 'bundled',
    });
  });

  test('returns missing when neither override nor bundled CLI exists', () => {
    const resolved = resolveCliPath({
      configuredCliPath: '',
      extensionPath: '/extension',
      existsSync: () => false,
    });

    assert.deepStrictEqual(resolved, { source: 'missing' });
  });

  test('uses the .exe suffix for Windows bundles', () => {
    assert.strictEqual(bundledCliFileName('win32'), 'flvt.exe');
    assert.strictEqual(resolveBundledCliPath('/extension', 'win32'), '/extension/bin/flvt.exe');
  });
});

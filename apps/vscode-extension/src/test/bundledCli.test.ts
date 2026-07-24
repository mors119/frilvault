import * as assert from 'node:assert';

import { suite, test } from 'mocha';

import {
  bundledCliFileName,
  bundledCliTarget,
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
      platform: 'darwin',
      arch: 'arm64',
      existsSync: (filePath) => filePath === '/extension/bin/darwin-arm64/flvt',
    });

    assert.deepStrictEqual(resolved, {
      cliPath: '/extension/bin/darwin-arm64/flvt',
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
    assert.strictEqual(bundledCliTarget('win32', 'x64'), 'win32-x64');
    assert.strictEqual(
      resolveBundledCliPath('/extension', 'win32', 'x64'),
      '/extension/bin/win32-x64/flvt.exe',
    );
  });

  test('throws for unsupported host targets', () => {
    assert.throws(
      () => bundledCliTarget('linux', 'arm64'),
      /Unsupported platform: linux-arm64/,
    );
  });
});

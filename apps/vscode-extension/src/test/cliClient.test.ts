import * as assert from 'node:assert';

import { suite, test } from 'mocha';

import { CliClient } from '../core/cliClient';

suite('CliClient', () => {
  test('uses the bundled CLI by default when cliPath is empty', async () => {
    const calls: string[] = [];
    const logs: string[] = [];
    const cliClient = new CliClient({
      extensionPath: '/extension',
      extensionVersion: '0.1.0',
      existsSync: (filePath) => filePath === '/extension/bin/flvt',
      execFile: async (file, args) => {
        calls.push(`${file} ${args.join(' ')}`);

        if (args[0] === '--version') {
          return { stdout: 'flvt 0.1.0\n', stderr: '' };
        }

        return { stdout: '[]', stderr: '' };
      },
      outputChannel: {
        appendLine(value) {
          logs.push(value);
        },
      },
    });

    const notes = await cliClient.listNotes('/workspace', 'src/sample.ts');

    assert.deepStrictEqual(notes, []);
    assert.deepStrictEqual(calls, [
      '/extension/bin/flvt --version',
      '/extension/bin/flvt list --file src/sample.ts --format json',
    ]);
    assert.ok(logs.some((line) => line.includes('path=/extension/bin/flvt')));
  });

  test('prefers a custom cliPath override over the bundled CLI', async () => {
    const calls: string[] = [];
    const cliClient = new CliClient({
      getConfiguredCliPath: () => '/custom/flvt',
      extensionPath: '/extension',
      extensionVersion: '0.1.0',
      existsSync: () => true,
      execFile: async (file, args) => {
        calls.push(`${file} ${args.join(' ')}`);

        if (args[0] === '--version') {
          return { stdout: 'flvt 0.1.0\n', stderr: '' };
        }

        return { stdout: '[]', stderr: '' };
      },
    });

    await cliClient.listNotes('/workspace', 'src/sample.ts');

    assert.deepStrictEqual(calls, [
      '/custom/flvt --version',
      '/custom/flvt list --file src/sample.ts --format json',
    ]);
  });

  test('reports a clear error when no bundled CLI is available', async () => {
    const cliClient = new CliClient({
      extensionPath: '/extension',
      existsSync: () => false,
    });

    await assert.rejects(
      async () => {
        await cliClient.listNotes('/workspace', 'src/sample.ts');
      },
      (error: unknown) => {
        assert.ok(error instanceof Error);
        assert.match(error.message, /frilvault cli could not be started/i);
        assert.match(error.message, /no bundled cli was found/i);
        return true;
      },
    );
  });

  test('reports non-executable bundled binaries before spawning', async () => {
    const cliClient = new CliClient({
      extensionPath: '/extension',
      existsSync: () => true,
      access: async () => {
        const error = new Error('permission denied') as Error & { code?: string };
        error.code = 'EACCES';
        throw error;
      },
    });

    await assert.rejects(
      async () => {
        await cliClient.listNotes('/workspace', 'src/sample.ts');
      },
      (error: unknown) => {
        assert.ok(error instanceof Error);
        assert.match(error.message, /not runnable/i);
        return true;
      },
    );
  });

  test('fails fast when the CLI version does not match the extension expectation', async () => {
    const cliClient = new CliClient({
      extensionPath: '/extension',
      extensionVersion: '0.0.1',
      existsSync: () => true,
      execFile: async (_file, args) => {
        if (args[0] === '--version') {
          return { stdout: 'flvt 0.1.0\n', stderr: '' };
        }

        return { stdout: '[]', stderr: '' };
      },
    });

    await assert.rejects(
      async () => {
        await cliClient.listNotes('/workspace', 'src/sample.ts');
      },
      (error: unknown) => {
        assert.ok(error instanceof Error);
        assert.strictEqual(
          error.message,
          'FrilVault CLI version mismatch. Expected 0.0.1, found 0.1.0.',
        );
        return true;
      },
    );
  });
});

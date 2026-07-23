import * as assert from 'node:assert';
import * as path from 'node:path';

import { suite, test } from 'mocha';

import {
  getRelativePathForDocument,
  normalizeWorkspaceRelativePath,
  tryGetRelativeFilePath,
} from '../utils/file';

suite('Workspace-relative path resolution', () => {
  test('normalizes POSIX paths to forward slashes', () => {
    assert.strictEqual(normalizeWorkspaceRelativePath('src/a.ts'), 'src/a.ts');
    assert.strictEqual(normalizeWorkspaceRelativePath(`src${path.sep}a.ts`), 'src/a.ts');
  });

  test('resolves files inside a POSIX workspace root', () => {
    assert.strictEqual(
      tryGetRelativeFilePath('/tmp/workspace', '/tmp/workspace/src/a.ts'),
      'src/a.ts',
    );
  });

  test('resolves Windows-style paths with mixed separators', function () {
    if (process.platform !== 'win32') {
      this.skip();
    }

    assert.strictEqual(
      tryGetRelativeFilePath('C:\\repo\\project', 'C:/repo/project/src/a.ts'),
      'src/a.ts',
    );
  });

  test('rejects sibling workspace prefixes', () => {
    assert.strictEqual(
      tryGetRelativeFilePath('/root/project', '/root/project-old/src/a.ts'),
      undefined,
    );
  });

  test('rejects files outside the configured workspace root', () => {
    assert.strictEqual(
      tryGetRelativeFilePath('/tmp/workspace', '/tmp/other/src/a.ts'),
      undefined,
    );
  });

  test('getRelativePathForDocument returns undefined for non-file schemes', () => {
    const document = {
      uri: {
        scheme: 'untitled',
        fsPath: '/tmp/workspace/src/a.ts',
      },
    } as import('vscode').TextDocument;

    assert.strictEqual(getRelativePathForDocument(document, '/tmp/workspace'), undefined);
  });

  test('getRelativePathForDocument resolves nested workspace roots', () => {
    const document = {
      uri: {
        scheme: 'file',
        fsPath: '/tmp/workspace/packages/app/src/main.ts',
      },
    } as import('vscode').TextDocument;

    assert.strictEqual(
      getRelativePathForDocument(document, '/tmp/workspace/packages/app'),
      'src/main.ts',
    );
  });
});

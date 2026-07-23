import * as assert from 'node:assert';

import { suite, test } from 'mocha';
import * as vscode from 'vscode';

import {
  decodeQueryParameter,
  InvalidFrilVaultUriError,
  parseWorkspaceQuery,
} from '../features/uri/parse';

suite('FrilVault URI parsing', () => {
  test('decodes a valid encoded workspace path', () => {
    assert.strictEqual(
      decodeQueryParameter(encodeURIComponent('/tmp/workspace'), 'workspace'),
      '/tmp/workspace',
    );
  });

  test('decodes spaces and Unicode in workspace paths', () => {
    assert.strictEqual(
      decodeQueryParameter(encodeURIComponent('/tmp/my workspace/프로젝트'), 'workspace'),
      '/tmp/my workspace/프로젝트',
    );
  });

  test('throws a controlled error for malformed percent encoding', () => {
    assert.throws(
      () => decodeQueryParameter('%', 'workspace'),
      (error: unknown) => {
        assert.ok(error instanceof InvalidFrilVaultUriError);
        assert.match(String(error), /Invalid percent encoding/);
        return true;
      },
    );
  });

  test('throws a controlled error for incomplete percent sequences', () => {
    assert.throws(
      () => decodeQueryParameter('%2', 'workspace'),
      InvalidFrilVaultUriError,
    );
  });

  test('parseWorkspaceQuery returns undefined when workspace is missing', () => {
    const uri = vscode.Uri.parse('frilvault://note/v1/note-1');

    assert.strictEqual(parseWorkspaceQuery(uri), undefined);
  });

  test('parseWorkspaceQuery decodes the workspace parameter once', () => {
    const uri = vscode.Uri.parse(
      `frilvault://note/v1/note-1?workspace=${encodeURIComponent('/tmp/workspace')}`,
    );

    assert.strictEqual(parseWorkspaceQuery(uri), '/tmp/workspace');
  });
});

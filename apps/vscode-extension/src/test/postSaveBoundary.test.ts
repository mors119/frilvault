import * as assert from 'node:assert';
import * as fs from 'node:fs';
import * as path from 'node:path';

import { suite, test } from 'mocha';

import { COMMAND_IDS } from '../constants/ids';
import { maybePromptForGitignore } from '../features/gitignore/prompt';
import { runOptionalPostSaveTasks } from '../features/post-save/tasks';

suite('Post-save boundaries', () => {
  test('maybePromptForGitignore reports CLI inspection failures without throwing', async () => {
    let warning = '';

    await maybePromptForGitignore({
      getWorkspaceRoot: () => '/tmp/workspace',
      cliClient: {
        checkGitignore: async () => {
          throw new Error('cli unavailable');
        },
      } as never,
      workspaceState: createWorkspaceState(),
      showWarningMessage: async (message, _options) => {
        warning = message;
        return undefined;
      },
    });

    assert.match(warning, /could not inspect \.gitignore/i);
  });

  test('runOptionalPostSaveTasks reports gitignore inspection failures without throwing', async () => {
    let warning = '';

    await runOptionalPostSaveTasks({
      getWorkspaceRoot: () => '/tmp/workspace',
      cliClient: {
        checkGitignore: async () => {
          throw new Error('gitignore prompt failed');
        },
      } as never,
      workspaceState: createWorkspaceState(),
      showWarningMessage: async (message, _options) => {
        warning = message;
        return undefined;
      },
    });

    assert.match(warning, /could not inspect \.gitignore/i);
  });
});

suite('Canonical add note command registration', () => {
  test('package.json exposes frilvault.addNote without createNoteHere', () => {
    const packageJsonPath = path.join(__dirname, '..', '..', 'package.json');
    const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8')) as {
      activationEvents?: string[];
      contributes?: {
        commands?: Array<{ command: string }>;
      };
    };

    const commands = packageJson.contributes?.commands?.map((entry) => entry.command) ?? [];

    assert.ok(commands.includes(COMMAND_IDS.addNote));
    assert.ok(!commands.includes('frilvault.createNoteHere'));
    assert.ok(packageJson.activationEvents?.includes(`onCommand:${COMMAND_IDS.addNote}`));
    assert.ok(!packageJson.activationEvents?.includes('onCommand:frilvault.createNoteHere'));
  });
});

function createWorkspaceState(): import('vscode').Memento {
  const values = new Map<string, unknown>();

  return {
    keys: () => [...values.keys()],
    get: <T>(key: string) => values.get(key) as T | undefined,
    update: async (key: string, value: unknown) => {
      values.set(key, value);
    },
  } as import('vscode').Memento;
}

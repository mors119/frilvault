import * as path from 'node:path';
import { execFile } from 'node:child_process';
import { promisify } from 'node:util';

import type { NoteView } from '../../types';

const execFileAsync = promisify(execFile);

export interface NotesPanelServiceInput {
  cliPath: string;
  workspaceRoot: string;
  sourceFile: string;
}

export class NotesPanelService {
  public async listNotes(input: NotesPanelServiceInput): Promise<NoteView[]> {
    const relativeFilePath = path.relative(input.workspaceRoot, input.sourceFile);

    if (
      relativeFilePath.length === 0 ||
      relativeFilePath.startsWith('..') ||
      path.isAbsolute(relativeFilePath)
    ) {
      throw new Error('The active file must be inside the current workspace.');
    }

    const { stdout } = await execFileAsync(
      input.cliPath,
      ['list', '--file', relativeFilePath, '--format', 'json'],
      { cwd: input.workspaceRoot },
    );

    return JSON.parse(stdout) as NoteView[];
  }
}

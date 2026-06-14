import * as path from 'node:path';

import { executeAddNoteCli } from './cli';

export interface AddNoteRequest {
  cliPath: string;
  workspaceRoot: string;
  sourceFile: string;
  line: number;
  column: number;
  content: string;
}

export class AddNoteService {
  public async execute(request: AddNoteRequest): Promise<void> {
    const relativeFilePath = path.relative(request.workspaceRoot, request.sourceFile);

    if (
      relativeFilePath.length === 0 ||
      relativeFilePath.startsWith('..') ||
      path.isAbsolute(relativeFilePath)
    ) {
      throw new Error('The active file must be inside the current workspace.');
    }

    await executeAddNoteCli({
      cliPath: request.cliPath,
      workspaceRoot: request.workspaceRoot,
      relativeFilePath,
      line: request.line,
      column: request.column,
      content: request.content,
    });
  }
}

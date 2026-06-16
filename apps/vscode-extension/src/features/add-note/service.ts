import type { CliClient } from '../../core/cliClient';
import { getRelativeFilePath } from '../../utils/file';

export interface AddNoteRequest {
  workspaceRoot: string;
  sourceFile: string;
  line: number;
  column: number;
  content: string;
}

export class AddNoteService {
  public constructor(private readonly cliClient: CliClient) {}

  public async execute(request: AddNoteRequest): Promise<void> {
    await this.cliClient.addLineNote({
      workspaceRoot: request.workspaceRoot,
      sourceFile: getRelativeFilePath(request.workspaceRoot, request.sourceFile),
      line: request.line,
      column: request.column,
      content: request.content,
    });
  }
}

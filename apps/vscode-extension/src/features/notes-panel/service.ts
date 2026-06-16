import type { CliClient } from '../../core/cliClient';
import type { NoteView } from '../../types';

export class NotesPanelService {
  public constructor(private readonly cliClient: CliClient) {}

  public listNotes(workspaceRoot: string, sourceFile: string): Promise<NoteView[]> {
    return this.cliClient.listNotes(workspaceRoot, sourceFile);
  }
}

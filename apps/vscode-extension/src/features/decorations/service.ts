import type { NoteView } from '../../types';
import { NotesPanelService } from '../notes-panel/service';
import { getConfiguredCliPath } from '../add-note/cli';

export interface DecorationsServiceInput {
  workspaceRoot: string;
  sourceFile: string;
}

export class DecorationsService {
  private readonly notesPanelService = new NotesPanelService();

  public async listNotes(input: DecorationsServiceInput): Promise<NoteView[]> {
    return this.notesPanelService.listNotes({
      cliPath: getConfiguredCliPath(),
      workspaceRoot: input.workspaceRoot,
      sourceFile: input.sourceFile,
    });
  }
}

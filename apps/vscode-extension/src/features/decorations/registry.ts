import type { NoteView } from '../../types';

export class GutterNoteRegistry {
  private readonly byEditor = new Map<string, Map<number, NoteView[]>>();

  public set(editorUri: string, lineNotes: Map<number, NoteView[]>): void {
    this.byEditor.set(editorUri, lineNotes);
  }

  public get(editorUri: string, line: number): NoteView[] {
    return this.byEditor.get(editorUri)?.get(line) ?? [];
  }

  public findNote(
    editorUri: string,
    noteId: string,
    sourceFile: string,
  ): NoteView | undefined {
    const lines = this.byEditor.get(editorUri);

    if (!lines) {
      return undefined;
    }

    for (const notes of lines.values()) {
      const match = notes.find(
        (note) => note.note.id === noteId && note.source_file === sourceFile,
      );

      if (match) {
        return match;
      }
    }

    return undefined;
  }

  public clear(editorUri?: string): void {
    if (editorUri) {
      this.byEditor.delete(editorUri);
      return;
    }

    this.byEditor.clear();
  }
}

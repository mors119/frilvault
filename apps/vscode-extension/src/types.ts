export type NoteAnchor =
  | {
      type: 'Line';
      line: number;
      column: number;
    }
  | {
      type: 'Symbol';
      name: string;
      kind: string;
      signature?: string;
      line_hint?: number;
    };

export interface FrilVaultNote {
  id: string;
  anchor: NoteAnchor;
  content: string;
  created_at: string;
  updated_at: string;
}

export interface NoteView {
  source_file: string;
  note: FrilVaultNote;
}

export interface WorkspaceStats {
  file_count: number;
  total_notes: number;
  existing_files: number;
  missing_files: number;
  line_notes: number;
  symbol_notes: number;
}

export interface WorkspaceHealth {
  missing_source_files: string[];
}

export interface RepairSuggestion {
  missing_file: string;
  candidates: string[];
}

export interface MutationResult {
  note: FrilVaultNote | null;
}

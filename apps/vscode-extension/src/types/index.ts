export type NoteAnchor = {
  type: 'Line' | 'Symbol';
  line?: number;
  column?: number;
  name?: string;
  kind?: string;
  signature?: string;
  line_hint?: number;
};

export type NoteView = {
  source_file: string;
  note: {
    id: string;
    content: string;
    anchor: NoteAnchor;
    created_at?: string;
    updated_at?: string;
  };
};

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

export interface SyncResult {
  notes_synced: boolean;
  repairs_applied: number;
}

export interface MutationResult {
  note: NoteView['note'] | null;
}

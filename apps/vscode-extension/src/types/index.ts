export type NoteAnchor = {
  type: 'Line' | 'Symbol';
  line?: number;
  column?: number;
  name?: string;
  kind?: string;
  signature?: string;
  line_hint?: number;
};

export type NoteAttachment = {
  id: string;
  filename: string;
  mime_type: string;
  extension: string;
};

export type ResolvedSymbol = {
  line: number;
  column: number;
};

export type NoteView = {
  source_file: string;
  note: {
    id: string;
    content: string;
    anchor: NoteAnchor;
    tags?: string[];
    attachments?: NoteAttachment[];
    created_at?: string;
    updated_at?: string;
  };
  resolved?: ResolvedSymbol;
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

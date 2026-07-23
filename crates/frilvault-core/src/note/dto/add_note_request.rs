use std::path::PathBuf;

use crate::note::NoteAnchor;

/// Input for creating a note through `NoteService::add_note`.
///
/// `NoteService::add_note`로 노트를 생성할 때 사용하는 입력입니다.
#[derive(Debug, Clone)]
pub struct AddNoteRequest {
    /// Workspace-relative source file path.
    ///
    /// 워크스페이스 기준 상대 source file 경로입니다.
    pub source_file: PathBuf,

    /// Anchor describing where the note belongs in the source file.
    ///
    /// source file에서 노트 위치를 설명하는 앵커입니다.
    pub anchor: NoteAnchor,

    /// Markdown note body to persist.
    ///
    /// 저장할 Markdown 본문입니다.
    pub content: String,

    /// Optional tags stored with the note.
    ///
    /// 노트와 함께 저장할 선택적 태그입니다.
    pub tags: Option<Vec<String>>,
}

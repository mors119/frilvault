use chrono::{DateTime, Utc};

/// Input for updating an existing note through `NoteService::update_note`.
///
/// `NoteService::update_note`로 기존 노트를 수정할 때 사용하는 입력입니다.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateNoteRequest {
    pub content: String,

    /// When set, replaces the note's tag list entirely.
    ///
    /// 설정되면 노트의 tag 목록을 전체 교체합니다.
    pub tags: Option<Vec<String>>,

    /// When set, update fails with `ConcurrentModification` if the stored
    /// `updated_at` value differs.
    ///
    /// 설정되면 저장된 `updated_at`과 다를 경우 `ConcurrentModification`으로
    /// 수정이 실패합니다.
    pub expected_updated_at: Option<DateTime<Utc>>,
}

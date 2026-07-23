use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{ResolvedSymbol, note::Note};

/// Read model returned to callers that need source context with a note.
///
/// `resolved` is populated for symbol notes when current source text can locate
/// the anchor; it is omitted for line notes and unresolved symbol notes.
///
/// 노트와 함께 source context가 필요한 호출자에게 반환하는 읽기 모델입니다.
///
/// `resolved`는 현재 source text에서 앵커를 찾을 수 있는 symbol note에만
/// 채워지며, line note와 해석되지 않은 symbol note에는 포함되지 않습니다.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteView {
    pub source_file: PathBuf,

    pub note: Note,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<ResolvedSymbol>,
}

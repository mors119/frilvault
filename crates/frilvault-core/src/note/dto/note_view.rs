use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{ResolvedSymbol, note::Note};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteView {
    pub source_file: PathBuf,

    pub note: Note,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<ResolvedSymbol>,
}

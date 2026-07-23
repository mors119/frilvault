use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateNoteRequest {
    pub content: String,
    pub tags: Option<Vec<String>>,
    pub expected_updated_at: Option<DateTime<Utc>>,
}

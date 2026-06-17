use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileMove {
    pub from: String,
    pub to: String,

    /// infer rename candidates using filename similarity
    pub confidence: f32,
}

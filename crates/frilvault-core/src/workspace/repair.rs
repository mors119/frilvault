#[derive(Debug, Clone)]
pub struct RepairSuggestion {
    pub missing_file: String,
    pub candidates: Vec<String>,
}

impl RepairSuggestion {
    pub fn best_candidate(&self) -> Option<&String> {
        self.candidates.first()
    }
}

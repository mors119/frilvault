use crate::workspace::FileMove;

#[derive(Debug, Clone)]
pub struct RepairSuggestion {
    pub missing_file: String,
    pub candidates: Vec<String>,
}

impl RepairSuggestion {
    pub fn best_candidate(&self) -> Option<&String> {
        self.candidates.first()
    }

    pub fn from_moves(moves: Vec<FileMove>) -> Vec<RepairSuggestion> {
        let mut suggestions = Vec::new();

        for mv in moves {
            if mv.confidence < 0.5 {
                continue;
            }

            suggestions.push(RepairSuggestion {
                missing_file: mv.from,
                candidates: vec![mv.to],
            });
        }

        suggestions
    }
}

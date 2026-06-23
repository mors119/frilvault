use crate::FrilVaultResult;
use crate::note::NoteFile;

#[derive(Debug, Default, Clone)]
pub struct JsonParser;

impl JsonParser {
    pub fn serialize(&self, note_file: &NoteFile) -> FrilVaultResult<String> {
        Ok(serde_json::to_string(note_file)?)
    }

    pub fn deserialize(&self, content: &str) -> FrilVaultResult<NoteFile> {
        Ok(serde_json::from_str(content)?)
    }
}

use crate::FrilVaultResult;
use crate::note::NoteFile;
use crate::parser::NoteParser;

#[derive(Debug, Default, Clone)]
pub struct YamlParser;

impl NoteParser for YamlParser {
    fn serialize(&self, note_file: &NoteFile) -> FrilVaultResult<String> {
        Ok(serde_yml::to_string(note_file)?)
    }

    fn deserialize(&self, content: &str) -> FrilVaultResult<NoteFile> {
        Ok(serde_yml::from_str(content)?)
    }
}

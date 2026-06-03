use crate::FrilVaultResult;
use crate::note::NoteFile;

pub trait NoteParser {
    fn serialize(&self, note_file: &NoteFile) -> FrilVaultResult<String>;

    fn deserialize(&self, content: &str) -> FrilVaultResult<NoteFile>;
}

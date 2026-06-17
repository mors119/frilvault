use crate::FrilVaultResult;
use crate::note::NoteFile;

pub trait NoteParser {
    /// Converts a NoteFile into a serialized representation.
    ///
    /// Implementations may choose YAML, JSON, or any other format.
    fn serialize(&self, note_file: &NoteFile) -> FrilVaultResult<String>;

    /// Converts serialized content back into a NoteFile.
    ///
    /// Returns an error if the content is invalid or unsupported.
    fn deserialize(&self, content: &str) -> FrilVaultResult<NoteFile>;
}

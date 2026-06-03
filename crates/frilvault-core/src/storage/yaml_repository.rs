use crate::FrilVaultResult;
use crate::constants::VAULT_DIR_NAME;
use crate::note::{Note, NoteFile};
use crate::parser::{NoteParser, YamlParser};
use crate::workspace::PathResolver;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct YamlNoteRepository {
    path_resolver: PathResolver,
    parser: YamlParser,
}

impl YamlNoteRepository {
    fn vault_root(&self) -> PathBuf {
        self.path_resolver.workspace_root().join(VAULT_DIR_NAME)
    }

    // Create a YamlNoteRepository with the given PathResolver.
    pub fn new(path_resolver: PathResolver) -> Self {
        Self {
            path_resolver,
            parser: YamlParser,
        }
    }

    // Read the existing note file, append the new note, and save it again.
    pub fn append_note(&self, note: &Note) -> FrilVaultResult<()> {
        let mut note_file = self.load_by_source_file(&note.source_file)?;

        note_file.notes.push(note.clone());

        self.save_by_source_file(&note.source_file, &note_file)?;

        Ok(())
    }

    // If the file does not exist, it returns an empty NoteFile.
    pub fn load_by_source_file(&self, source_file: &Path) -> FrilVaultResult<NoteFile> {
        let note_path = self.path_resolver.resolve_note_path(source_file);

        self.load_by_note_path(note_path)
    }

    pub fn save_by_source_file(
        &self,
        source_file: &Path,
        note_file: &NoteFile,
    ) -> FrilVaultResult<()> {
        let note_path = self.path_resolver.resolve_note_path(source_file);

        if let Some(parent) = note_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let yaml = self.parser.serialize(note_file)?;

        fs::write(note_path, yaml)?;

        Ok(())
    }

    fn load_by_note_path(&self, note_path: PathBuf) -> FrilVaultResult<NoteFile> {
        if !note_path.exists() {
            return Ok(NoteFile::default());
        }

        let content = fs::read_to_string(note_path)?;
        let note_file = self.parser.deserialize(&content)?;

        Ok(note_file)
    }

    pub fn replace_notes(&self, source_file: &Path, notes: Vec<Note>) -> FrilVaultResult<()> {
        let note_file = NoteFile { notes };

        self.save_by_source_file(source_file, &note_file)
    }

    pub fn list_all_note_files(&self) -> FrilVaultResult<Vec<NoteFile>> {
        let mut result = Vec::new();
        let vault_root = self.vault_root();

        if !vault_root.exists() {
            return Ok(result);
        }

        self.collect_note_files(&vault_root, &mut result)?;

        Ok(result)
    }

    fn collect_note_files(
        &self,
        directory: &Path,
        result: &mut Vec<NoteFile>,
    ) -> FrilVaultResult<()> {
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                self.collect_note_files(&path, result)?;
                continue;
            }

            if path.extension().and_then(|ext| ext.to_str()) != Some("yml") {
                continue;
            }

            let note_file = self.load_by_note_path(path)?;

            result.push(note_file);
        }

        Ok(())
    }
}

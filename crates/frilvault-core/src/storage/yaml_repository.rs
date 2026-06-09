use crate::note::{Note, NoteFile};
use crate::parser::{NoteParser, YamlParser};
use crate::workspace::PathResolver;
use crate::{FrilVaultResult, NoteFileRecord, NoteRepository};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct YamlNoteRepository {
    path_resolver: PathResolver,
    parser: YamlParser,
}

impl YamlNoteRepository {
    pub fn new(path_resolver: PathResolver) -> Self {
        Self {
            path_resolver,
            parser: YamlParser,
        }
    }

    pub fn append_note(&self, source_file: &Path, note: &Note) -> FrilVaultResult<()> {
        let mut note_file = self.load_by_source_file(source_file)?;

        note_file.notes.push(note.clone());

        self.save_by_source_file(source_file, &note_file)?;

        Ok(())
    }

    pub fn load_by_source_file(&self, source_file: &Path) -> FrilVaultResult<NoteFile> {
        let note_path = self.path_resolver.resolve_note_path(source_file);

        self.load_by_note_path(&note_path)
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

    pub fn replace_notes(&self, source_file: &Path, notes: Vec<Note>) -> FrilVaultResult<()> {
        let note_file = NoteFile { notes };

        self.save_by_source_file(source_file, &note_file)
    }

    pub fn list_all_note_files(&self) -> FrilVaultResult<Vec<NoteFileRecord>> {
        let notes_root = self.path_resolver.notes_root();

        if !notes_root.exists() {
            return Ok(Vec::new());
        }

        let mut records = Vec::new();

        self.collect_note_files(&notes_root, &mut records)?;

        Ok(records)
    }

    fn collect_note_files(
        &self,
        directory: &Path,
        records: &mut Vec<NoteFileRecord>,
    ) -> FrilVaultResult<()> {
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                self.collect_note_files(&path, records)?;
                continue;
            }

            if !Self::is_note_file(&path) {
                continue;
            }

            let source_file = self.path_resolver.source_file_from_note_path(&path)?;
            let note_file = self.load_by_note_path(&path)?;

            records.push(NoteFileRecord {
                source_file,
                note_file,
            });
        }

        Ok(())
    }

    fn load_by_note_path(&self, note_path: &Path) -> FrilVaultResult<NoteFile> {
        if !note_path.exists() {
            return Ok(NoteFile::default());
        }

        let content = fs::read_to_string(note_path)?;
        let note_file = self.parser.deserialize(&content)?;

        Ok(note_file)
    }

    fn is_note_file(path: &Path) -> bool {
        path.extension().and_then(|extension| extension.to_str())
            == Some(crate::constants::NOTE_FILE_EXTENSION)
    }
}

impl NoteRepository for YamlNoteRepository {
    fn append_note(&self, source_file: &Path, note: &Note) -> FrilVaultResult<()> {
        YamlNoteRepository::append_note(self, source_file, note)
    }

    fn load_by_source_file(&self, source_file: &Path) -> FrilVaultResult<NoteFile> {
        YamlNoteRepository::load_by_source_file(self, source_file)
    }

    fn replace_notes(&self, source_file: &Path, notes: Vec<Note>) -> FrilVaultResult<()> {
        YamlNoteRepository::replace_notes(self, source_file, notes)
    }

    fn list_all_note_files(&self) -> FrilVaultResult<Vec<NoteFileRecord>> {
        YamlNoteRepository::list_all_note_files(self)
    }
}

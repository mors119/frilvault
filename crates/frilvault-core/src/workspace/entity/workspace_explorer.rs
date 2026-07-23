use std::collections::BTreeMap;

use serde::Serialize;

use crate::{Note, NoteAnchor};

/// Workspace-wide tree of notes grouped by directory, file, and anchor kind.
#[derive(Debug, Serialize)]
pub struct WorkspaceExplorer {
    pub root: ExplorerNode,
}

/// A node in the workspace explorer tree.
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum ExplorerNode {
    Directory {
        name: String,
        path: String,
        children: Vec<ExplorerNode>,
    },
    File {
        source_file: String,
        exists: bool,
        groups: Vec<ExplorerGroup>,
    },
}

/// Note groups under a source file.
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum ExplorerGroup {
    LineNotes { notes: Vec<Note> },
    SymbolNotes { notes: Vec<Note> },
}

struct DirectoryBuilder {
    name: String,
    path: String,
    directories: BTreeMap<String, DirectoryBuilder>,
    files: Vec<(String, ExplorerNode)>,
}

impl DirectoryBuilder {
    fn into_node(mut self) -> ExplorerNode {
        let mut children = Vec::new();

        for (_, directory) in self.directories {
            children.push(directory.into_node());
        }

        self.files.sort_by(|a, b| a.0.cmp(&b.0));
        for (_, file) in self.files {
            children.push(file);
        }

        ExplorerNode::Directory {
            name: self.name,
            path: self.path,
            children,
        }
    }
}

pub fn build_workspace_explorer(
    file_entries: impl IntoIterator<Item = (String, bool, Vec<Note>)>,
) -> WorkspaceExplorer {
    let mut root = DirectoryBuilder {
        name: String::new(),
        path: String::new(),
        directories: BTreeMap::new(),
        files: Vec::new(),
    };

    for (source_file, exists, notes) in file_entries {
        let file_node = build_file_node(&source_file, exists, notes);
        insert_file(&mut root, &source_file, file_node);
    }

    WorkspaceExplorer {
        root: root.into_node(),
    }
}

fn insert_file(root: &mut DirectoryBuilder, source_file: &str, file_node: ExplorerNode) {
    let parts: Vec<&str> = source_file.split('/').collect();

    if parts.is_empty() {
        return;
    }

    if parts.len() == 1 {
        root.files.push((parts[0].to_string(), file_node));
        return;
    }

    let dir_name = parts[0].to_string();
    let child_path = if root.path.is_empty() {
        dir_name.clone()
    } else {
        format!("{}/{}", root.path, dir_name)
    };

    let child = root
        .directories
        .entry(dir_name.clone())
        .or_insert_with(|| DirectoryBuilder {
            name: dir_name,
            path: child_path,
            directories: BTreeMap::new(),
            files: Vec::new(),
        });

    insert_file(child, &parts[1..].join("/"), file_node);
}

fn build_file_node(source_file: &str, exists: bool, notes: Vec<Note>) -> ExplorerNode {
    let mut line_notes = Vec::new();
    let mut symbol_notes = Vec::new();

    for note in notes {
        match note.anchor {
            NoteAnchor::Line(_) => line_notes.push(note),
            NoteAnchor::Symbol(_) => symbol_notes.push(note),
        }
    }

    line_notes.sort_by_key(|note| match &note.anchor {
        NoteAnchor::Line(anchor) => anchor.line,
        _ => 0,
    });

    symbol_notes.sort_by(|left, right| {
        let left_name = match &left.anchor {
            NoteAnchor::Symbol(anchor) => anchor.name.as_str(),
            _ => "",
        };
        let right_name = match &right.anchor {
            NoteAnchor::Symbol(anchor) => anchor.name.as_str(),
            _ => "",
        };
        left_name.cmp(right_name)
    });

    let mut groups = Vec::new();

    if !line_notes.is_empty() {
        groups.push(ExplorerGroup::LineNotes { notes: line_notes });
    }

    if !symbol_notes.is_empty() {
        groups.push(ExplorerGroup::SymbolNotes {
            notes: symbol_notes,
        });
    }

    ExplorerNode::File {
        source_file: source_file.to_string(),
        exists,
        groups,
    }
}

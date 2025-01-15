use std::path::PathBuf;
use std::time::SystemTime;
use std::slice::{Iter, IterMut};
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
#[derive(Default)]
pub struct TreeEntry {
    pub entry: ListEntry,
    pub children: Vec<TreeEntry>,
}

impl TreeEntry {
    pub fn iter(&self) -> Iter<'_, TreeEntry> {
        self.children.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, TreeEntry> {
        self.children.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.children.len()
    }

    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    pub fn path(&self) -> &PathBuf {
        &self.entry.path
    }

    pub fn is_dir(&self) -> bool {
        self.entry.is_dir
    }

    pub fn size(&self) -> Option<u64> {
        self.entry.size
    }

    pub fn stats(&self) -> Option<&FileMetadata> {
        self.entry.stats.as_ref()
    }

    pub fn entry_type(&self) -> Option<&String> {
        self.entry.entry_type.as_ref()
    }
}

impl Index<usize> for TreeEntry {
    type Output = TreeEntry;

    fn index(&self, index: usize) -> &Self::Output {
        &self.children[index]
    }
}

impl IndexMut<usize> for TreeEntry {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.children[index]
    }
}

impl<'a> IntoIterator for &'a TreeEntry {
    type Item = &'a TreeEntry;
    type IntoIter = Iter<'a, TreeEntry>;

    fn into_iter(self) -> Self::IntoIter {
        self.children.iter()
    }
}

impl<'a> IntoIterator for &'a mut TreeEntry {
    type Item = &'a mut TreeEntry;
    type IntoIter = IterMut<'a, TreeEntry>;

    fn into_iter(self) -> Self::IntoIter {
        self.children.iter_mut()
    }
}

impl IntoIterator for TreeEntry {
    type Item = TreeEntry;
    type IntoIter = std::vec::IntoIter<TreeEntry>;

    fn into_iter(self) -> Self::IntoIter {
        self.children.into_iter()
    }
}

impl AsRef<TreeEntry> for TreeEntry {
    fn as_ref(&self) -> &TreeEntry {
        self
    }
}


// File operations types
#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub modified: SystemTime,
    pub size: u64,
    pub line_count: Option<usize>,
}

#[derive(Debug)]
pub enum WriteOperation {
    Full(String),
    Partial(PartialWrite),
    PartialLarge(PartialWriteLarge),
}

#[derive(Debug)]
pub struct PartialWrite {
    pub context_lines: usize,
    pub return_full_content: bool,
    pub changes: Vec<Change>,
}

#[derive(Debug)]
pub struct Change {
    pub old_str: String,
    pub new_str: String,
    pub allow_multiple_matches: bool,
}

#[derive(Debug)]
pub struct PartialWriteLarge {
    pub start_str: String,
    pub end_str: String,
    pub new_str: String,
    pub context_lines: usize,
}

#[derive(Debug)]
pub struct WriteResult {
    pub line_count: usize,
    pub size: usize,
    pub partial_write_result: Option<PartialWriteResult>,
    pub partial_write_large_result: Option<PartialWriteLargeResult>,
}

#[derive(Debug)]
pub struct PartialWriteResult {
    pub change_results: Vec<ChangeResult>,
    pub full_content: Option<String>,
}

#[derive(Debug)]
pub struct PartialWriteLargeResult {
    pub line_number_start: usize,
    pub line_number_end: usize,
    pub context: LargeChangeContext,
}

#[derive(Debug)]
pub struct LargeChangeContext {
    pub before_start: Vec<String>,
    pub start_content: Vec<String>,
    pub end_content: Vec<String>,
    pub after_end: Vec<String>,
}

#[derive(Debug)]
pub struct ChangeResult {
    pub partial_write_index: usize,
    pub line_number_start: usize,
    pub line_number_end: usize,
    pub context: String,
}

#[derive(Debug)]
pub struct MatchInfo {
    pub pattern_index: usize,
    pub relative_match_start: usize,
}

// Command types
#[derive(Debug, Clone)]
pub struct CommandOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

// Directory types
#[derive(Debug, Default, Clone)]
pub struct ListOptions {
    pub include_size: bool,
    pub include_modified: bool,
    pub include_type: bool,
    pub file_pattern: Option<String>,
    pub recursive: bool,
    pub count_lines: bool,
}

#[derive(Debug, Clone)]
pub struct ListEntry {
    pub path: PathBuf,
    pub size: Option<u64>,
    pub modified: Option<SystemTime>,
    pub entry_type: Option<String>,
    pub is_dir: bool,
    pub symlink: bool,
    pub stats: Option<FileMetadata>,
}

impl Default for ListEntry {
    fn default() -> Self {
        Self {
            path: PathBuf::new(),
            size: None,
            modified: None,
            entry_type: None,
            is_dir: false,
            symlink: false,
            stats: None,
        }
    }
}

// Grep types
#[derive(Debug, Clone, Default)]
pub struct GrepOptions {
    pub pattern: String,
    pub case_sensitive: bool,
    pub context_before: usize,
    pub context_after: usize,
    pub file_pattern: Option<String>,
}

#[derive(Debug, Default)]
pub struct GrepMatch {
    pub line_number: usize,
    pub context: String,
}

#[derive(Debug, Default)]
pub struct GrepFileMatch {
    pub path: PathBuf,
    pub matches: Vec<GrepMatch>,
}
use std::path::PathBuf;
use std::slice::{Iter, IterMut};
use std::ops::{Index, IndexMut};
use super::list::ListEntry;
use crate::types::FileMetadata;

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

    pub fn entry_type(&self) -> &str {
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
use std::fs;
use std::io;
use std::path::Path;
use crate::types::ListOptions;

pub fn list_directory(
    path: impl AsRef<Path>,
    options: ListOptions,
) -> io::Result<Vec<fs::DirEntry>> {
    let mut entries = Vec::new();
    
    if options.recursive {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            
            if file_type.is_dir() {
                entries.extend(list_directory(entry.path(), options.clone())?);
            } else if file_type.is_file() {
                if let Some(pattern) = &options.file_pattern {
                    let file_name = entry.file_name();
                    let file_name_str = file_name.to_string_lossy();
                    
                    if !glob::Pattern::new(pattern).unwrap().matches(&file_name_str) {
                        continue;
                    }
                }
                entries.push(entry);
            }
        }
    } else {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                if let Some(pattern) = &options.file_pattern {
                    let file_name = entry.file_name();
                    let file_name_str = file_name.to_string_lossy();
                    
                    if !glob::Pattern::new(pattern).unwrap().matches(&file_name_str) {
                        continue;
                    }
                }
                entries.push(entry);
            }
        }
    }
    
    Ok(entries)
}
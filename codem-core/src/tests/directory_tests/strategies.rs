use proptest::prelude::*;
use std::path::PathBuf;

pub fn file_content_strategy() -> impl Strategy<Value = (String, usize)> {
    proptest::collection::vec("[a-zA-Z0-9 ]{1,50}", 1..20)
        .prop_map(|lines| {
            let content = lines.join("\n");
            (content, lines.len())
        })
}

pub fn dir_structure_strategy() -> impl Strategy<Value = Vec<(PathBuf, Option<String>)>> {
    let dir_count = 1..5usize;
    let file_count = 1..10usize;
    
    (dir_count, file_count).prop_flat_map(|(dir_count, file_count)| {
        let dirs = proptest::collection::vec("[a-zA-Z0-9]{1,10}", dir_count);
        let files = proptest::collection::vec("[a-zA-Z0-9]{1,10}", file_count);
        let content_strategy = file_content_strategy()
            .prop_map(|(content, _)| content);
        (dirs, files, content_strategy)
    }).prop_map(|(dirs, files, content)| {
        let mut paths = Vec::new();
        for dir in dirs {
            paths.push((PathBuf::from(&dir), None));
        }
        for file in files {
            paths.push((
                PathBuf::from(format!("{}.txt", file)),
                Some(content.clone()),
            ));
        }
        paths
    })
}
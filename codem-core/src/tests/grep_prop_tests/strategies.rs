use proptest::prelude::*;
use std::path::PathBuf;

pub fn random_text_strategy() -> impl Strategy<Value = String> {
    proptest::collection::vec(any::<char>(), 1..50).prop_map(|chars| chars.into_iter().collect())
}

pub fn file_content_strategy() -> impl Strategy<Value = String> {
    proptest::collection::vec(random_text_strategy(), 1..10)
        .prop_map(|lines| lines.join("\n"))
}

pub fn codebase_strategy() -> impl Strategy<Value = Vec<(PathBuf, String)>> {
    let file_count = 1..10usize;
    file_count.prop_flat_map(|count| {
        let file_contents = file_content_strategy();
        proptest::collection::vec((any::<String>(), file_contents), count)
            .prop_map(|files| {
                files.into_iter()
                    .map(|(name, content)| {
                        let path = PathBuf::from(format!("{}.txt", name));
                        (path, content)
                    })
                    .collect()
            })
    })
}
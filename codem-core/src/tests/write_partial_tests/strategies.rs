use proptest::prelude::*;
use crate::types::{PartialWrite, WriteOperation, Change};

pub fn content_strategy() -> impl Strategy<Value = String> {
    let line_length = 1..50usize;
    line_length
        .prop_flat_map(|len| proptest::collection::vec("[a-zA-Z0-9 ]{1,10}", len))
        .prop_map(|lines| lines.join("\n") + "\n")
}

pub fn partial_write_strategy() -> impl Strategy<Value = (String, WriteOperation)> {
    let line_length = 1..5usize;
    line_length
        .prop_flat_map(|len| {
            let content = proptest::collection::vec("[a-zA-Z0-9 ]{1,10}", len)
                .prop_map(|lines| lines.join("\n") + "\n");
            let pattern = proptest::collection::vec("[a-zA-Z0-9 ]{1,10}", 1)
                .prop_map(|lines| lines.join("\n") + "\n");
            (content, pattern)
        })
        .prop_map(|(content, pattern)| {
            let operation = WriteOperation::Partial(PartialWrite {
                context_lines: 1,
                return_full_content: true,
                changes: vec![Change {
                    old_str: pattern.clone(),
                    new_str: "replacement\n".to_string(),
                    allow_multiple_matches: false,
                }],
            });
            (content + &pattern, operation)
        })
}
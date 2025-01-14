use crate::{
    fs_ops::*,
    types::{PartialWrite, PartialWriteInner, WriteOperation},
};
use rstest::rstest;
use std::fs;
use tempfile::TempDir;
use tokio_test::block_on;

#[test]
fn test_read_file() -> anyhow::Result<()> {
    block_on(async {
        let temp = TempDir::new()?;
        let test_file = temp.path().join("test.txt");
        fs::write(&test_file, "test content")?;

        let (content, metadata) = read_file(&test_file, ReadOptions::default()).await?;
        assert_eq!(content, "test content");
        assert_eq!(metadata.size, "test content".len() as u64);

        Ok(())
    })
}

#[test]
fn test_read_with_line_count() -> anyhow::Result<()> {
    block_on(async {
        let temp = TempDir::new()?;
        let test_file = temp.path().join("test.txt");
        fs::write(&test_file, "line 1\nline 2\nline 3")?;

        let (content, metadata) = read_file(&test_file, ReadOptions { count_lines: true }).await?;

        assert_eq!(content, "line 1\nline 2\nline 3");
        assert_eq!(metadata.line_count, Some(3));

        Ok(())
    })
}

#[rstest]
#[case("new content")]
#[case("multiple\nlines\nof\ncontent")]
#[case("")] // Empty content
fn test_write_file_full(#[case] content: &str) {
    block_on(async {
        let temp = TempDir::new().unwrap();
        let test_file = temp.path().join("test.txt");

        let write_op = WriteOperation::Full(content.to_string());
        let result = write_file(&test_file, write_op, None).await.unwrap();
        
        assert_eq!(result.size, content.len());
        assert_eq!(result.line_count, content.lines().count());
        assert!(result.partial_write_result.is_none());

        let written_content = fs::read_to_string(&test_file).unwrap();
        assert_eq!(written_content, content);
    })
}

#[rstest]
#[case("original text", "text", "replaced", 1)]
#[case("text text text", "text", "word", 3)]
#[case("line1\nline2\nline3", "line", "replaced", 3)]
#[case("no matches here", "nonexistent", "replaced", 0)]
fn test_partial_write_matches(
    #[case] initial: &str,
    #[case] old_str: &str,
    #[case] new_str: &str,
    #[case] expected_matches: usize,
) {
    block_on(async {
        let temp = TempDir::new().unwrap();
        let test_file = temp.path().join("test.txt");
        fs::write(&test_file, initial).unwrap();

        let write_op = WriteOperation::Partial(PartialWrite {
            context_lines: 1,
            return_full_content: true,
            writes: vec![PartialWriteInner {
                old_str: old_str.to_string(),
                new_str: new_str.to_string(),
                allow_multiple_matches: true,
            }],
        });

        let result = write_file(&test_file, write_op, None).await.unwrap();
        
        if let Some(partial_result) = result.partial_write_result {
            assert_eq!(partial_result.content.len(), expected_matches);
        } else {
            assert_eq!(expected_matches, 0);
        }
    })
}

#[rstest]
#[case("a\nb\nc\nd\ne", "c", "middle", 1, "b\nmiddle\nd")]
#[case("first\nsecond\nthird\nfourth\nfifth", "third", "replaced", 2, "first\nsecond\nreplaced\nfourth\nfifth")]
#[case("one line", "one", "changed", 0, "changed line")]
#[case("\n\ntext\n\n", "text", "replaced", 1, "\nreplaced\n")]
#[case("start\nmiddle\nend", "middle", "changed", 1, "start\nchanged\nend")]
#[case("content", "content", "replaced", 0, "replaced")]
fn test_partial_write_context(
    #[case] initial: &str,
    #[case] old_str: &str,
    #[case] new_str: &str,
    #[case] context_lines: usize,
    #[case] expected_context: &str,
) {
    block_on(async {
        let temp = TempDir::new().unwrap();
        let test_file = temp.path().join("test.txt");
        fs::write(&test_file, initial).unwrap();

        let write_op = WriteOperation::Partial(PartialWrite {
            context_lines,
            return_full_content: true,
            writes: vec![PartialWriteInner {
                old_str: old_str.to_string(),
                new_str: new_str.to_string(),
                allow_multiple_matches: false,
            }],
        });

        let result = write_file(&test_file, write_op, None).await.unwrap();
        
        if let Some(partial_result) = result.partial_write_result {
            assert_eq!(partial_result.content[0].context, expected_context);
        } else {
            panic!("Expected partial write result");
        }
    })
}

#[rstest]
#[case("A\nB X\nC\nD X\nE", "X", "Y", 1, "A\nB Y\nC", "C\nD Y\nE")]
#[case("one\nkeyword\ntwo\nthree\nkeyword\nfour", "keyword", "replaced", 1, "one\nreplaced\ntwo", "three\nreplaced\nfour")]
#[case("A X\nB X", "X", "Y", 0, "A Y", "B Y")]
#[case("a\nX\nb\nX\nc", "X", "Z", 1, "a\nZ\nb", "b\nZ\nc")]
fn test_partial_write_multiple_matches(
    #[case] initial: &str,
    #[case] old_str: &str,
    #[case] new_str: &str,
    #[case] context_lines: usize,
    #[case] expected_context_1: &str,
    #[case] expected_context_2: &str,
) {
    block_on(async {
        let temp = TempDir::new().unwrap();
        let test_file = temp.path().join("test.txt");
        fs::write(&test_file, initial).unwrap();

        let write_op = WriteOperation::Partial(PartialWrite {
            context_lines,
            return_full_content: true,
            writes: vec![PartialWriteInner {
                old_str: old_str.to_string(),
                new_str: new_str.to_string(),
                allow_multiple_matches: true,
            }],
        });

        let result = write_file(&test_file, write_op, None).await.unwrap();
        
        if let Some(partial_result) = result.partial_write_result {
            assert_eq!(partial_result.content.len(), 2, "Expected exactly two matches");
            assert_eq!(partial_result.content[0].context, expected_context_1);
            assert_eq!(partial_result.content[1].context, expected_context_2);
        } else {
            panic!("Expected partial write result");
        }
    })
}

#[rstest]
#[case("text\ntext\ntext", "text", "new\ntext")]
#[case("line", "line", "line1\nline2\nline3")]
#[case("content", "content", "")]
fn test_partial_write_multiline_replacements(
    #[case] initial: &str,
    #[case] old_str: &str,
    #[case] new_str: &str,
) {
    block_on(async {
        let temp = TempDir::new().unwrap();
        let test_file = temp.path().join("test.txt");
        fs::write(&test_file, initial).unwrap();

        let write_op = WriteOperation::Partial(PartialWrite {
            context_lines: 1,
            return_full_content: true,
            writes: vec![PartialWriteInner {
                old_str: old_str.to_string(),
                new_str: new_str.to_string(),
                allow_multiple_matches: true,
            }],
        });

        let _result = write_file(&test_file, write_op, None).await.unwrap();
    })
}

#[test]
fn test_partial_write_multiple_patterns() {
    block_on(async {
        let temp = TempDir::new().unwrap();
        let test_file = temp.path().join("test.txt");
        let initial = "hello world hello world";
        fs::write(&test_file, initial).unwrap();

        let write_op = WriteOperation::Partial(PartialWrite {
            context_lines: 1,
            return_full_content: true,
            writes: vec![
                PartialWriteInner {
                    old_str: "hello".to_string(),
                    new_str: "hi".to_string(),
                    allow_multiple_matches: true,
                },
                PartialWriteInner {
                    old_str: "world".to_string(),
                    new_str: "earth".to_string(),
                    allow_multiple_matches: true,
                },
            ],
        });

        let result = write_file(&test_file, write_op, None).await.unwrap();
        let content = fs::read_to_string(&test_file).unwrap();
        assert_eq!(content, "hi earth hi earth");
        assert!(result.partial_write_result.is_some());
    });
}
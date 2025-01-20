#[cfg(test)]
mod tests {
    use rstest::rstest;
    use std::fs;
    use std::path::Path;
    use tempfile::TempDir;
    use crate::write::write_file_small;

    #[rstest]
    fn test_write_file_small_line_range() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let test_file = temp_dir.path().join("test.txt");
        
        // Create a test file with repeated patterns
        fs::write(&test_file, "Line 1 has pattern\nLine 2 has pattern\nLine 3 has something\nLine 4 has pattern\nLine 5 has pattern")?;

        // Try to replace "pattern" with "test" only between lines 2-4
        let result = write_file_small(
            &test_file,
            &[("pattern", "test", Some((2, 4)))],
            false
        );

        // This should succeed and only modify patterns in lines 2-4
        assert!(result.is_ok(), "Write operation failed: {:?}", result);

        // Read the file contents
        let content = fs::read_to_string(&test_file)?;
        
        // The expected content should only have "pattern" replaced in lines 2-4
        let expected = "Line 1 has pattern\nLine 2 has test\nLine 3 has something\nLine 4 has test\nLine 5 has pattern";
        assert_eq!(content, expected, "File content does not match expected");

        Ok(())
    }
}
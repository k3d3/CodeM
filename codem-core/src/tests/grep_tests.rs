#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Write;
    use tempfile::NamedTempFile;
    use tempfile::TempDir;

    use crate::types::GrepOptions;
    use crate::grep::{grep_file, grep_codebase};
    use regex::Regex;

    #[tokio::test]
    async fn test_grep_file_basic() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "test line 1\ntest line 2\nother line\n").unwrap();
        let pattern = Regex::new("test").unwrap();

        let file_match = grep_file(&file, &pattern, &GrepOptions::default()).await.unwrap().unwrap();
        assert_eq!(file_match.matches.len(), 2);
        assert_eq!(file_match.matches[0].line_number, 1);
        assert_eq!(file_match.matches[1].line_number, 2);
    }

    #[tokio::test]
    async fn test_grep_file_no_match() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "other line 1\nother line 2\n").unwrap();
        let pattern = Regex::new("test").unwrap();

        let file_match = grep_file(&file, &pattern, &GrepOptions::default()).await.unwrap();
        assert!(file_match.is_none());
    }

    #[tokio::test]
    async fn test_grep_codebase() {
        let temp = TempDir::new().unwrap();
        create_test_files(&temp).unwrap();
        let pattern = Regex::new("test").unwrap();

        let file_matches = grep_codebase(
            temp.path(),
            &pattern,
            GrepOptions {
                context_before: 1,
                context_after: 1,
                ..Default::default()
            },
        ).await.unwrap();

        assert_eq!(file_matches.len(), 2);

        // Sort matches by file name
        let mut sorted_matches = file_matches;
        sorted_matches.sort_by(|a, b| a.path.file_name().unwrap().cmp(b.path.file_name().unwrap()));

        // Check matches in first file
        assert_eq!(sorted_matches[0].matches.len(), 1);
        assert_eq!(sorted_matches[0].matches[0].line_number, 1);

        // Check matches in second file
        assert_eq!(sorted_matches[1].matches.len(), 2);
        assert_eq!(sorted_matches[1].matches[0].line_number, 2);
        assert_eq!(sorted_matches[1].matches[1].line_number, 3);
    }

    fn create_test_files(temp_dir: &TempDir) -> std::io::Result<()> {
        let file1_path = temp_dir.path().join("file1.txt");
        let mut file1 = fs::File::create(file1_path)?;
        writeln!(file1, "test line 1\nother line\nfinal line")?;

        let file2_path = temp_dir.path().join("file2.txt");
        let mut file2 = fs::File::create(file2_path)?;
        writeln!(file2, "first line\ntest line 2\ntest line 3\nfinal line")?;

        Ok(())
    }
}

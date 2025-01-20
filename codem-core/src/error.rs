use error_set::error_set;

use crate::types::CommandOutput;

error_set! {
    DirectoryError = {
        #[display("Regex parsing error: {0}")]
        RegexError(regex::Error),
        #[display("IO error: {0}")]
        IoError(std::io::Error),
    };

    WriteError = {
        #[display("File already exists")]
        FileExists {
            content: String,
        },
        #[display("IO error when writing file")]
        IoError(std::io::Error),
        #[display("Aho-Corasick error")]
        AhoCorasickError(aho_corasick::BuildError),
        #[display("File has been modified externally")]
        TimestampMismatch {
            content: String,
        },
        #[display("Error: Multiple matches found for pattern at index {index}\\nPattern: \"{old_str}\"\\nFound {match_count} matches:\\n\\n{context}")]
        MultiplePatternMatches {
            index: usize,
            old_str: String,
            content: String,
            match_count: usize,
            context: String,
        },
        #[display("Error: Pattern not found at index {index}\\nPattern: \"{old_str}\"")]
        PatternNotFound {
            index: usize,
            old_str: String,
            content: String,
        },
        #[display("Start pattern not found")]
        StartPatternNotFound {
            content: String,
        },
        #[display("End pattern not found")]
        EndPatternNotFound {
            content: String,
        },
        #[display("Multiple start patterns found")]
        MultipleStartPatternsFound {
            content: String,
        },
        #[display("Multiple end patterns found")]
        MultipleEndPatternsFound {
            content: String,
        },
        #[display("End pattern appears before start pattern")]
        EndPatternBeforeStart {
            content: String,
        },
        #[display("Patterns overlap or are nested")]
        InvalidPatternPair {
            content: String,
        },
    };

    CommandError = {
        #[display("Command failed (exit code {exit_code}):\\nstdout:\\n{stdout}\\nstderr:\\n{stderr}")]
        CommandFailed {
            stdout: String,
            stderr: String,
            exit_code: i32,
        },
        #[display("Command timed out after {timeout_ms}ms.\\nstdout:\\n{stdout}\\nstderr:\\n{stderr}")]
        Timeout {
            timeout_ms: u64,
            stdout: String,
            stderr: String,
            output: CommandOutput,
        },
        #[display("IO error when running command: {0}")]
        IoError(std::io::Error),
    };
}
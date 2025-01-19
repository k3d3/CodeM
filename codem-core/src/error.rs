use error_set::error_set;

use crate::types::CommandOutput;

error_set! {
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
        #[display("Multiple pattern matches found when allow_multiple_matches is false")]
        MultiplePatternMatches {
            index: usize,
            content: String,
            matches: Vec<(usize, String)>, // (line number, matched text)
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
        #[display("Command failed")]
        CommandFailed {
            stdout: String,
            stderr: String,
            exit_code: i32,
        },
        #[display("Command timed out after {timeout_ms}ms")]
        Timeout {
            timeout_ms: u64,
            stdout: String,
            stderr: String,
            output: CommandOutput,
        },
        #[display("IO error when running command")]
        IoError(std::io::Error),
    };
}

use error_set::error_set;

use crate::types::CommandOutput;

error_set! {
    WriteError = {
        #[display("IO error when writing file")]
        IoError(std::io::Error),
        #[display("Aho-Corasick error")]
        AhoCorasickError(aho_corasick::BuildError),
        #[display("File has been modified externally")]
        TimestampMismatch,
        #[display("Multiple pattern matches found when allow_multiple_matches is false")]
        MultiplePatternMatches {
            index: usize,
        },
        #[display("Start pattern not found")]
        StartPatternNotFound,
        #[display("End pattern not found")]
        EndPatternNotFound,
        #[display("Multiple start patterns found")]
        MultipleStartPatternsFound,
        #[display("Multiple end patterns found")]
        MultipleEndPatternsFound,
        #[display("End pattern appears before start pattern")]
        EndPatternBeforeStart,
        #[display("Patterns overlap or are nested")]
        InvalidPatternPair,
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

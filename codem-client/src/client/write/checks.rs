use crate::types::{CheckOptions, WriteResult};

pub(super) fn run_checks(result: &mut WriteResult, options: &CheckOptions) {
    if options.run_check {
        result.check_output = Some("Check command output".to_string());
    }
    if options.run_lint {
        result.lint_output = Some("Lint command output".to_string());
    }
    if options.run_test {
        result.test_output = Some("Test command output".to_string());
    }
}

use codem_core::command::run_command;
use crate::{
    error::{ClientError, OperationError},
    project::Project,
    types::file_ops::WriteResultWithChecks,
};

pub(super) async fn run_checks(
    result: &mut WriteResultWithChecks,
    project: &Project,
    _path: &std::path::Path,
) -> Result<(), ClientError> {
    if let Some(check_cmd) = &project.check_command {
        let output = run_command(check_cmd, None, None)
            .await
            .map_err(|e| OperationError::CommandError(e))?;
        result.check_results.push(format!("Check results:\n{}", output.stdout));
    }

    if let Some(lint_cmd) = &project.lint_command {
        let output = run_command(lint_cmd, None, None)
            .await
            .map_err(|e| OperationError::CommandError(e))?;
        result.check_results.push(format!("Lint results:\n{}", output.stdout));
    }

    if let Some(test_cmd) = &project.test_command {
        let output = run_command(test_cmd, None, None)
            .await
            .map_err(|e| OperationError::CommandError(e))?;
        result.check_results.push(format!("Test results:\n{}", output.stdout));
    }

    Ok(())
}
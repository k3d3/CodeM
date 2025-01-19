use crate::error::ClientError;
use codem_core::{
    fs_write::{write_file, write_new_file},
    types::{WriteOperation, WriteResult},
    command::run_command
};
use std::path::Path;

pub(crate) async fn handle_operation(
    client: &crate::Client,
    session_id: &str,
    path: &Path,
    operation: WriteOperation,
    run_test: bool,
) -> Result<WriteResult, ClientError> {
    // Get session to access project
    let session = client.sessions.get_session(session_id).await?;

    // Resolve path relative to project base path
    let absolute_path = session.project.base_path.join(path);

    // Validate the path
    client.sessions.check_path(session_id, &absolute_path).await?;

    // Check timestamp/sync which now includes file content if needed
    session.get_timestamp(&absolute_path).await?;

    // Perform the write operation
    let result = write_file(&absolute_path, operation, None)
        .await
        .map_err(ClientError::WriteError)?;

    // Run test command if requested
    if run_test {
        run_test_command(&session).await?;
    }

    Ok(result)
}

pub(crate) async fn handle_new_file(
    client: &crate::Client,
    session_id: &str,
    path: &Path,
    content: &str,
    run_test: bool,
) -> Result<WriteResult, ClientError> {
    // Get session to access project
    let session = client.sessions.get_session(session_id).await?;

    // Resolve path relative to project base path
    let absolute_path = session.project.base_path.join(path);

    // Validate the path
    client.sessions.check_path(session_id, &absolute_path).await?;

    // Create the new file
    let result = write_new_file(&absolute_path, content)
        .await
        .map_err(|e| match e {
            codem_core::WriteError::FileExists { content } => ClientError::WriteError(
                codem_core::WriteError::FileExists { content }
            ),
            other => ClientError::WriteError(other)
        })?;

    // Run test command if requested
    if run_test {
        run_test_command(&session).await?
    }

    Ok(result)
}

async fn run_test_command(session: &crate::session::manager::session::Session) -> Result<(), ClientError> {
    let test_cmd = session.project.test_command.as_ref().ok_or(ClientError::TestCommandNotConfigured)?;

    let output = run_command(test_cmd, Some(&session.project.base_path), None).await
        .map_err(ClientError::CommandError)?;

    if output.exit_code != 0 {
        return Err(ClientError::TestCommandFailed {
            stdout: output.stdout, stderr: output.stderr, exit_code: output.exit_code
        });
    }

    Ok(())
}
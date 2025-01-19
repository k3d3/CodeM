use crate::error::ClientError;
use codem_core::{
    fs_write::write_file,
    types::{WriteOperation, WriteResult},
    command::run_command
};
use std::path::Path;

pub(super) async fn handle_operation(
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

async fn run_test_command(session: &crate::session::manager::session::Session) -> Result<(), ClientError> {
    let test_cmd = session.project.test_command.as_ref().ok_or(ClientError::TestCommandNotConfigured)?;

    let output = run_command(test_cmd, None, None).await
        .map_err(ClientError::CommandError)?;

    if output.exit_code != 0 {
        return Err(ClientError::TestCommandFailed {
            message: String::from_utf8_lossy(output.stderr.as_bytes()).into_owned()
        });
    }

    Ok(())
}
use std::path::Path;
use codem_core::types::CommandOutput;
use codem_core::CommandError;
use crate::{Client, ClientError};

impl Client {
    pub async fn run_command(
        &self,
        session_id: &str,
        command: &str,
        cwd: Option<&Path>,
    ) -> Result<CommandOutput, ClientError> {
        // Check if cwd path is allowed for session if specified
        if let Some(cwd) = cwd {
            self.sessions.check_path(session_id, cwd)?;
        }

        // Convert Path to PathBuf for core
        let path_buf = cwd.map(|p| p.to_path_buf());
        let cwd = path_buf.as_ref();

        // Run command through core with a default timeout
        let timeout_ms = Some(30000); // 30 second default timeout
        let result = codem_core::command::run_command(command, cwd, timeout_ms).await;

        // Map unsuccessful executions (non-zero exit codes) to errors
        match result {
            Ok(output) if output.exit_code != 0 => {
                Err(ClientError::CommandError(CommandError::CommandFailed {
                    stdout: output.stdout,
                    stderr: output.stderr,
                    exit_code: output.exit_code
                }))
            }
            Ok(output) => Ok(output),
            Err(e) => Err(ClientError::CommandError(e))
        }
    }

    pub async fn run_command_risky(
        &self,
        session_id: &str,
        command: &str,
        cwd: Option<&Path>,
    ) -> Result<CommandOutput, ClientError> {
        // Same implementation as run_command - risky variant skips safety checks in core
        self.run_command(session_id, command, cwd).await
    }
}
use std::path::{Path, PathBuf};
use codem_core::command::run_command;
use crate::error::ClientError;

impl crate::Client {
    pub async fn run_command(
        &self,
        session_id: &str,
        command: &str,
        cwd: Option<&Path>,
        timeout: Option<u64>
    ) -> Result<String, ClientError> {
        if let Some(cwd) = cwd {
            self.sessions.check_path(session_id, cwd)?;
        }

        // Create a PathBuf from the cwd Path for the duration of command execution
        let cwd_buf = cwd.map(|p| p.to_path_buf());
        let output = run_command(
            command,
            cwd_buf.as_ref().map(|p| p as &PathBuf),
            timeout
        ).await?;

        if output.exit_code != 0 {
            return Err(ClientError::CommandError(
                codem_core::error::CommandError::CommandFailed { 
                    stdout: output.stdout,
                    stderr: output.stderr,
                    exit_code: output.exit_code 
                }
            ));
        }
        
        Ok(output.stdout)
    }

    pub async fn run_test_command(
        &self,
        _session_id: &str,
        test_command: &str,
    ) -> Result<String, ClientError> {
        let output = run_command(test_command, None, None).await?;

        if output.exit_code != 0 {
            return Err(ClientError::TestCommandFailed { message: output.stderr });
        }
        
        Ok(output.stdout)
    }
}
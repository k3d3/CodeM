use std::path::Path;
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
        let session = self.sessions.get_session(session_id).await?;
        
        // cwd is project base_path
        let cwd = cwd.or(Some(session.project.base_path.as_path()));

        // Check command safety and raise if unsafe
        let is_safe = self.sessions.config().is_command_safe(command);
        if !is_safe {
            return Err(ClientError::UnsafeCommand { command: command.to_string() });
        }

        let output = run_command(
            command,
            cwd,
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
        
        // Combine stdout and stderr
        let mut combined = String::new();
        if !output.stdout.is_empty() {
            combined.push_str(&output.stdout);
        }
        if !output.stderr.is_empty() {
            if !combined.is_empty() {
                combined.push('\n');
            }
            combined.push_str(&output.stderr);
        }

        Ok(combined)
    }

    pub async fn run_command_risky(
        &self,
        session_id: &str,
        command: &str,
        cwd: Option<&Path>,
        timeout: Option<u64>
    ) -> Result<String, ClientError> {
        let session = self.sessions.get_session(session_id).await?;
        
        // cwd is project base_path
        let cwd = cwd.or(Some(session.project.base_path.as_path()));

        // No safety checks - can run any command

        let output = run_command(
            command,
            cwd,
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
        
        // Combine stdout and stderr
        let mut combined = String::new();
        if !output.stdout.is_empty() {
            combined.push_str(&output.stdout);
        }
        if !output.stderr.is_empty() {
            if !combined.is_empty() {
                combined.push('\n');
            }
            combined.push_str(&output.stderr);
        }

        Ok(combined)
    }

    pub async fn run_test_command(
        &self,
        session_id: &str,
    ) -> Result<String, ClientError> {
        // Validate session exists and get project
        let session = self.sessions.get_session(session_id).await?;

        // Test command is in project config
        let test_command = session.project.test_command
            .as_ref()
            .ok_or(ClientError::TestCommandNotConfigured)?;

        // cwd is project base_path
        let cwd = Some(session.project.base_path.as_path());

        let output = run_command(test_command, cwd, None).await?;

        if output.exit_code != 0 {
            return Err(ClientError::TestCommandFailed { stdout: output.stdout, stderr: output.stderr, exit_code: output.exit_code });
        }
        
        // Combine stdout and stderr
        let mut combined = String::new();
        if !output.stdout.is_empty() {
            combined.push_str(&output.stdout);
        }
        if !output.stderr.is_empty() {
            if !combined.is_empty() {
                combined.push('\n');
            }
            combined.push_str(&output.stderr);
        }

        Ok(combined)
    }
}
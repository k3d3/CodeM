use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Duration;
use wait_timeout::ChildExt;

use crate::types::CommandOutput;
use crate::CommandError;

pub async fn run_command(
    command: &str,
    cwd: Option<&PathBuf>,
    timeout_ms: Option<u64>,
) -> Result<CommandOutput, CommandError> {
    let mut cmd = Command::new("sh");
    cmd.args(["-c", command]);


    if let Some(cwd) = cwd {
        cmd.current_dir(cwd);
    }

    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let mut child = cmd.spawn()?;

    let output = if let Some(timeout) = timeout_ms {
        match child.wait_timeout(Duration::from_millis(timeout))? {
            Some(status) => {
                let output = child.wait_with_output()?;
                CommandOutput {
                    stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
                    stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
                    exit_code: status.code().unwrap_or(-1),
                }
            }
            None => {
                child.kill()?;
                let output = child.wait_with_output()?;
                let cmd_output = CommandOutput {
                    stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
                    stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
                    exit_code: -1,
                };
                return Err(CommandError::Timeout {
                    timeout_ms: timeout,
                    stdout: cmd_output.stdout.clone(),
                    stderr: cmd_output.stderr.clone(),
                    output: cmd_output,
                });
            }
        }
    } else {
        let output = child.wait_with_output()?;
        CommandOutput {
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
            exit_code: output.status.code().unwrap_or(-1),
        }
    };

    Ok(output)
}

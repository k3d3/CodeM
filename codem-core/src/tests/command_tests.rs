use crate::command::run_command;
use tempfile::TempDir;

#[tokio::test]
async fn test_command_timeout() -> anyhow::Result<()> {
    let result = run_command("sleep 2", None, Some(100)).await;

    assert!(matches!(
        result,
        Err(crate::error::CommandError::Timeout {
            timeout_ms: 100,
            ..
        })
    ));

    Ok(())
}

#[tokio::test]
async fn test_command_fails() -> anyhow::Result<()> {
    let result = run_command("ls nonexistent_file", None, None).await.unwrap();

    assert!(result.exit_code != 0);

    Ok(())
}

#[tokio::test]
async fn test_command_with_cwd() -> anyhow::Result<()> {
    let temp = TempDir::new()?;
    let path = temp.path().to_path_buf();

    // On Linux, pwd writes to stdout, on Windows, cd writes to stderr
    let result = run_command("pwd", Some(&path), None).await?;

    let actual = result.stdout.trim();
    let path_str = temp.path().to_string_lossy();
    let expected = path_str.trim();

    assert_eq!(actual, expected);

    Ok(())
}

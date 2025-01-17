use crate::server::Mcp;
use codem_client::config::ClientConfig;
use codem_client::Project;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_new() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    let mut project = Project::new(temp_path.to_path_buf());
    project.allowed_paths = Some(vec![temp_path.to_path_buf()]);
    let projects = vec![project];

    let session_dir = temp_path.join("session");
    fs::create_dir_all(&session_dir).unwrap();
    fs::write(
        session_dir.join("session.toml"),
        "# Codem session file\n"
    ).unwrap();

    let config = ClientConfig::new(
        projects,
        temp_path.join("session").join("session.toml"),
        vec!["^echo [a-zA-Z0-9_-]+$".to_string()],
        vec![]
    ).unwrap();

    let _mcp = Mcp::new(config);
}
use crate::server::Mcp;
use codem_client::config::ClientConfig;
use codem_client::Project;
use std::fs;
use tempfile::TempDir;

pub fn create_test_env(project_name: &str) -> (Mcp, TempDir) {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    let project_dir = temp_path.join(project_name);
    fs::create_dir_all(&project_dir).unwrap();
    let mut project = Project::new(project_dir.clone());
    project.name = project_name.to_string();
    project.allowed_paths = Some(vec![project_dir.clone()]);
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

    (Mcp::new(config), temp_dir)
}

pub fn create_test_files(temp_dir: &TempDir, project_name: &str) {
    let test_dir = temp_dir.path().join(project_name);

    fs::write(
        test_dir.join("file1.txt"),
        "Content of file 1\nLine 2\n"
    ).unwrap();
    fs::write(
        test_dir.join("file2.txt"),
        "Content of file 2\nMore content\nLine 3\n"
    ).unwrap();
}
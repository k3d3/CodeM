use crate::types::session_store::SessionStore;
use tempfile::TempDir;

#[test]
fn test_session_store_creation() {
    let store = SessionStore::new();
    assert!(store.is_empty());
}

#[test]
fn test_session_store_load_save() -> std::io::Result<()> {
    let temp = TempDir::new()?;
    let path = temp.path().join("sessions.json");

    let store = SessionStore::new();
    store.save_to_file(&path)?;

    let loaded = SessionStore::load_from_file(&path)?;
    assert!(loaded.is_empty());

    Ok(())
}

#[test]
fn test_session_store_missing_file() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("sessions.json");
    let result = SessionStore::load_from_file(&path);
    assert!(result.is_err());
}

#[test]
fn test_session_store_invalid_toml() -> std::io::Result<()> {
    let temp = TempDir::new()?;
    let path = temp.path().join("sessions.json");
    std::fs::write(&path, "invalid toml content")?;

    let result = SessionStore::load_from_file(&path);
    assert!(result.is_err());
    Ok(())
}
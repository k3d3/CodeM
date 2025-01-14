// use crate::write_file;
// use std::fs;
// use tempfile::TempDir;

// #[test]
// fn test_write_file() {
//     let dir = TempDir::new().unwrap();
//     let file = dir.path().join("test.txt");

//     let content = "test content";
//     write_file(&file, content).unwrap();

//     assert_eq!(fs::read_to_string(&file).unwrap(), content);
// }

// #[test]
// fn test_write_file_nested_dirs() {
//     let dir = TempDir::new().unwrap();
//     let file = dir.path().join("a/b/c/test.txt");

//     let content = "test content";
//     write_file(&file, content).unwrap();

//     assert_eq!(fs::read_to_string(&file).unwrap(), content);
//     assert!(file.parent().unwrap().exists());
// }

use std::fs;

const TEST_FILES_DIR: &str = r#"/home/lauglam/文档/RustCode/ehviewer-tests"#;

pub fn read_test_file(filename: &str) -> String {
    fs::read_to_string(&format!("{}/{}", TEST_FILES_DIR, filename)).unwrap()
}

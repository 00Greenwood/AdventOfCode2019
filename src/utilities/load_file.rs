use std::fs;

pub fn load_file(file_path: String) -> String {
    fs::read_to_string(file_path).unwrap()
}

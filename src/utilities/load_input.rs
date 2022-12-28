use std::fs;

pub fn load_input(input_name: &str, is_test: bool) -> String {
    let mut path = String::from("./inputs/");
    path.push_str(input_name);
    if is_test {
        path.push_str("_test");
    }
    path.push_str(".txt");
    fs::read_to_string(path).unwrap()
}

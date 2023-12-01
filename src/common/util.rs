use std::fs;

fn get_input(path: &str) -> String {
    return fs::read_to_string(path).expect("Was not able to read, does the file exist?")
}
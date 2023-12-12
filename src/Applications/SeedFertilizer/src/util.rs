pub fn get_sub_str(from: &str, to: &str, string: &str) -> Option<String> {
    let start_bytes = string.find(from).unwrap_or(0) + from.len();
    let end_bytes = string.find(to).unwrap_or(string.len());

    return Some(string[start_bytes..end_bytes].to_string());
}
pub fn build_string(prefix: &str, connector: &str, file_color: &str, name: std::borrow::Cow<str>, no_color: bool) -> String {
    let mut string = String::new();
    if no_color {
        string.push_str(prefix);
        string.push_str(connector);
        string.push_str(&name);
    } else {
        string.push_str(prefix);
        string.push_str(connector);
        string.push_str(file_color);
        string.push_str(&name);
        string.push_str("\x1b[0m");
    }
    string
}
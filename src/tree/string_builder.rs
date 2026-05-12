use std::path::Path;
use terminal_link::Link;

pub fn build_string(
    prefix: &str,
    connector: &str,
    file_color: &str,
    name: std::borrow::Cow<str>,
    no_markup: bool,
    clickable: bool,
    path: &Path,
) -> String {
    let mut string = String::new();
    if no_markup {
        string.push_str(prefix);
        string.push_str(connector);
        string.push_str(&name);
    } else {
        string.push_str(prefix);
        string.push_str(connector);
        string.push_str(file_color);
        if clickable {
            let name = name.to_string().to_owned();
            let path = path.to_str().unwrap_or("").to_owned();
            let full_path = path.to_string() + "/" + &name;
            let link = Link::new(&name, &full_path);
            string.push_str(&link.to_string());
        } else {
            string.push_str(&name);
        }
        string.push_str("\x1b[0m");
    }
    string
}

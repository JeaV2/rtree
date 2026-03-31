pub fn color_to_ansi(color: &str) -> &'static str {
    match color.to_lowercase().as_str() {
        "black" => "\x1b[90m",
        "red" => "\x1b[91m",
        "green" => "\x1b[92m",
        "yellow" => "\x1b[93m",
        "blue" => "\x1b[94m",
        "magenta" => "\x1b[95m",
        "purple" => "\x1b[95m",
        "cyan" => "\x1b[96m",
        "white" => "\x1b[97m",
        _ => "\x1b[0m", // Default to reset
    }
}

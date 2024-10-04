pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const RESET: &str = "\x1b[0m";

pub fn red(string: &str) -> String {
    format!("{}{}{}", RED, string, RESET)
}

#[warn(dead_code)]
pub fn green(string: &str) -> String {
    format!("{}{}{}", GREEN, string, RESET)
}

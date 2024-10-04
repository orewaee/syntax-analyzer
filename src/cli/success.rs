use crate::cli::colors::{GREEN, RESET};

pub fn with_message(string: &str, message: &str) {
    println!("{}success: {}\n{}{}", GREEN, message, string, RESET);
}
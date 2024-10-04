use crate::cli::colors::{RED, GREEN, RESET, red};

pub fn with_message(string: &str, index: usize, message: &str) {
    let right = &string[..index];
    let wrong = &string[index..(index + 1)];
    let other = &string[(index + 1)..];

    println!("{}{}{}{}{}{}", GREEN, right, RED, wrong, RESET, other);
    println!("{}{}", " ".repeat(index), red("^"));
    println!("{}error: {}{}", RED, message, RESET);
}

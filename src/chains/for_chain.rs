use crate::state::State;
use crate::cli::{error, success};

pub fn for_chain(string: &str) {
    println!("{}", "-".repeat(10));

    let mut state = State::S;
    let mut index = 0;
    let mut symbol: char;

    while state != State::F && state != State::E && index < string.len() {
        symbol = string.chars().nth(index).unwrap();
        match state {
            State::S => {
                if symbol == 'f' || symbol == 'F' {
                    state = State::A;
                } else {
                    state = State::E;
                    error::with_message(string, index, "maybe you wanted to start with f (for)");
                    break;
                }
            }

            State::A => {
                if symbol == 'o' || symbol == 'O' {
                    state = State::B;
                } else {
                    state = State::E;
                    error::with_message(string, index, "maybe you wanted to use o (for)");
                    break;
                }
            }

            State::B => {
                if symbol == 'r' || symbol == 'R' {
                    state = State::C;
                } else {
                    state = State::E;
                    error::with_message(string, index, "maybe you wanted to use r (for)");
                    break;
                }
            }

            State::C => {
                if symbol == ' ' {
                    state = State::F;
                } else {
                    state = State::E;
                    error::with_message(string, index, "maybe you wanted to use space (end chain)");
                    break;
                }
            }

            State::E | State::F => {
                break;
            }
        }

        index += 1;
    }

    if state == State::F {
        success::with_message(string, "chain belongs to the lang");
        println!("{}", "-".repeat(10));
        return;
    }

    if state == State::E {
        return;
    }

    error::with_message(string, string.len() - 1, "the chain does not contain a terminating char");
    println!("{}", "-".repeat(10));
}
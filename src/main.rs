use crate::chain::chain::{Chain, ForChain};

mod chains;
mod state;
mod cli;
mod chain;

fn main() {
    let string = "for ";
    let for_chain = ForChain::from(string);
    let result = for_chain.is_valid();
    match result {
        Ok(()) => println!("ok"),
        Err((index, message)) => cli::error::with_message(string, index, message)
    }
}

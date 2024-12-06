
mod cli;
mod core;

use crate::core::analyzer::analyze;

fn main() {
    let chains: Vec<&str> = vec![
        "FOR F21A3 [IAX12, 25, J, 256] := -1 TO 1 DO;",
        "FOR I := " /* +10 TO +20 BY +100 DO; */
    ];

    for chain in chains {
        match analyze(chain, ';') {
            Ok(()) => println!("yes. \"{}\" is chain", chain),
            Err(error) => cli::error::with_message(chain, error.0, error.1)
        }
    }
}

mod cli;
mod machine;
mod plugboard;

use clap::Parser;
use enigma_core::EnigmaState;

use cli::{Cli, Command};
use machine::build_machine;

fn main() {
    let cli = Cli::parse();

    let machine = build_machine(
        cli.rotors,
        cli.steps,
        cli.swap.clone(),
    );

    match cli.command {
        Command::Encrypt { input } => {
            let mut state = EnigmaState::new(cli.rotors);
            let output = machine
                .process_bytes(input.as_bytes(), &mut state)
                .expect("encryption failed");

            if cli.verbose {
                println!("state: {:?}", state);
            }

            println!("{}", String::from_utf8_lossy(&output));
        }
        Command::Decrypt { input } => {
            let mut state = EnigmaState::new(cli.rotors);
            let output = machine
                .process_bytes(input.as_bytes(), &mut state)
                .expect("decryption failed");

            if cli.verbose {
                println!("state: {:?}", state);
            }

            println!("{}", String::from_utf8_lossy(&output));
        }
    }
}

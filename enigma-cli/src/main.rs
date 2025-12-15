mod cli;
mod machine;
mod plugboard;

use clap::Parser;
use enigma_core::EnigmaState;

use cli::{Cli, Command, CommandOptions};
use machine::build_machine;

fn build_state(rotors: usize, seed: Option<u64>) -> EnigmaState {
    let mut state = EnigmaState::new(rotors);

    if let Some(seed) = seed {
        for (i, pos) in state.rotor_positions.iter_mut().enumerate() {
            *pos = ((seed >> (i * 8)) & 0xFF) as u32;
        }
    }

    state
}

fn run(opts: CommandOptions) {
    let machine = build_machine(
        opts.rotors,
        opts.steps,
        opts.swap.clone(),
    );

    let mut state = build_state(opts.rotors, opts.seed);

    let output = machine
        .process_bytes(opts.input.as_bytes(), &mut state)
        .expect("processing failed");

    if opts.verbose {
        println!("state: {:?}", state);
    }

    println!("{}", String::from_utf8_lossy(&output));
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Encrypt(opts) => run(opts),
        Command::Decrypt(opts) => run(opts),
    }
}

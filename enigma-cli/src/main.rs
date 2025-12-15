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

    let mut output = Vec::with_capacity(opts.input.len());

    for (idx, &byte) in opts.input.as_bytes().iter().enumerate() {
        if opts.trace {
            println!(
                "[{}] '{}' ({})",
                idx,
                byte as char,
                byte
            );
            println!(
                "  state before: pos={:?}, step={}",
                state.rotor_positions,
                state.step_counter
            );
        }

        let out = machine
            .process_byte(byte, &mut state)
            .expect("processing failed");

        if opts.trace {
            println!(
                "  output: '{}' ({})",
                out as char,
                out
            );
            println!(
                "  state after:  pos={:?}, step={}",
                state.rotor_positions,
                state.step_counter
            );
            println!();
        }

        output.push(out);
    }

    if !opts.trace {
        println!("{}", String::from_utf8_lossy(&output));
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Encrypt(opts) => run(opts),
        Command::Decrypt(opts) => run(opts),
    }
}

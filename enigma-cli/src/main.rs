mod cli;
mod machine;
mod plugboard;
mod alphabet;

use clap::Parser;
use enigma_core::EnigmaState;

use alphabet::{char_to_symbol, symbol_to_char};
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
        opts.rotor_mode.clone(),
        opts.reflector_mode.clone(),
        opts.seed,
    );

    let mut state = build_state(opts.rotors, opts.seed);

    let mut output = Vec::with_capacity(opts.input.len());

    for (idx, &byte) in opts.input.as_bytes().iter().enumerate() {
        let Some(symbol) = char_to_symbol(byte) else {
            continue; // ignora caratteri non validi
        };

        if opts.trace {
            println!(
                "[{}] '{}' -> {}",
                idx,
                byte as char,
                symbol
            );
            println!(
                "  state before: pos={:?}, step={}",
                state.rotor_positions,
                state.step_counter
            );
        }

        let encrypted = machine
            .process_byte(symbol, &mut state)
            .expect("processing failed");

        let output_char = symbol_to_char(encrypted);

        if opts.trace {
            println!(
                "  output: '{}' ({})",
                output_char as char,
                encrypted
            );
            println!(
                "  state after:  pos={:?}, step={}",
                state.rotor_positions,
                state.step_counter
            );
            println!();
        }

        output.push(output_char);
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

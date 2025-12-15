mod cli;
mod encoding;
mod machine;
mod plugboard;

use clap::Parser;
use enigma_core::EnigmaState;

use crate::encoding::{decode_ciphertext, encode_ciphertext};
use cli::{Cli, Command, CommandOptions};
use machine::build_machine;

/// Build initial Enigma state, optionally seeded.
fn build_state(rotors: usize, seed: Option<u64>) -> EnigmaState {
    let mut state = EnigmaState::new(rotors);

    if let Some(seed) = seed {
        for (i, pos) in state.rotor_positions.iter_mut().enumerate() {
            *pos = ((seed >> (i * 8)) & 0xFF) as u32;
        }
    }

    state
}

fn run_encrypt(opts: CommandOptions) {
    let machine = build_machine(
        opts.rotors,
        opts.steps,
        opts.swap.clone(),
        opts.rotor_mode.clone(),
        opts.reflector_mode.clone(),
        opts.seed,
    );

    let mut state = build_state(opts.rotors, opts.seed);
    let input = opts.input.as_bytes();

    let mut ciphertext = Vec::with_capacity(input.len());

    if opts.trace {
        for (i, &b) in input.iter().enumerate() {
            println!("[{}] '{}' ({})", i, b as char, b);
            println!(
                "  state before: pos={:?}, step={}",
                state.rotor_positions, state.step_counter
            );

            let out = machine
                .process_byte(b, &mut state)
                .expect("encryption failed");

            println!("  output byte: {}", out);
            println!(
                "  state after:  pos={:?}, step={}",
                state.rotor_positions, state.step_counter
            );
            println!();

            ciphertext.push(out);
        }
    } else {
        ciphertext = machine
            .process_bytes(input, &mut state)
            .expect("encryption failed");
    }

    println!("{}", encode_ciphertext(&ciphertext, &opts.encoding));
}

fn run_decrypt(opts: CommandOptions) {
    let machine = build_machine(
        opts.rotors,
        opts.steps,
        opts.swap.clone(),
        opts.rotor_mode.clone(),
        opts.reflector_mode.clone(),
        opts.seed,
    );

    let mut state = build_state(opts.rotors, opts.seed);
    let ciphertext = decode_ciphertext(&opts.input, &opts.encoding);

    let mut plaintext = Vec::with_capacity(ciphertext.len());

    if opts.trace {
        for (i, &b) in ciphertext.iter().enumerate() {
            println!("[{}] byte {}", i, b);
            println!(
                "  state before: pos={:?}, step={}",
                state.rotor_positions, state.step_counter
            );

            let out = machine
                .process_byte(b, &mut state)
                .expect("decryption failed");

            println!("  output char: '{}' ({})", out as char, out);
            println!(
                "  state after:  pos={:?}, step={}",
                state.rotor_positions, state.step_counter
            );
            println!();

            plaintext.push(out);
        }
    } else {
        plaintext = machine
            .process_bytes(&ciphertext, &mut state)
            .expect("decryption failed");
    }

    println!("{}", String::from_utf8_lossy(&plaintext));
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Encrypt(opts) => run_encrypt(opts),
        Command::Decrypt(opts) => run_decrypt(opts),
    }
}

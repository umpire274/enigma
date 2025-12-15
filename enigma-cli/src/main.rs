mod cli;
mod machine;
mod plugboard;

use clap::Parser;
use data_encoding::BASE32HEX_NOPAD;
use enigma_core::EnigmaState;

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

/// Encode ciphertext bytes as Base32 (uppercase letters + digits, no padding)
fn encode_ciphertext(bytes: &[u8]) -> String {
    BASE32HEX_NOPAD.encode(bytes)
}

/// Decode Base32 ciphertext back into raw bytes
fn decode_ciphertext(s: &str) -> Vec<u8> {
    BASE32HEX_NOPAD
        .decode(s.as_bytes())
        .expect("invalid Base32 ciphertext")
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

    let ciphertext = machine
        .process_bytes(opts.input.as_bytes(), &mut state)
        .expect("encryption failed");

    if opts.trace {
        println!("final state: {:?}", state);
    }

    println!("{}", encode_ciphertext(&ciphertext));
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

    let ciphertext = decode_ciphertext(&opts.input);

    let plaintext = machine
        .process_bytes(&ciphertext, &mut state)
        .expect("decryption failed");

    if opts.trace {
        println!("final state: {:?}", state);
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

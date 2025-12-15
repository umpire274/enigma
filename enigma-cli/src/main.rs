mod cli;
mod machine;
mod plugboard;

use clap::Parser;
use data_encoding::{BASE32HEX_NOPAD, HEXUPPER};
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

/// Encode ciphertext bytes using the selected encoding
fn encode_ciphertext(bytes: &[u8], encoding: &str) -> String {
    match encoding {
        "hex" => HEXUPPER.encode(bytes),
        "base32" => BASE32HEX_NOPAD.encode(bytes),
        _ => panic!("unsupported encoding: {}", encoding),
    }
}

/// Decode ciphertext string back into raw bytes
fn decode_ciphertext(s: &str, encoding: &str) -> Vec<u8> {
    match encoding {
        "hex" => HEXUPPER
            .decode(s.as_bytes())
            .expect("invalid HEX ciphertext"),
        "base32" => BASE32HEX_NOPAD
            .decode(s.as_bytes())
            .expect("invalid Base32 ciphertext"),
        _ => panic!("unsupported encoding: {}", encoding),
    }
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

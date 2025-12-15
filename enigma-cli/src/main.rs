use clap::{Parser, Subcommand};

use enigma_core::{
    EnigmaComponent,
    EnigmaMachine,
    EnigmaState,
    LinearStepping,
    Plugboard,
    Reflector,
    Rotor,
};

#[derive(Parser)]
#[command(name = "enigma-cli")]
#[command(about = "Demo CLI for the enigma-core library")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Encrypt a string
    Encrypt {
        input: String,
    },
    /// Decrypt a string
    Decrypt {
        input: String,
    },
}

fn build_machine(rotor_count: usize) -> EnigmaMachine {
    let plugboard = Box::new(Plugboard::identity());

    let mut rotors: Vec<Box<dyn EnigmaComponent>> = Vec::new();
    for i in 0..rotor_count {
        rotors.push(Box::new(Rotor::identity(i)));
    }

    let reflector = Box::new(Reflector::identity());
    let stepping = Box::new(LinearStepping::new(256));

    EnigmaMachine::new(
        plugboard,
        rotors,
        reflector,
        stepping,
    )
        .expect("invalid Enigma configuration")
}

fn main() {
    let cli = Cli::parse();

    let machine = build_machine(1);

    match cli.command {
        Command::Encrypt { input } => {
            let mut state = EnigmaState::new(1);
            let output = machine
                .process_bytes(input.as_bytes(), &mut state)
                .expect("encryption failed");

            println!("{}", String::from_utf8_lossy(&output));
        }
        Command::Decrypt { input } => {
            let mut state = EnigmaState::new(1);
            let output = machine
                .process_bytes(input.as_bytes(), &mut state)
                .expect("decryption failed");

            println!("{}", String::from_utf8_lossy(&output));
        }
    }
}

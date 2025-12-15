use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "enigma-cli")]
#[command(about = "Demo CLI for the enigma-core library")]
pub struct Cli {
    /// Number of rotors
    #[arg(long, default_value_t = 1)]
    pub rotors: usize,

    /// Stepping modulus
    #[arg(long, default_value_t = 256)]
    pub steps: u32,

    /// Simple plugboard swap (format: A:B as byte values)
    #[arg(long)]
    pub swap: Option<String>,

    /// Verbose output
    #[arg(long)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Encrypt a string
    Encrypt {
        input: String,
    },
    /// Decrypt a string
    Decrypt {
        input: String,
    },
}

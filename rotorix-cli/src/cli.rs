use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rotorix-cli")]
#[command(about = "Demo CLI for the rotorix-core library")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Encrypt a string
    Encrypt(CommandOptions),

    /// Decrypt a string
    Decrypt(CommandOptions),
}

#[derive(Parser)]
pub struct CommandOptions {
    /// Input string
    pub input: String,

    /// Number of rotors
    #[arg(long, default_value_t = 1)]
    pub rotors: usize,

    /// Rotor mode: identity | shifted | seed
    #[arg(long, default_value = "identity")]
    pub rotor_mode: String,

    /// Reflector mode: identity or paired
    #[arg(long, default_value = "identity")]
    pub reflector_mode: String,

    /// Stepping modulus
    #[arg(long, default_value_t = 256)]
    pub steps: u32,

    /// Seed for deterministic initial rotor positions
    #[arg(long)]
    pub seed: Option<u64>,

    /// Simple plugboard swap (format: A:B as byte values)
    #[arg(long)]
    pub swap: Option<String>,

    /// Verbose output
    #[arg(long)]
    pub verbose: bool,

    /// Trace each byte through the Enigma pipeline
    #[arg(long)]
    pub trace: bool,

    /// Output encoding: base32, hex, or base64
    #[arg(long, default_value = "base32")]
    pub encoding: String,
}

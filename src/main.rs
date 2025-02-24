/// Module for handling command-line interface (CLI) interactions.
///
/// This module provides functions for getting user input and displaying output.
mod cli;

/// Module containing the implementation of the Enigma machine.
///
/// This module defines the `EnigmaMachine` struct and its associated methods for encryption
/// and decryption.
mod enigma;

use enigma::enigma::EnigmaMachine;

/// Main entry point of the program.
///
/// This program simulates the Enigma machine, allowing users to encrypt and decrypt messages.
/// It loads the Enigma configuration from a JSON file, prompts the user for a message,
/// processes the message using the Enigma machine, and displays the result.
///
/// # Steps
/// 1. Load the Enigma machine configuration from a JSON file.
/// 2. Prompt the user for a message to encrypt or decrypt.
/// 3. Process the message using the Enigma machine.
/// 4. Display the encrypted or decrypted result to the user.
///
/// # Example
/// ```bash
/// Enter the message to encrypt/decrypt: HELLO
/// Encrypted text: RFKTZ
/// ```
fn main() {
    // Load Enigma configuration from JSON
    let mut enigma =
        EnigmaMachine::from_config("config.json").expect("Error loading configuration!");

    // Prompt the user for a message to encrypt or decrypt
    let input = cli::get_user_input("Enter the message to encrypt/decrypt: ");

    // Encrypt or decrypt the message using the Enigma machine
    let encrypted_text = enigma.encrypt_message(&input);

    // Display the result to the user
    cli::display_output(&encrypted_text);
}
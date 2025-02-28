/// Module for handling command-line interface (CLI) interactions.
///
/// This module provides functions for getting user input and displaying output.
mod cli;

/// Module containing the implementation of the Enigma machine.
///
/// This module defines the `EnigmaMachine` struct and its associated methods for encryption
/// and decryption.
mod enigma;

use base64::engine::general_purpose::STANDARD as base64_engine;
use base64::Engine;
use enigma::enigma::EnigmaMachine;
use enigma::utils;
use log::info;

/// Main entry point of the program.
///
/// This program simulates the Enigma machine, allowing users to encrypt and decrypt messages.
/// It loads the Enigma configuration from a JSON file, prompts the user for a message,
/// processes the message using the Enigma machine (and AES if necessary), and displays the result.
///
/// # Steps
/// 1. Load the Enigma machine configuration from a JSON file.
/// 2. Prompt the user for a message to encrypt or decrypt.
/// 3. Ask the user if they want to encrypt or decrypt.
/// 4. Process the message using the Enigma machine and AES (if necessary).
/// 5. Display the result to the user.
///
/// # Example
/// ```bash
/// Enter the message to encrypt/decrypt: HELLO
/// Do you want to encrypt or decrypt? (e/d): e
/// AES-Encrypted message (base64): 5f4dcc3b5aa765d61d8327deb882cf99
/// ```
fn main() {
    env_logger::init();
    info!("Starting application...");

    // Ensure the config file exists
    let config_path = match utils::ensure_config_file() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Failed to create or access config file: {}", e);
            return;
        }
    };

    // Load Enigma configuration from JSON
    let mut enigma = match EnigmaMachine::from_config(&config_path) {
        Ok(machine) => machine,
        Err(e) => {
            eprintln!("Error loading configuration: {}", e);
            return;
        }
    };

    // Prompt the user for a message to encrypt or decrypt
    let input = cli::get_user_input("Enter the message to encrypt/decrypt: ");

    // Ask the user if they want to encrypt or decrypt
    let operation = cli::get_user_input("Do you want to encrypt or decrypt? (e/d): ");
    let operation = operation.trim().to_lowercase();

    let key = &utils::KEY[..]; // 32-byte key for AES-256
    let iv = &utils::IV[..]; // 16-byte IV for AES-256-CBC

    // Process the message based on the operation
    let result = match operation.as_str() {
        "e" => {
            // Encrypt the message with Enigma
            let enigma_encrypted = match enigma.encrypt_message(&input.trim().to_uppercase()) {
                Ok(text) => text,
                Err(e) => {
                    eprintln!("Error encrypting with Enigma: {}", e);
                    return;
                }
            };

            // Encrypt the Enigma output with AES
            let aes_encrypted = match utils::encrypt_aes(&enigma_encrypted, key, iv) {
                Ok(encrypted) => encrypted,
                Err(e) => {
                    eprintln!("Error encrypting with AES: {}", e);
                    return;
                }
            };

            let aes_encrypted_base64 = base64_engine.encode(&aes_encrypted);

            Ok(aes_encrypted_base64)
        }
        "d" => {
            let encrypted_message = input.trim().to_string();
            // Decode the base64-encoded AES-encrypted message
            let aes_encrypted = match base64_engine.decode(&encrypted_message) {
                Ok(data) => data,
                Err(e) => {
                    eprintln!("Error decoding base64 message: {}", e);
                    return;
                }
            };

            let aes_decrypted = match utils::decrypt_aes(&aes_encrypted, key, iv) {
                Ok(decrypted) => decrypted,
                Err(e) => {
                    eprintln!("Error decrypting with AES: {}", e);
                    return;
                }
            };

            // Decrypt the Enigma message
            enigma.encrypt_message(&String::from_utf8_lossy(&aes_decrypted))
        }
        _ => {
            eprintln!("Invalid input. Please enter 'e' for encrypt or 'd' for decrypt.");
            return;
        }
    };

    // Display the result to the user
    cli::display_output(&result);

    info!("Application ended.");
}

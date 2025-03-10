/// Module for handling command-line interface (CLI) interactions.
mod cli;

/// Module containing the implementation of the Enigma machine.
mod enigma;

use crate::cli::{postprocess_output, preprocess_input};
use crate::enigma::utils::collect_pre_message;
use base64::engine::general_purpose::STANDARD as base64_engine;
use base64::Engine;
use chrono::prelude::*;
use enigma::enigma::EnigmaMachine;
use enigma::utils;
use log::{debug, info};
use std::fs;

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
    let config_str = match fs::read_to_string(&config_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read config file: {}", e);
            return;
        }
    };

    // Parse the JSON configuration
    let config: utils::Config = match serde_json::from_str(&config_str) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to parse config file: {}", e);
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
    let result: Result<String, &'static str> = match operation.as_str() {
        "e" => {
            // Create the Enigma machine using the configuration from the JSON file
            let mut enigma = match EnigmaMachine::new_from_params(
                config.sstk,                                // Seed from JSON
                config.n_rt,                                // Number of rotors from JSON
                &Local::now().format("%Y%m%d").to_string(), // Today's date
                config.plugboard_pairs,                     // Plugboard pairs from JSON
            ) {
                Ok(machine) => machine,
                Err(e) => {
                    eprintln!("Error creating Enigma machine: {}", e);
                    return;
                }
            };

            let pre_message = collect_pre_message(&*enigma.vec_plug);
            // Encrypt the Enigma output with AES
            let aes_encrypted_pre_message = match utils::encrypt_aes(&pre_message, key, iv) {
                Ok(encrypted) => encrypted,
                Err(e) => {
                    eprintln!("Error encrypting with AES: {}", e);
                    return;
                }
            };
            debug!(
                "AES encrypted pre-message: {:?}",
                &aes_encrypted_pre_message
            );
            let aes_encrypted_base64_pre_message = base64_engine.encode(&aes_encrypted_pre_message);
            debug!(
                "AES encrypted base64 pre-message: {:?}",
                &aes_encrypted_base64_pre_message
            );

            let input = preprocess_input(input.trim());
            debug!("preprocessed input: {:?}", input);

            // Encrypt the message with Enigma
            let enigma_encrypted = match enigma.encrypt_message(&input.to_uppercase()) {
                Ok(text) => text,
                Err(e) => {
                    eprintln!("Error encrypting with Enigma: {}", e);
                    return;
                }
            };
            debug!("Enigma encrypted: {:?}", &enigma_encrypted);

            let message = aes_encrypted_base64_pre_message.to_string() + "|" + &enigma_encrypted;
            debug!("Enigma message: {:?}", &message);

            // Encrypt the Enigma output with AES
            let aes_encrypted = match utils::encrypt_aes(&message, key, iv) {
                Ok(encrypted) => encrypted,
                Err(e) => {
                    eprintln!("Error encrypting with AES: {}", e);
                    return;
                }
            };
            debug!("AES encrypted: {:?}", &aes_encrypted);

            let aes_encrypted_base64 = base64_engine.encode(&aes_encrypted);
            debug!("AES encrypted base64: {:?}", &aes_encrypted_base64);

            Ok(postprocess_output(&aes_encrypted_base64))
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
            debug!("AES encrypted message: {:?}", &aes_encrypted);

            let aes_decrypted = match utils::decrypt_aes(&aes_encrypted, key, iv) {
                Ok(decrypted) => decrypted,
                Err(e) => {
                    eprintln!("Error decrypting with AES: {}", e);
                    return;
                }
            };
            debug!("AES decrypted: {:?}", &aes_decrypted);

            debug!(
                "Enigma text: {:?}",
                &String::from_utf8_lossy(&aes_decrypted)
            );

            let message = String::from_utf8_lossy(&aes_decrypted);
            // Splitta la stringa utilizzando "|" come separatore
            let parts: Vec<&str> = message.split('|').collect();

            let pre_message = parts[0]; // Prima parte: "cMhAKbYccysBwGJU1TLGLskBK0xBC52C3YPe5IE="
            let enigma_message = parts[1]; // Seconda parte: "VOBB-ETOI-IJDO"

            debug!("pre_message: {}", pre_message);
            debug!("enigma message: {}", enigma_message);

            let aes_pre_decrypted_base64 = match base64_engine.decode(&pre_message) {
                Ok(data) => data,
                Err(e) => {
                    eprintln!("Error decoding base64 message: {}", e);
                    return;
                }
            };
            debug!(
                "AES Base64 decrypted pre-message: {:?}",
                &aes_pre_decrypted_base64
            );

            let aes_pre_decrypted = match utils::decrypt_aes(&aes_pre_decrypted_base64, key, iv) {
                Ok(decrypted) => decrypted,
                Err(e) => {
                    eprintln!("Error decrypting with AES: {}", e);
                    return;
                }
            };
            debug!(
                "AES decrypted pre-message: {:?}",
                &String::from_utf8_lossy(&aes_pre_decrypted)
            );
            let premessage = format!("{}", &String::from_utf8_lossy(&aes_pre_decrypted));

            let premex: Vec<&str> = premessage.split('|').collect();
            let data = premex[0];
            let pairs = premex[1];

            // Step 1: Split the string into pairs of characters
            let plugboard_pairs: Vec<(char, char)> = pairs
                .chars() // Convert the string into an iterator of characters
                .collect::<Vec<char>>() // Collect the characters into a Vec<char>
                .chunks(2) // Split the Vec<char> into chunks of 2 characters
                .map(|chunk| (chunk[0], chunk[1])) // Convert each chunk into a tuple (char, char)
                .collect(); // Collect all tuples into a Vec<(char, char)>

            // Create the Enigma machine using the configuration from the JSON file
            let mut enigma = match EnigmaMachine::new_from_params(
                config.sstk,                                // Seed from JSON
                config.n_rt,                                // Number of rotors from JSON
                data, // Today's date
                plugboard_pairs,                     // Plugboard pairs from JSON
            ) {
                Ok(machine) => machine,
                Err(e) => {
                    eprintln!("Error creating Enigma machine: {}", e);
                    return;
                }
            };

            //Ok("Hello".to_string())
            // Decrypt the Enigma message
            enigma.encrypt_message(enigma_message)
        }
        _ => {
            eprintln!("Invalid input. Please enter 'e' for encrypt or 'd' for decrypt.");
            return;
        }
    };

    // Display the result to the user
    let output = match result {
        Ok(output) => output,
        Err(e) => {
            eprintln!("Error output string: {}", e);
            return;
        }
    };
    cli::display_output(&output);

    info!("Application ended.");
}

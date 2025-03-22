// src/main.rs

mod cli;
mod enigma;
mod crypto;

use std::io;
use crate::cli::{display_output, get_user_input};
use crate::crypto::{decrypt_message, encrypt_message};
use crate::enigma::utils;
use log::info;

fn main() {
    env_logger::init();
    info!("Starting application...");

    // Load Enigma configuration
    let config = match utils::load_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            return;
        }
    };

    // Prompt the user for a message to encrypt or decrypt
    let input = get_user_input("Enter the message to encrypt/decrypt: ");

    // Ask the user if they want to encrypt or decrypt
    let operation = get_user_input("Do you want to encrypt or decrypt? (e/d): ");
    let operation = operation.trim().to_lowercase();

    let key = &utils::KEY[..]; // 32-byte key for AES-256
    let iv = &utils::IV[..]; // 16-byte IV for AES-256-CBC

    // Process the message based on the operation
    let result = match operation.as_str() {
        "e" => encrypt_message(&input, &config, key, iv),
        "d" => decrypt_message(&input, &config, key, iv),
        _ => {
            eprintln!("Invalid input. Please enter 'e' for encrypt or 'd' for decrypt.");
            return;
        }
    };

    // Display the result to the user
    match result {
        Ok(output) => display_output(&output, io::stdout()),
        Err(e) => eprintln!("Error: {}", e),
    }

    info!("Application ended.");
}
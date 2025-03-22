// src/crypto.rs

use crate::cli;
use crate::enigma::enigma::EnigmaMachine;
use crate::enigma::utils;
use base64::engine::general_purpose::STANDARD as base64_engine;
use base64::Engine;
use chrono::Local;

/// Encrypts a message using the Enigma machine and AES encryption.
///
/// # Arguments
/// * `input` - The message to encrypt.
/// * `config` - The configuration for the Enigma machine.
/// * `key` - The AES encryption key.
/// * `iv` - The AES initialization vector.
///
/// # Returns
/// The encrypted message as a base64-encoded string.
pub fn encrypt_message(
    input: &str,
    config: &utils::Config,
    key: &[u8],
    iv: &[u8],
) -> Result<String, String> {
    let mut enigma = match EnigmaMachine::new_from_params(
        config.sstk,
        config.n_rt,
        &Local::now().format("%Y%m%d").to_string(),
        config.plugboard_pairs.clone(),
    ) {
        Ok(machine) => machine,
        Err(e) => return Err(format!("Error creating Enigma machine: {}", e)),
    };

    let pre_message = utils::collect_pre_message(&enigma.vec_plug);
    let aes_encrypted_pre_message = match utils::encrypt_aes(&pre_message, key, iv) {
        Ok(encrypted) => encrypted,
        Err(e) => return Err(format!("Error encrypting with AES: {}", e)),
    };

    let aes_encrypted_base64_pre_message = base64_engine.encode(&aes_encrypted_pre_message);

    let input = cli::preprocess_input(input.trim());
    let enigma_encrypted = match enigma.encrypt_message(&input.to_uppercase()) {
        Ok(text) => text,
        Err(e) => return Err(format!("Error encrypting with Enigma: {}", e)),
    };

    let message = aes_encrypted_base64_pre_message.to_string() + "|" + &enigma_encrypted;
    let aes_encrypted = match utils::encrypt_aes(&message, key, iv) {
        Ok(encrypted) => encrypted,
        Err(e) => return Err(format!("Error encrypting with AES: {}", e)),
    };

    Ok(base64_engine.encode(&aes_encrypted))
}

/// Decrypts a message using the Enigma machine and AES decryption.
///
/// # Arguments
/// * `input` - The encrypted message to decrypt.
/// * `config` - The configuration for the Enigma machine.
/// * `key` - The AES decryption key.
/// * `iv` - The AES initialization vector.
///
/// # Returns
/// The decrypted message as a string.
pub fn decrypt_message(
    input: &str,
    config: &utils::Config,
    key: &[u8],
    iv: &[u8],
) -> Result<String, String> {
    let aes_encrypted = match base64_engine.decode(input.trim()) {
        Ok(data) => data,
        Err(e) => return Err(format!("Error decoding base64 message: {}", e)),
    };

    let aes_decrypted = match utils::decrypt_aes(&aes_encrypted, key, iv) {
        Ok(decrypted) => decrypted,
        Err(e) => return Err(format!("Error decrypting with AES: {}", e)),
    };

    let message = String::from_utf8_lossy(&aes_decrypted);
    let parts: Vec<&str> = message.split('|').collect();

    let pre_message = parts[0];
    let enigma_message = parts[1];

    let aes_pre_decrypted_base64 = match base64_engine.decode(pre_message) {
        Ok(data) => data,
        Err(e) => return Err(format!("Error decoding base64 message: {}", e)),
    };

    let aes_pre_decrypted = match utils::decrypt_aes(&aes_pre_decrypted_base64, key, iv) {
        Ok(decrypted) => decrypted,
        Err(e) => return Err(format!("Error decrypting with AES: {}", e)),
    };

    let premessage = String::from_utf8_lossy(&aes_pre_decrypted);
    let premex: Vec<&str> = premessage.split('|').collect();
    let data = premex[0];
    let pairs = premex[1];

    let plugboard_pairs: Vec<(char, char)> = pairs
        .chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect();

    let mut enigma =
        match EnigmaMachine::new_from_params(config.sstk, config.n_rt, data, plugboard_pairs) {
            Ok(machine) => machine,
            Err(e) => return Err(format!("Error creating Enigma machine: {}", e)),
        };

    match enigma.encrypt_message(enigma_message) {
        Ok(decrypted) => Ok(decrypted),
        Err(e) => Err(format!("Error decrypting with Enigma: {}", e)),
    }
}

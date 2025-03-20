use base64::engine::general_purpose::STANDARD as base64_engine;
use base64::Engine;
use std::io::{self, Write};
use crate::enigma::enigma::EnigmaMachine;
use crate::enigma::utils;

/// Prompts the user for input and returns the entered text.
///
/// # Arguments
/// * `prompt` - The message displayed to the user as a prompt.
///
/// # Returns
/// The user's input as a `String`, trimmed of leading and trailing whitespace.
pub fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Ensures the prompt is printed immediately

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap(); // Reads the user's input

    // Removes leading and trailing whitespace, newlines, and carriage returns
    input.trim().to_string()
}

/// Displays the result of an encryption or decryption operation.
///
/// # Arguments
/// * `output` - The output string to display.
pub fn display_output(output: &str) {
    println!("Encrypted/Decrypted text: {}", output);
}

/// Preprocesses the input by converting numbers to a prefixed sequence and keeping letters in uppercase.
///
/// # Arguments
/// * `input` - The input string to preprocess.
///
/// # Returns
/// The preprocessed string.
pub fn preprocess_input(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_digit() {
                // Convert numbers to a prefixed sequence
                let num = c.to_digit(10).unwrap() as u8;
                let letter = (b'Z' - num) as char; // Use the formula to find the corresponding letter
                format!("X{}", letter)
            } else {
                // Keep original letters in uppercase
                c.to_ascii_uppercase().to_string()
            }
        })
        .collect()
}

/// Postprocesses the output by converting prefixed sequences back to numbers.
///
/// # Arguments
/// * `output` - The output string to postprocess.
///
/// # Returns
/// The postprocessed string.
pub fn postprocess_output(output: &str) -> String {
    let mut result = String::new();
    let mut chars = output.chars().peekable();

    while let Some(c) = chars.next() {
        if c == 'X' {
            // If we find an 'X', the next character represents a number
            if let Some(next_c) = chars.next() {
                if next_c >= 'R' && next_c <= 'Z' {
                    // Use the formula to find the corresponding number
                    let num = (b'Z' - next_c as u8).to_string();
                    result.push_str(&num);
                } else {
                    // If it's not a valid letter, keep the original characters
                    result.push('X');
                    result.push(next_c);
                }
            }
        } else {
            // Keep original letters
            result.push(c);
        }
    }

    result
}

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
        &chrono::Local::now().format("%Y%m%d").to_string(),
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

    let input = preprocess_input(input.trim());
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

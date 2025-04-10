use chrono::Local;
use lazy_static::lazy_static;
use openssl::error::ErrorStack;
use openssl::symm::{Cipher, Crypter, Mode};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use log::debug;

lazy_static! {
    pub static ref KEY: [u8; 32] = *b"0123456789abcdef0123456789abcdef";
    pub static ref IV: [u8; 16] = *b"1234567890abcdef";
}

pub static FIXED_HASH: u64 = 1737;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub n_rt: usize,
    pub plugboard_pairs: Vec<(char, char)>,
    pub sstk: usize,
}

impl Config {
    pub fn load() -> Result<Self, String> {
        let path = Self::config_path()?;
        if !path.exists() {
            Self::create_default_config(&path)?;
        }

        let data = fs::read_to_string(path).map_err(|e| e.to_string())?;
        serde_json::from_str(&data).map_err(|e| e.to_string())
    }

    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_path()?;
        debug!("saving config: {}", path.display());
        let temp_path = path.with_extension("tmp");

        let config_str = serde_json::to_string_pretty(self)
            .map_err(|e| e.to_string())?;

        fs::write(&temp_path, config_str).map_err(|e| e.to_string())?;
        fs::rename(temp_path, path).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn config_path() -> Result<PathBuf, String> {
        dirs::home_dir()
            .map(|p| p.join(".enigma/config.json"))
            .ok_or("Home directory not found".to_string())
    }

    fn create_default_config(path: &PathBuf) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }

        let default_config = Config {
            n_rt: 3,
            plugboard_pairs: vec![('A', 'B'), ('C', 'D')],
            sstk: 12345,
        };

        default_config.save()
    }
}

/// Encrypts a message using AES-256-GCM.
///
/// # Arguments
/// * `message` - The message to encrypt.
/// * `key` - The AES-256 key (32 bytes).
/// * `iv` - The initialization vector (16 bytes).
///
/// # Returns
/// - `Ok(Vec<u8>)`: The encrypted message, including the authentication tag.
/// - `Err(ErrorStack)`: An error if encryption fails.
///
/// # Example
/// ```rust
/// let encrypted = encrypt_aes("Hello, world!", &KEY, &IV)?;
/// println!("Encrypted message: {:?}", encrypted);
/// ```
pub fn encrypt_aes(message: &str, key: &[u8], iv: &[u8]) -> Result<Vec<u8>, ErrorStack> {
    let cipher = Cipher::aes_256_gcm();

    // Buffer for encrypted data
    let mut encrypted = vec![0; message.len() + cipher.block_size()];

    // Create a Crypter for encryption
    let mut crypter = Crypter::new(cipher, Mode::Encrypt, key, Some(iv))?;

    // Add AAD (Additional Authenticated Data), if necessary
    crypter.aad_update(&[])?;

    // Encrypt the message
    let count = crypter.update(message.as_bytes(), &mut encrypted)?;
    let rest = crypter.finalize(&mut encrypted[count..])?;
    encrypted.truncate(count + rest);

    // Get the authentication tag
    let mut tag = vec![0; 16];
    crypter.get_tag(&mut tag)?;

    // Combine the encrypted data and the tag
    encrypted.extend_from_slice(&tag);
    Ok(encrypted)
}

/// Decrypts a message using AES-256-GCM.
///
/// # Arguments
/// * `encrypted_message` - The encrypted message, including the authentication tag.
/// * `key` - The AES-256 key (32 bytes).
/// * `iv` - The initialization vector (16 bytes).
///
/// # Returns
/// - `Ok(Vec<u8>)`: The decrypted message.
/// - `Err(ErrorStack)`: An error if decryption fails.
///
/// # Example
/// ```rust
/// let decrypted = decrypt_aes(&encrypted, &KEY, &IV)?;
/// println!("Decrypted message: {:?}", decrypted);
/// ```
pub fn decrypt_aes(encrypted_message: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, ErrorStack> {
    // Verifica che la lunghezza dei dati sia valida
    if encrypted_message.len() < 16 {
        return Err(ErrorStack::get()); // Restituisci un errore se i dati sono troppo corti
    }

    let cipher = Cipher::aes_256_gcm();

    // Separate the encrypted data and the tag
    let (encrypted_data, tag) = encrypted_message.split_at(encrypted_message.len() - 16);

    // Buffer for decrypted data
    let mut decrypted = vec![0; encrypted_data.len() + cipher.block_size()];

    // Create a Crypter for decryption
    let mut crypter = Crypter::new(cipher, Mode::Decrypt, key, Some(iv))?;

    // Add AAD (Additional Authenticated Data), if necessary
    crypter.aad_update(&[])?;

    // Decrypt the message
    let count = crypter.update(encrypted_data, &mut decrypted)?;
    crypter.set_tag(tag)?; // Set the authentication tag
    let rest = crypter.finalize(&mut decrypted[count..])?;
    decrypted.truncate(count + rest);

    Ok(decrypted)
}

/// Collects a pre-message string containing the current date and plugboard pairs.
///
/// # Arguments
/// * `plugboard_pairs` - A list of plugboard pairs.
///
/// # Returns
/// A string in the format `DATE|PLUGBOARD_PAIRS`.
///
/// # Example
/// ```rust
/// let plugboard_pairs = vec![('A', 'B'), ('C', 'D')];
/// let pre_message = collect_pre_message(&plugboard_pairs);
/// println!("Pre-message: {}", pre_message);
/// ```
pub fn collect_pre_message(plugboard_pairs: &[(char, char)]) -> String {
    let date = Local::now().format("%Y%m%d").to_string(); // Date in YYYYMMDD format
    let plugboard_chars: String = plugboard_pairs
        .iter()
        .flat_map(|(a, b)| vec![*a, *b])
        .collect(); // Example: "ABCD" for pairs [('A', 'B'), ('C', 'D')]
    format!("{}|{}", date, plugboard_chars) // Format: DATE|PLUGBOARD_PAIRS
}

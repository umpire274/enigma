use chrono::Local;
use lazy_static::lazy_static;
use openssl::error::ErrorStack;
use openssl::symm::{Cipher, Crypter, Mode};
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

lazy_static! {
    /// AES-256 key (32 bytes) used for encryption and decryption.
    pub static ref KEY: [u8; 32] = *b"0123456789abcdef0123456789abcdef";

    /// Initialization vector (IV) for AES-256-GCM (16 bytes).
    pub static ref IV: [u8; 16] = *b"1234567890abcdef";
}

/// A fixed hash value used for generating seeds in the Enigma machine.
pub static FIXED_HASH: u64 = 1737;

/// Represents the configuration for initializing an Enigma machine.
#[derive(Serialize, Deserialize)]
pub struct Config {
    /// Number of rotors in the Enigma machine.
    pub n_rt: usize,

    /// Plugboard pairs for character swapping in the Enigma machine.
    pub plugboard_pairs: Vec<(char, char)>,

    /// Seed for random generation of rotors, reflectors, and plugboard.
    pub sstk: usize,
}

/// Loads the Enigma machine configuration from the config file.
///
/// # Returns
/// - `Ok(Config)`: The configuration loaded from the file.
/// - `Err(io::Error)`: An error if the file cannot be read or parsed.
///
/// # Example
/// ```rust
/// let config = load_config()?;
/// println!("Number of rotors: {}", config.n_rt);
/// ```
pub fn load_config() -> io::Result<Config> {
    // Ensure the config file exists
    let config_path = ensure_config_file()?;

    // Read the config file
    let config_str = fs::read_to_string(&config_path)?;

    // Parse the JSON configuration
    let config: Config = serde_json::from_str(&config_str)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

    Ok(config)
}

/// Checks if the configuration file exists. If not, creates the necessary directory and file.
///
/// # Returns
/// - `Ok(String)`: The path to the configuration file.
/// - `Err(io::Error)`: An error if the directory or file cannot be created.
///
/// # Example
/// ```rust
/// let config_path = ensure_config_file()?;
/// println!("Config file path: {}", config_path);
/// ```
pub fn ensure_config_file() -> io::Result<String> {
    // Get the appropriate config directory based on the OS
    let config_dir = get_config_dir()?;
    let config_path = config_dir.join("config.json");

    // Check if the config file exists
    if !config_path.exists() {
        // Create the directory if it doesn't exist
        fs::create_dir_all(&config_dir)?;

        // Create a default configuration
        let default_config = serde_json::json!({
            "n_rt": 3, // Number of rotors
            "plugboard_pairs": [['A', 'B'], ['C', 'D']], // Plugboard pairs
            "sstk": 12345 // Seed for random generation
        });

        // Write the default configuration to the file
        let mut file = fs::File::create(&config_path)?;
        file.write_all(serde_json::to_string_pretty(&default_config)?.as_bytes())?;
    }

    Ok(config_path.to_string_lossy().into_owned())
}

/// Returns the configuration directory based on the operating system.
///
/// - On macOS/Linux: `$HOME/.enigma`
/// - On Windows: `%APPDATA%\enigma`
///
/// # Returns
/// - `Ok(PathBuf)`: The path to the configuration directory.
/// - `Err(io::Error)`: An error if the home directory cannot be determined.
///
/// # Example
/// ```rust
/// let config_dir = get_config_dir()?;
/// println!("Config directory: {:?}", config_dir);
/// ```
fn get_config_dir() -> io::Result<std::path::PathBuf> {
    let home_dir = if cfg!(target_os = "windows") {
        env::var("APPDATA").map(|path| Path::new(&path).join("enigma"))
    } else {
        env::var("HOME").map(|path| Path::new(&path).join(".enigma"))
    };

    home_dir.map_err(|_| io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))
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

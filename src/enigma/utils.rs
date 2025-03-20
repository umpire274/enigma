use openssl::error::ErrorStack;
use openssl::symm::{Cipher, Mode, Crypter};
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use chrono::Local;
use lazy_static::lazy_static;

lazy_static! {
    // Chiave AES-256 (32 byte)
    pub static ref KEY: [u8; 32] = *b"0123456789abcdef0123456789abcdef";
    // Vettore di inizializzazione (IV) per AES-256-GCM (16 byte)
    pub static ref IV: [u8; 16] = *b"1234567890abcdef";
}

pub static FIXED_HASH: u64 = 1737;

/// Represents the configuration for initializing an Enigma machine.
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub n_rt: usize,                     // Number of rotors
    pub plugboard_pairs: Vec<(char, char)>, // Plugboard pairs
    pub sstk: usize,                     // Seed for random generation
}

/// Loads the Enigma machine configuration from the config file.
///
/// # Returns
/// - `Ok(Config)`: The configuration loaded from the file.
/// - `Err(io::Error)`: An error if the file cannot be read or parsed.
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
fn get_config_dir() -> io::Result<std::path::PathBuf> {
    let home_dir = if cfg!(target_os = "windows") {
        env::var("APPDATA").map(|path| Path::new(&path).join("enigma"))
    } else {
        env::var("HOME").map(|path| Path::new(&path).join(".enigma"))
    };

    home_dir.map_err(|_| io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))
}

pub fn encrypt_aes(message: &str, key: &[u8], iv: &[u8]) -> Result<Vec<u8>, ErrorStack> {
    let cipher = Cipher::aes_256_gcm();

    // Buffer per i dati crittografati
    let mut encrypted = vec![0; message.len() + cipher.block_size()];

    // Crea un Crypter per la crittografia
    let mut crypter = Crypter::new(cipher, Mode::Encrypt, key, Some(iv))?;

    // Aggiungi AAD (Additional Authenticated Data), se necessario
    crypter.aad_update(&[])?;

    // Crittografia
    let count = crypter.update(message.as_bytes(), &mut encrypted)?;
    let rest = crypter.finalize(&mut encrypted[count..])?;
    encrypted.truncate(count + rest);

    // Ottieni il tag di autenticazione
    let mut tag = vec![0; 16];
    crypter.get_tag(&mut tag)?;

    // Combina i dati crittografati e il tag
    encrypted.extend_from_slice(&tag);
    Ok(encrypted)
}

pub fn decrypt_aes(encrypted_message: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, ErrorStack> {
    let cipher = Cipher::aes_256_gcm();

    // Separa i dati crittografati e il tag
    let (encrypted_data, tag) = encrypted_message.split_at(encrypted_message.len() - 16);

    // Buffer per i dati decrittografati
    let mut decrypted = vec![0; encrypted_data.len() + cipher.block_size()];

    // Crea un Crypter per la decrittografia
    let mut crypter = Crypter::new(cipher, Mode::Decrypt, key, Some(iv))?;

    // Aggiungi AAD (Additional Authenticated Data), se necessario
    crypter.aad_update(&[])?;

    // Decrittografia
    let count = crypter.update(encrypted_data, &mut decrypted)?;
    crypter.set_tag(tag)?; // Imposta il tag di autenticazione
    let rest = crypter.finalize(&mut decrypted[count..])?;
    decrypted.truncate(count + rest);

    Ok(decrypted)
}

pub fn collect_pre_message(plugboard_pairs: &[(char, char)]) -> String {
    let date = Local::now().format("%Y%m%d").to_string(); // Data in formato AAAAMMGG
    let plugboard_chars: String = plugboard_pairs
        .iter()
        .flat_map(|(a, b)| vec![*a, *b])
        .collect(); // Esempio: "ABCD" per le coppie [('A', 'B'), ('C', 'D')]
    format!("{}|{}", date, plugboard_chars) // Formato: DATA-MESSAGGIO-PLUG
}
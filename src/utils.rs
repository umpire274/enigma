use serde_json;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub static FIXED_HASH: u64 = 1737;

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

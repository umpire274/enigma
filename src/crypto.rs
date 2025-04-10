use crate::cli;
use crate::enigma::enigma::EnigmaMachine;
use crate::enigma::utils;
use base64::engine::general_purpose::STANDARD as base64_engine;
use base64::Engine;
use chrono::Local;
use log::{debug, info, warn};

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
    config: &mut utils::Config,
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

    debug!("premessage: {}", premessage);
    debug!("premex: {:?}", premex);
    debug!("data: {}", data);
    debug!("pairs: {}", pairs);

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

    let decrypted = enigma
        .encrypt_message(enigma_message)
        .map_err(|e| format!("Error decrypting with Enigma: {}", e))?;

    if decrypted.contains("MESSAGGIOPERTE") {
        info!("Trigger rilevato: aggiornamento configurazione");

        // Estrai le parti separate dal pipe
        let config_parts: Vec<&str> = premessage.split('|').collect();
        if config_parts.len() >= 2 {
            let config_json = format!(
                r#"{{"n_rt":{},"plugboard_pairs":{},"sstk":{}}}"#,
                config.n_rt,  // Mantieni n_rt esistente o usa nuovo valore se disponibile
                serde_json::to_string(&config_parts[1].chars()
                    .collect::<Vec<char>>()
                    .chunks(2)
                    .map(|chunk| (chunk[0], chunk[1]))
                    .collect::<Vec<(char, char)>>())
                    .map_err(|e| format!("Serialization error: {}", e))?,
                config.sstk  // Mantieni sstk esistente o usa nuovo valore se disponibile
            );

            debug!("Config from message: {}", config_json);

            if let Ok(new_config) = serde_json::from_str::<utils::Config>(&config_json) {
                debug!("new_config: {:?}", new_config);
                *config = new_config;
                config.save().map_err(|e| format!("Failed to save config: {}", e))?;
                info!("Configurazione aggiornata con successo");
            } else {
                warn!("Formato configurazione non valido");
            }
        }
    }

    Ok(decrypted)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enigma::utils::Config;

    // Configurazione di esempio per i test
    fn get_test_config() -> Config {
        Config {
            sstk: 12345,                                   // Seed di esempio
            n_rt: 3,                                       // Numero di rotori di esempio
            plugboard_pairs: vec![('A', 'B'), ('C', 'D')], // Coppie di plugboard di esempio
        }
    }

    // Chiave e IV di esempio per AES
    const TEST_KEY: &[u8] = b"0123456789abcdef0123456789abcdef"; // 32 byte per AES-256
    const TEST_IV: &[u8] = b"1234567890abcdef"; // 16 byte per AES-256-CBC

    #[test]
    fn test_encrypt_message_success() {
        let config = get_test_config();
        let input = "Hello, World!";

        let result = encrypt_message(input, &config, TEST_KEY, TEST_IV);

        assert!(result.is_ok(), "Encryption should succeed");
        let encrypted = result.unwrap();
        assert!(
            !encrypted.is_empty(),
            "Encrypted message should not be empty"
        );
    }

    #[test]
    fn test_encrypt_message_invalid_enigma() {
        let mut config = get_test_config();
        config.n_rt = 0; // Numero di rotori non valido

        let input = "Hello, World!";

        let result = encrypt_message(input, &config, TEST_KEY, TEST_IV);

        assert!(
            result.is_err(),
            "Encryption should fail due to invalid Enigma configuration"
        );
        assert!(result
            .unwrap_err()
            .contains("Error creating Enigma machine"));
    }

    #[test]
    fn test_decrypt_message_success() {
        let config = get_test_config();
        let input = "HELLOWORLD"; // Input già in formato atteso (maiuscolo, senza spazi)

        let encrypted =
            encrypt_message(input, &config, TEST_KEY, TEST_IV).expect("Encryption failed");
        let decrypted = decrypt_message(&encrypted, &mut config.clone(), TEST_KEY, TEST_IV)
            .expect("Decryption failed");

        assert_eq!(decrypted, input, "Decrypted message should match original");
    }

    #[test]
    fn test_decrypt_message_invalid_base64() {
        let mut config = get_test_config();
        let result = decrypt_message("InvalidBase64!!", &mut config, TEST_KEY, TEST_IV);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("base64"));
    }

    #[test]
    fn test_decrypt_message_invalid_aes() {
        let mut config = get_test_config();
        let invalid_data = base64_engine.encode(b"InvalidAESData");
        let result = decrypt_message(&invalid_data, &mut config, TEST_KEY, TEST_IV);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("AES"));
    }

    #[test]
    fn test_decrypt_message_invalid_config() {
        let mut invalid_config = Config {
            n_rt: 0, // Configurazione invalida
            ..get_test_config()
        };

        let encrypted = encrypt_message("TEST", &get_test_config(), TEST_KEY, TEST_IV)
            .expect("Encryption failed");
        let result = decrypt_message(&encrypted, &mut invalid_config, TEST_KEY, TEST_IV);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Enigma"));
    }

    #[test]
    fn test_config_update_trigger() {
        let mut config = get_test_config();
        let new_config = Config {
            n_rt: 4,
            plugboard_pairs: vec![('X', 'Y')],
            sstk: 67890,
        };

        // Simula un messaggio con trigger e nuova configurazione
        let message = format!(
            "{}|MESSAGGIOPERTE",
            serde_json::to_string(&new_config).unwrap()
        );

        let encrypted =
            encrypt_message(&message, &config, TEST_KEY, TEST_IV).expect("Encryption failed");
        let _ =
            decrypt_message(&encrypted, &mut config, TEST_KEY, TEST_IV).expect("Decryption failed");

        assert_eq!(config.n_rt, new_config.n_rt);
        assert_eq!(config.plugboard_pairs, new_config.plugboard_pairs);
    }
}

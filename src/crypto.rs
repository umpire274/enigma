use crate::cli;
use crate::enigma::enigma::EnigmaMachine;
use crate::enigma::utils;
use base64::engine::general_purpose::STANDARD as base64_engine;
use base64::Engine;
use chrono::Local;
use log::debug;

pub fn encrypt_message(
    input: &str,
    config: Option<&utils::Config>,
    key: &[u8],
    iv: &[u8],
) -> Result<String, String> {
    let default_config = utils::Config::load().map_err(|e| format!("Failed to load config: {}", e))?;
    let config = config.unwrap_or(&default_config);

    let current_date = Local::now().format("%Y%m%d").to_string();
    let plugboard_str = config.plugboard_pairs.iter()
        .flat_map(|(a, b)| vec![*a, *b])
        .collect::<String>();

    let mut enigma = match EnigmaMachine::new_from_params(
        config.sstk,
        config.n_rt,
        &current_date,
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

    let _aes_encrypted_base64_pre_message = base64_engine.encode(&aes_encrypted_pre_message);

    let input = cli::preprocess_input(input.trim());
    let enigma_encrypted = match enigma.encrypt_message(&input.to_uppercase()) {
        Ok(text) => text,
        Err(e) => return Err(format!("Error encrypting with Enigma: {}", e)),
    };

    let message = format!("{}|{}|{}|{}|{}",
                          config.sstk,
                          config.n_rt,
                          current_date,
                          plugboard_str,
                          enigma_encrypted
    );

    let aes_encrypted = match utils::encrypt_aes(&message, key, iv) {
        Ok(encrypted) => encrypted,
        Err(e) => return Err(format!("Error encrypting with AES: {}", e)),
    };

    Ok(base64_engine.encode(&aes_encrypted))
}

pub fn decrypt_message(
    input: &str,
    key: &[u8],
    iv: &[u8],
) -> Result<(String, Option<utils::Config>), String> {
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
    if parts.len() < 5 {
        return Err("Invalid message format - expected 5 parts".into());
    }

    debug!("parts: {:?}", parts);

    let sstk = parts[0].parse().unwrap_or(12345);
    let n_rt = parts[1].parse().unwrap_or(3);
    let message_date = parts[2].to_string();
    let plugboard_str = parts[3];
    let enigma_message = parts[4];

    let plugboard_pairs: Vec<(char, char)> = plugboard_str
        .chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .filter(|chunk| chunk.len() == 2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect();

    let config_from_message = utils::Config {
        sstk,
        n_rt,
        plugboard_pairs: plugboard_pairs.clone(),
    };

    let mut enigma = match EnigmaMachine::new_from_params(
        sstk,
        n_rt,
        &message_date,
        plugboard_pairs,
    ) {
        Ok(machine) => machine,
        Err(e) => return Err(format!("Error creating Enigma machine: {}", e)),
    };

    let decrypted = match enigma.encrypt_message(enigma_message) {
        Ok(text) => text,
        Err(e) => return Err(format!("Error decrypting with Enigma: {}", e)),
    };

    Ok((decrypted, Some(config_from_message)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enigma::utils::Config;

    fn get_test_config() -> Config {
        Config {
            sstk: 12345,
            n_rt: 3,
            plugboard_pairs: vec![('A', 'B'), ('C', 'D')],
        }
    }

    const TEST_KEY: &[u8] = b"0123456789abcdef0123456789abcdef";
    const TEST_IV: &[u8] = b"1234567890abcdef";

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let config = get_test_config();
        let input = "TESTMESSAGE";

        let encrypted = encrypt_message(input, Some(&config), TEST_KEY, TEST_IV).unwrap();
        let (decrypted, new_config) = decrypt_message(&encrypted, TEST_KEY, TEST_IV).unwrap();

        assert_eq!(decrypted, input);
        assert!(new_config.is_some());
        assert_eq!(new_config.unwrap().plugboard_pairs, vec![('A','B'), ('C','D')]);
    }
}
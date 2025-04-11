use std::io::{self, Write};
use log::debug;
use crate::crypto::{decrypt_message, encrypt_message};
use crate::enigma::utils;

pub fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn display_output<W: Write>(output: &str, mut writer: W) {
    writeln!(writer, "Result: {}", output).unwrap();
}

pub fn preprocess_input(input: &str) -> String {
    input.chars()
        .map(|c| {
            if c.is_ascii_digit() {
                let num = c.to_digit(10).unwrap() as u8;
                let letter = (b'Z' - num) as char;
                format!("X{}", letter)
            } else {
                c.to_ascii_uppercase().to_string()
            }
        })
        .collect()
}

pub fn postprocess_output(output: &str) -> String {
    let mut result = String::new();
    let mut chars = output.chars().peekable();

    while let Some(c) = chars.next() {
        if c == 'X' {
            if let Some(next_c) = chars.next() {
                if next_c >= 'R' && next_c <= 'Z' {
                    let num = (b'Z' - next_c as u8).to_string();
                    result.push_str(&num);
                } else {
                    result.push('X');
                    result.push(next_c);
                }
            }
        } else {
            result.push(c);
        }
    }
    result
}

pub fn run_cli() -> Result<(), Box<dyn std::error::Error>> {
    println!("Enigma Machine CLI Mode");

    let mut config = match utils::Config::load() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            return Ok(());
        }
    };

    let key = &utils::KEY[..];
    let iv = &utils::IV[..];

    let input = get_user_input("Enter message: ");
    let operation = get_user_input("Encrypt (e) or decrypt (d)? ");

    debug!("Using config: {:?}", config);

    match operation.trim() {
        "e" => {
            let encrypted = encrypt_message(&input, Some(&config), key, iv)?;
            display_output(&encrypted, io::stdout());
        }
        "d" => {
            let (decrypted, new_config) = decrypt_message(&input, key, iv)?;
            if let Some(new_cfg) = new_config {
                config = new_cfg;
                config.save()?;
            }
            display_output(&decrypted, io::stdout());
        }
        _ => eprintln!("Invalid operation"),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preprocess_postprocess_roundtrip() {
        let input = "A1B2C3";
        let processed = preprocess_input(input);
        let output = postprocess_output(&processed);
        assert_eq!(output, "A1B2C3");
    }
}
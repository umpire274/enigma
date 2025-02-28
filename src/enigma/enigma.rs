use super::{plugboard::Plugboard, reflector::Reflector, rotor::Rotor};
use crate::utils;
use chrono::prelude::*;
use log::debug;
use rand::rngs::StdRng;
use rand::{seq::SliceRandom, SeedableRng};
use serde::Deserialize;
use std::fs;

/// Represents the configuration for initializing an Enigma machine.
///
/// This struct is used to deserialize the configuration from a file (e.g., JSON)
/// and contains all the necessary components to set up the machine:
/// - Number of rotors and their notches.
/// - Plugboard pairings.
/// - A seed value for generating random rotors and reflectors.
///
/// # Fields
/// - `n_rt`: The number of rotors to generate.
/// - `notches`: A vector of characters, where each character represents the notch position of a rotor.
/// - `plugboard_pairs`: A vector of character pairs representing the plugboard connections.
/// - `sstk`: A seed value used for random generation of rotors and reflectors.
#[derive(Deserialize)]
pub struct Config {
    n_rt: usize,
    plugboard_pairs: Vec<(char, char)>,
    sstk: usize,
}

/// Represents an Enigma machine with its core components.
///
/// This struct encapsulates the state of the Enigma machine, including:
/// - The rotors, which perform the substitution cipher.
/// - The reflector, which redirects the electrical signal back through the rotors.
/// - The plugboard, which swaps pairs of letters before and after the rotors.
///
/// # Fields
/// - `rotors`: A vector of `Rotor` instances, each representing a rotor in the machine.
/// - `reflector`: A `Reflector` instance representing the reflector in the machine.
/// - `plugboard`: A `Plugboard` instance representing the plugboard in the machine.
pub struct EnigmaMachine {
    rotors: Vec<Rotor>,
    reflector: Reflector,
    plugboard: Plugboard,
}

impl EnigmaMachine {
    /// Initializes a new Enigma machine from a configuration file.
    ///
    /// This function reads a JSON configuration file containing the settings for the Enigma machine,
    /// including the number of rotors, notch positions, plugboard pairings, and a seed for random generation.
    /// It then constructs and returns an `EnigmaMachine` instance based on the provided configuration.
    ///
    /// # Arguments
    /// * `file_path` - The path to the JSON configuration file.
    ///
    /// # Errors
    /// Returns an error in the following cases:
    /// - The file cannot be read (e.g., invalid path or permissions).
    /// - The file content is not valid JSON or does not match the expected structure.
    /// - The number of rotors and notches does not match.
    /// - Any of the components (rotors, reflector, plugboard) fail to initialize due to invalid configurations.
    ///
    /// # Example
    /// ```rust
    /// let enigma = EnigmaMachine::from_config("config.json")?;
    /// println!("Enigma machine initialized successfully!");
    /// ```
    pub fn from_config(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string(file_path)?;
        let config: Config = serde_json::from_str(&config_str)?;

        // Generate notches dynamically
        let notches = Self::generate_notches(config.sstk, config.n_rt);

        // Validate plugboard pairs (no duplicate characters)
        let mut used_chars = std::collections::HashSet::new();
        for (a, b) in &config.plugboard_pairs {
            if !a.is_ascii_uppercase() || !b.is_ascii_uppercase() {
                return Err(
                    "Invalid character in plugboard pairs: Must be ASCII uppercase letters".into(),
                );
            }
            if used_chars.contains(a) || used_chars.contains(b) {
                return Err("Duplicate character in plugboard pairs".into());
            }
            used_chars.insert(*a);
            used_chars.insert(*b);
        }

        let mut rotors: Vec<String> = Vec::new();
        for idx_rotor in 1..=config.n_rt {
            let rotor = Self::generate_rotor(config.sstk, idx_rotor);
            rotors.push(rotor);
            debug!("rotor nr.{}:\t{}", idx_rotor, rotors[idx_rotor - 1]);
        }
        let reflt = Self::generate_reflector(config.n_rt);
        debug!("reflector:\t{}", reflt);

        let rotors = rotors
            .iter()
            .zip(notches)
            .map(|(wiring, notch)| Rotor::new(wiring, notch, 'A'))
            .collect::<Result<Vec<_>, _>>()?;

        let reflector = Reflector::new(reflt.as_str())?;
        debug!("plugboard:\t{:?}", config.plugboard_pairs);
        let plugboard = Plugboard::new(config.plugboard_pairs)?;

        Ok(Self {
            rotors,
            reflector,
            plugboard,
        })
    }

    /// Encrypts a message using the Enigma machine's current configuration.
    ///
    /// This function processes each character of the input message through the Enigma machine's components:
    /// 1. The plugboard swaps the character (if a mapping exists).
    /// 2. The rotors perform a forward substitution.
    /// 3. The reflector redirects the signal back through the rotors.
    /// 4. The rotors perform a reverse substitution.
    /// 5. The plugboard swaps the character again (if a mapping exists).
    ///
    /// # Arguments
    /// * `message` - The message to encrypt. It must consist of ASCII uppercase letters (`A-Z`).
    ///
    /// # Returns
    /// - `Ok(String)`: The encrypted message, where each character is transformed according to the Enigma machine's configuration.
    /// - `Err(&'static str)`: An error message if the input contains invalid characters (non-ASCII uppercase letters).
    ///
    /// # Errors
    /// Returns an error if the message contains any character that is not an ASCII uppercase letter.
    ///
    /// # Example
    /// ```rust
    /// let enigma = EnigmaMachine::from_config("config.json")?;
    /// let encrypted = enigma.encrypt("HELLO")?;
    /// println!("Encrypted: {}", encrypted); // Output: "RFKTZ"
    /// ```
    pub fn encrypt(&self, message: &str) -> Result<String, &'static str> {
        message
            .chars()
            .map(|c| {
                if !c.is_ascii_uppercase() {
                    return Err("Invalid character: Must be an ASCII uppercase letter");
                }
                let mut c = self.plugboard.swap(c)?;
                for rotor in &self.rotors {
                    c = rotor.forward(c)?;
                }
                c = self.reflector.reflect(c)?;
                for rotor in self.rotors.iter().rev() {
                    c = rotor.reverse(c)?;
                }
                self.plugboard.swap(c)
            })
            .collect()
    }

    /// Formats a given text by grouping uppercase ASCII characters into chunks of 4, separated by dashes.
    ///
    /// # Arguments
    /// * `text` - The input text to be formatted.
    ///
    /// # Returns
    /// A `String` where uppercase ASCII characters are grouped into quartets (4 characters each),
    /// separated by dashes (`-`). Non-uppercase characters and spaces are filtered out.
    ///
    /// # Example
    /// ```rust
    /// let formatted = format_dashed("HELLO WORLD");
    /// assert_eq!(formatted, "HELL-OWOR-LD");
    /// ```
    fn format_dashed(&self, text: &str) -> String {
        text.chars()
            .filter(|c| c.is_ascii_uppercase())
            .collect::<Vec<_>>()
            .chunks(4)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("-")
    }

    /// Removes dashes from the input text and joins all characters into a continuous string.
    ///
    /// # Arguments
    /// * `text` - The input text formatted with dashes.
    ///
    /// # Returns
    /// A `String` where all characters are joined together without any dashes.
    ///
    /// # Example
    /// ```rust
    /// let continuous = format_continuous("HELL-OWOR-LD");
    /// assert_eq!(continuous, "HELLOWORLD");
    /// ```
    pub fn format_continuous(&self, text: &str) -> String {
        text.chars().filter(|c| *c != '-').collect::<String>()
    }

    /// Encrypts a message and formats the output based on the presence of dashes.
    ///
    /// This function processes the input message, encrypts it using the Enigma machine, and formats
    /// the output either as a continuous string (if dashes are present) or as dashed quartets (if no dashes are present).
    ///
    /// # Arguments
    /// * `text` - The message to be encrypted.
    ///
    /// # Returns
    /// - `Ok(String)`: The encrypted message, formatted as specified.
    /// - `Err(&'static str)`: An error message if the input contains invalid characters.
    ///
    /// # Example
    /// ```rust
    /// let mut enigma = EnigmaMachine::from_config("config.json").unwrap();
    /// let encrypted = enigma.encrypt_message("HELLO WORLD").unwrap();
    /// println!("Encrypted: {}", encrypted); // Output: "SOME-ENCR-YPTE-DTEX-T"
    /// ```
    pub fn encrypt_message(&mut self, text: &str) -> Result<String, &'static str> {
        let mut result = String::new();
        let mut is_cyphred = false;

        for c in text.chars() {
            if c.is_ascii_alphabetic() {
                self.step_rotors();
                let encrypted_char = self.encrypt(&c.to_ascii_uppercase().to_string())?;
                result.push(encrypted_char.chars().next().unwrap());
            } else if c == '-' {
                is_cyphred = true;
            }
        }

        if is_cyphred {
            Ok(self.format_continuous(&result))
        } else {
            Ok(self.format_dashed(&result))
        }
    }

    /// Rotates the rotors based on their current positions and notches.
    ///
    /// This function advances the rotors in a way that mimics the behavior of the Enigma machine:
    /// - The first rotor always rotates.
    /// - Each subsequent rotor rotates if the previous rotor has reached its notch position.
    ///
    /// # Behavior
    /// - The rotation starts from the first rotor and propagates to the next rotor only if the current rotor's notch is engaged.
    /// - If a rotor does not need to rotate (i.e., its notch is not engaged), the rotation process stops.
    fn step_rotors(&mut self) {
        let mut rotate_next = true;
        for i in 0..self.rotors.len() {
            if rotate_next {
                rotate_next = self.rotors[i].rotate();
            } else {
                break;
            }
        }
    }

    /// Generates a random rotor wiring based on a seed and rotor index.
    ///
    /// This function uses the current date and a seed value to generate a unique wiring for each rotor.
    ///
    /// # Arguments
    /// * `sstk` - A seed value used for random generation.
    /// * `rotor_index` - The index of the rotor (used to ensure unique wiring for each rotor).
    ///
    /// # Returns
    /// A `String` representing the wiring of the rotor.
    fn generate_rotor(sstk: usize, rotor_index: usize) -> String {
        let dmy = Local::now();
        let p1 = dmy.day() as u64;
        let p2 = dmy.month() as u64;
        let p3 = dmy.year() as u64;
        let seed =
            (p1 * sstk as u64) + (p2 * sstk as u64) + p3 + rotor_index as u64 + utils::FIXED_HASH;
        let mut rng = StdRng::seed_from_u64(seed);
        let mut alphabet: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
        alphabet.shuffle(&mut rng);
        alphabet.into_iter().collect()
    }

    /// Generates a random reflector wiring based on a seed.
    ///
    /// This function uses the current date and a seed value to generate a unique wiring for the reflector.
    ///
    /// # Arguments
    /// * `sstk` - A seed value used for random generation.
    ///
    /// # Returns
    /// A `String` representing the wiring of the reflector.
    fn generate_reflector(sstk: usize) -> String {
        let dmy = Local::now();
        let p1 = dmy.day() as u64;
        let p2 = dmy.month() as u64;
        let p3 = dmy.year() as u64;
        let seed = (p1 * sstk as u64) + (p2 * sstk as u64) + p3 + utils::FIXED_HASH;
        let mut rng = StdRng::seed_from_u64(seed);

        let mut alphabet: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
        alphabet.shuffle(&mut rng);

        let mut reflector = vec![' '; 26];
        for i in (0..26).step_by(2) {
            reflector[alphabet[i] as usize - 'A' as usize] = alphabet[i + 1];
            reflector[alphabet[i + 1] as usize - 'A' as usize] = alphabet[i];
        }

        reflector.into_iter().collect()
    }

    /// Generates a random set of notches for the rotors.
    ///
    /// # Arguments
    /// * `sstk` - A seed value used for random generation.
    /// * `n_rt` - The number of rotors (and thus the number of notches to generate).
    ///
    /// # Returns
    /// A `Vec<char>` containing the notches, one for each rotor.
    pub fn generate_notches(sstk: usize, n_rt: usize) -> Vec<char> {
        let dmy = chrono::Local::now();
        let p1 = dmy.day() as u64;
        let p2 = dmy.month() as u64;
        let p3 = dmy.year() as u64;
        let seed = (p1 * sstk as u64) + (p2 * sstk as u64) + p3 + utils::FIXED_HASH;

        let mut rng = StdRng::seed_from_u64(seed);
        let mut alphabet: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
        alphabet.shuffle(&mut rng);

        // Select the first `n_rt` characters as notches
        alphabet.into_iter().take(n_rt).collect()
    }
}

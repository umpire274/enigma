// src/enigma/enigma.rs
use super::plugboard::Plugboard;
use super::reflector::Reflector;
use super::rotor::Rotor;
use crate::cli::postprocess_output;
use crate::utils;
use chrono::prelude::*;
use log::debug;
use rand::rngs::StdRng;
use rand::{seq::SliceRandom, SeedableRng};

/// Represents an Enigma machine with its core components.
pub struct EnigmaMachine {
    rotors: Vec<Rotor>,              // List of rotors
    reflector: Reflector,            // Reflector
    plugboard: Plugboard,            // Plugboard
    pub vec_plug: Vec<(char, char)>, // Plugboard pairs in clear
}

impl EnigmaMachine {
    /// Creates a new Enigma machine from the provided parameters.
    ///
    /// # Arguments
    /// * `sstk` - Seed for random generation of rotors and reflector.
    /// * `n_rt` - Number of rotors.
    /// * `date` - Date in the format `%Y%m%d` (used for generating components).
    /// * `plugboard_pairs` - List of character pairs for the plugboard.
    ///
    /// # Returns
    /// - `Ok(Self)`: The Enigma machine instance.
    /// - `Err(&'static str)`: An error if any component fails to initialize.
    pub fn new_from_params(
        sstk: usize,
        n_rt: usize,
        date: &str,
        plugboard_pairs: Vec<(char, char)>,
    ) -> Result<Self, &'static str> {
        // Validate plugboard pairs using the function from plugboard.rs
        Plugboard::validate_plugboard_pairs(&plugboard_pairs)?;

        // Generate rotors
        debug!("Generating rotors...");
        let rotors = create_rotors(n_rt, sstk, date)?;
        debug!("Rotors generated successfully.");

        // Generate reflector
        debug!("Generating reflector...");
        let reflector = create_reflector(sstk, date)?;
        debug!("Reflector generated successfully.");

        // Create plugboard
        debug!("Creating plugboard...");
        let plugboard = create_plugboard(sstk, date)?;
        debug!("Plugboard created successfully.");

        Ok(Self {
            rotors,
            reflector,
            plugboard,
            vec_plug: plugboard_pairs,
        })
    }

    /// Encrypts a message using the Enigma machine's current configuration.
    ///
    /// # Arguments
    /// * `message` - The message to encrypt (must consist of ASCII uppercase letters).
    ///
    /// # Returns
    /// - `Ok(String)`: The encrypted message.
    /// - `Err(&'static str)`: An error if the input contains invalid characters.
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
    pub fn format_continuous(&self, text: &str) -> String {
        text.chars().filter(|c| *c != '-').collect::<String>()
    }

    /// Encrypts a message and formats the output based on the presence of dashes.
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
            let output = postprocess_output(&result);
            Ok(self.format_continuous(output.as_str()))
        } else {
            Ok(self.format_dashed(&result))
        }
    }

    /// Rotates the rotors based on their current positions and notches.
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

    /// Generates a random set of notches for the rotors.
    fn generate_notches(sstk: usize, n_rt: usize, date: &str) -> Vec<char> {
        let date = NaiveDate::parse_from_str(date, "%Y%m%d").unwrap();
        let p1 = date.day() as u64;
        let p2 = date.month() as u64;
        let p3 = date.year() as u64;
        let seed = (p1 * sstk as u64) + (p2 * sstk as u64) + p3 + utils::FIXED_HASH;

        let mut rng = StdRng::seed_from_u64(seed);
        let mut alphabet: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
        alphabet.shuffle(&mut rng);

        // Select the first `n_rt` characters as notches
        alphabet.into_iter().take(n_rt).collect()
    }
}

/// Creates the rotors for the Enigma machine.
fn create_rotors(n_rt: usize, sstk: usize, date: &str) -> Result<Vec<Rotor>, &'static str> {
    let notches = EnigmaMachine::generate_notches(sstk, n_rt, date);
    let mut rotors = Vec::new();

    for idx_rotor in 1..=n_rt {
        let date = NaiveDate::parse_from_str(date, "%Y%m%d").unwrap();
        let p1 = date.day() as u64;
        let p2 = date.month() as u64;
        let p3 = date.year() as u64;
        let seed =
            (p1 * sstk as u64) + (p2 * sstk as u64) + p3 + idx_rotor as u64 + utils::FIXED_HASH;

        // Create a new rotor with random wiring
        let rotor = Rotor::new(None, notches[idx_rotor - 1], 'A', Some(seed))?;
        rotors.push(rotor);
    }

    Ok(rotors)
}

fn create_plugboard(sstk: usize, date: &str) -> Result<Plugboard, &'static str> {
    let date = NaiveDate::parse_from_str(date, "%Y%m%d").unwrap();
    let p1 = date.day() as u64;
    let p2 = date.month() as u64;
    let p3 = date.year() as u64;
    let seed = (p1 * sstk as u64) + (p2 * sstk as u64) + p3 + utils::FIXED_HASH;

    Plugboard::new(None, Some(seed))
}

fn create_reflector(sstk: usize, date: &str) -> Result<Reflector, &'static str> {
    let date = NaiveDate::parse_from_str(date, "%Y%m%d").unwrap();
    let p1 = date.day() as u64;
    let p2 = date.month() as u64;
    let p3 = date.year() as u64;
    let seed = (p1 * sstk as u64) + (p2 * sstk as u64) + p3 + utils::FIXED_HASH;

    Reflector::new(None, Some(seed))
}

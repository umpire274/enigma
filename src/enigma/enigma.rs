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
#[derive(Debug, PartialEq)]
pub struct EnigmaMachine {
    /// List of rotors used in the Enigma machine.
    rotors: Vec<Rotor>,

    /// Reflector used to redirect the signal back through the rotors.
    reflector: Reflector,

    /// Plugboard used to swap pairs of letters before and after encryption.
    plugboard: Plugboard,

    /// Plugboard pairs in clear text, stored for reference.
    pub vec_plug: Vec<(char, char)>,
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
    ///
    /// # Example
    /// ```rust
    /// let enigma = EnigmaMachine::new_from_params(12345, 3, "20231001", vec![('A', 'B'), ('C', 'D')])?;
    /// ```
    pub fn new_from_params(
        sstk: usize,
        n_rt: usize,
        date: &str,
        plugboard_pairs: Vec<(char, char)>,
    ) -> Result<Self, &'static str> {
        // Verifica che il numero di rotori sia valido
        if n_rt < 1 || n_rt > 5 {
            return Err("Invalid number of rotors. Expected a value between 1 and 5.");
        }

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
        let plugboard = create_plugboard(sstk, date, plugboard_pairs.clone())?;
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
    ///
    /// # Example
    /// ```rust
    /// let encrypted = enigma.encrypt("HELLO")?;
    /// println!("Encrypted message: {}", encrypted);
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
    /// * `text` - The text to format.
    ///
    /// # Returns
    /// A string with characters grouped into chunks of 4, separated by dashes.
    ///
    /// # Example
    /// ```rust
    /// let formatted = enigma.format_dashed("ABCDEFGH");
    /// assert_eq!(formatted, "ABCD-EFGH");
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
    /// * `text` - The text to format.
    ///
    /// # Returns
    /// A continuous string with all dashes removed.
    ///
    /// # Example
    /// ```rust
    /// let formatted = enigma.format_continuous("ABCD-EFGH");
    /// assert_eq!(formatted, "ABCDEFGH");
    /// ```
    pub fn format_continuous(&self, text: &str) -> String {
        text.chars().filter(|c| *c != '-').collect::<String>()
    }

    /// Encrypts a message and formats the output based on the presence of dashes.
    ///
    /// # Arguments
    /// * `text` - The message to encrypt.
    ///
    /// # Returns
    /// - `Ok(String)`: The encrypted and formatted message.
    /// - `Err(&'static str)`: An error if the input contains invalid characters.
    ///
    /// # Example
    /// ```rust
    /// let encrypted = enigma.encrypt_message("HELLO")?;
    /// println!("Encrypted message: {}", encrypted);
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
            let output = postprocess_output(&result);
            Ok(self.format_continuous(output.as_str()))
        } else {
            Ok(self.format_dashed(&result))
        }
    }

    /// Rotates the rotors based on their current positions and notches.
    fn step_rotors(&mut self) {
        // Il primo rotore avanza sempre
        let mut should_rotate_next = self.rotors[0].rotate();

        // I rotori successivi avanzano solo se il precedente HA SEGNALATO la tacca
        for rotor in self.rotors.iter_mut().skip(1) {
            if !should_rotate_next {
                break;
            }
            should_rotate_next = rotor.rotate();
        }
    }
    /// Generates a random set of notches for the rotors.
    ///
    /// # Arguments
    /// * `sstk` - Seed for random generation.
    /// * `n_rt` - Number of rotors.
    /// * `date` - Date in the format `%Y%m%d` (used for generating components).
    ///
    /// # Returns
    /// A vector of characters representing the notches.
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
///
/// # Arguments
/// * `n_rt` - Number of rotors.
/// * `sstk` - Seed for random generation.
/// * `date` - Date in the format `%Y%m%d` (used for generating components).
///
/// # Returns
/// - `Ok(Vec<Rotor>)`: A vector of rotors.
/// - `Err(&'static str)`: An error if any rotor fails to initialize.
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

/// Creates the plugboard for the Enigma machine.
///
/// # Arguments
/// * `sstk` - Seed for random generation.
/// * `date` - Date in the format `%Y%m%d` (used for generating components).
///
/// # Returns
/// - `Ok(Plugboard)`: The plugboard instance.
/// - `Err(&'static str)`: An error if the plugboard fails to initialize.
fn create_plugboard(
    sstk: usize,
    date: &str,
    pairs: Vec<(char, char)>,
) -> Result<Plugboard, &'static str> {
    if !pairs.is_empty() {
        // Usa le coppie fornite
        Plugboard::new(Some(pairs), None)
    } else {
        let date = NaiveDate::parse_from_str(date, "%Y%m%d").unwrap();
        let p1 = date.day() as u64;
        let p2 = date.month() as u64;
        let p3 = date.year() as u64;
        let seed = (p1 * sstk as u64) + (p2 * sstk as u64) + p3 + utils::FIXED_HASH;

        Plugboard::new(None, Some(seed))
    }
}

/// Creates the reflector for the Enigma machine.
///
/// # Arguments
/// * `sstk` - Seed for random generation.
/// * `date` - Date in the format `%Y%m%d` (used for generating components).
///
/// # Returns
/// - `Ok(Reflector)`: The reflector instance.
/// - `Err(&'static str)`: An error if the reflector fails to initialize.
fn create_reflector(sstk: usize, date: &str) -> Result<Reflector, &'static str> {
    let date = NaiveDate::parse_from_str(date, "%Y%m%d").unwrap();
    let p1 = date.day() as u64;
    let p2 = date.month() as u64;
    let p3 = date.year() as u64;
    let seed = (p1 * sstk as u64) + (p2 * sstk as u64) + p3 + utils::FIXED_HASH;

    Reflector::new(None, Some(seed))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a test Enigma machine
    fn create_test_enigma() -> EnigmaMachine {
        EnigmaMachine::new_from_params(
            12345,                        // seed
            3,                            // 3 rotors
            "20231001",                   // fixed date
            vec![('A', 'B'), ('C', 'D')], // plugboard pairs
        )
        .unwrap()
    }

    #[test]
    fn test_new_from_params() {
        // Test valid creation
        let enigma =
            EnigmaMachine::new_from_params(12345, 3, "20231001", vec![('A', 'B'), ('C', 'D')]);
        assert!(enigma.is_ok());

        // Test invalid number of rotors
        assert_eq!(
            EnigmaMachine::new_from_params(12345, 0, "20231001", vec![]),
            Err("Invalid number of rotors. Expected a value between 1 and 5.")
        );
        assert_eq!(
            EnigmaMachine::new_from_params(12345, 6, "20231001", vec![]),
            Err("Invalid number of rotors. Expected a value between 1 and 5.")
        );

        // Test invalid plugboard pairs
        assert_eq!(
            EnigmaMachine::new_from_params(12345, 3, "20231001", vec![('A', 'a')]),
            Err("Invalid character in plugboard pairs: Must be ASCII uppercase letters")
        );
    }

    #[test]
    fn test_encrypt_basic() {
        // Creiamo una configurazione più semplice e prevedibile
        let enigma = EnigmaMachine::new_from_params(
            0, // seed fissa
            3,
            "20230101",                   // data fissa
            vec![('A', 'B'), ('C', 'D')], // solo queste sostituzioni
        )
        .unwrap();

        // Test plugboard isolato
        assert_eq!(enigma.plugboard.swap('A').unwrap(), 'B');
        assert_eq!(enigma.plugboard.swap('B').unwrap(), 'A');
        assert_eq!(enigma.plugboard.swap('C').unwrap(), 'D');
        assert_eq!(enigma.plugboard.swap('X').unwrap(), 'X'); // Nessuna mappatura

        // Test cifratura completa con rotori in posizione iniziale
        // (Dobbiamo conoscere la configurazione esatta dei rotori)
        // Potremmo dover mockare i rotori per questo test
    }

    #[test]
    fn test_encrypt_invalid_chars() {
        let enigma = create_test_enigma();

        // Test invalid characters
        assert_eq!(
            enigma.encrypt("a"),
            Err("Invalid character: Must be an ASCII uppercase letter")
        );
        assert_eq!(
            enigma.encrypt("1"),
            Err("Invalid character: Must be an ASCII uppercase letter")
        );
        assert_eq!(
            enigma.encrypt(" "),
            Err("Invalid character: Must be an ASCII uppercase letter")
        );
    }

    #[test]
    fn test_format_dashed() {
        let enigma = create_test_enigma();

        assert_eq!(enigma.format_dashed("ABCDEFGH"), "ABCD-EFGH");
        assert_eq!(enigma.format_dashed("ABCDEF"), "ABCD-EF");
        assert_eq!(enigma.format_dashed("ABC"), "ABC");
        assert_eq!(enigma.format_dashed("ABCDEFGHIJKL"), "ABCD-EFGH-IJKL");
    }

    #[test]
    fn test_format_continuous() {
        let enigma = create_test_enigma();

        assert_eq!(enigma.format_continuous("ABCD-EFGH"), "ABCDEFGH");
        assert_eq!(enigma.format_continuous("AB-CD-EF"), "ABCDEF");
        assert_eq!(enigma.format_continuous("ABC"), "ABC");
    }

    #[test]
    fn test_encrypt_message() {
        let mut enigma = create_test_enigma();

        // Test without dashes (should add formatting)
        let result = enigma.encrypt_message("HELLO").unwrap();
        assert_eq!(result.len(), 5 + 1); // 5 letters + 1 dash (if grouped in 4)

        // Test with dashes (should remove formatting)
        let result = enigma.encrypt_message("HELLO-WORLD").unwrap();
        assert!(result.chars().all(|c| c != '-'));
    }

    #[test]
    fn test_step_rotors() {
        let mut enigma = create_test_enigma();

        // Configurazione controllata
        enigma.rotors[0].notch = 'B'; // Tacca in posizione 1
        enigma.rotors[0].position = 0; // 'A'
        enigma.rotors[1].position = 0; // 'A'

        // Primo step - solo il primo rotore avanza
        enigma.step_rotors();
        assert_eq!(enigma.rotors[0].position, 1); // 'B'
        assert_eq!(enigma.rotors[1].position, 1); // Deve rimanere fermo

        // Secondo step - primo rotore STA PER PASSARE 'B' (tacca)
        enigma.step_rotors();
        assert_eq!(enigma.rotors[0].position, 2); // 'C'
        assert_eq!(enigma.rotors[1].position, 1); // Avanzato perché il primo ha segnalato
    }

    #[test]
    fn test_generate_notches() {
        let notches = EnigmaMachine::generate_notches(12345, 3, "20231001");
        assert_eq!(notches.len(), 3);
        assert!(notches.iter().all(|c| c.is_ascii_uppercase()));
        assert_eq!(
            notches
                .iter()
                .collect::<std::collections::HashSet<_>>()
                .len(),
            3
        ); // All unique
    }

    #[test]
    fn test_create_components() {
        // Test rotor creation
        let rotors = create_rotors(3, 12345, "20231001").unwrap();
        assert_eq!(rotors.len(), 3);

        // Test plugboard creation
        let plugboard = create_plugboard(12345, "20231001", vec![('A', 'B'), ('C', 'D')]).unwrap();
        assert!(plugboard.swap('A').is_ok()); // Should work with any char

        // Test reflector creation
        let reflector = create_reflector(12345, "20231001").unwrap();
        assert!(reflector.reflect('A').is_ok());
    }

    #[test]
    fn test_encrypt_roundtrip() {
        let mut enigma1 =
            EnigmaMachine::new_from_params(12345, 3, "20231001", vec![('A', 'B'), ('C', 'D')])
                .unwrap();

        let plaintext = "TESTMESSAGE";
        let encrypted = enigma1.encrypt_message(plaintext).unwrap();

        // Reset rotors to initial positions
        let mut enigma2 =
            EnigmaMachine::new_from_params(12345, 3, "20231001", vec![('A', 'B'), ('C', 'D')])
                .unwrap();

        let decrypted = enigma2.encrypt_message(&encrypted).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_set_position_before_notch() {
        let mut enigma = create_test_enigma();

        // Configurazione esplicita:
        // - Primo rotore: posizione 0 ('A'), tacca in 'B' (posizione 1)
        // - Secondo rotore: posizione 0 ('A'), tacca in 'C' (posizione 2)
        enigma.rotors[0].position = 0;
        enigma.rotors[0].notch = 'B';
        enigma.rotors[1].position = 0;
        enigma.rotors[1].notch = 'C';

        // Primo step_rotors:
        // - Rotor 0 avanza da 'A'(0) a 'B'(1) → segnala passaggio tacca
        // - Rotor 1 dovrebbe avanzare perché Rotor 0 ha segnalato
        enigma.step_rotors();

        assert_eq!(enigma.rotors[0].position, 1); // 'B'
        assert_eq!(enigma.rotors[1].position, 1); // 'B' (avanzato perché Rotor 0 ha segnalato)
    }
}

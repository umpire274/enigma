use super::{plugboard::Plugboard, reflector::Reflector, rotor::Rotor};
use serde::Deserialize;
use std::fs;

/// Represents the configuration for initializing an Enigma machine.
///
/// This struct is used to deserialize the configuration from a file (e.g., JSON)
/// and contains all the necessary components to set up the machine:
/// - Rotor wirings and notches.
/// - Reflector wiring.
/// - Plugboard pairings.
///
/// # Fields
/// - `rotors`: A vector of strings, where each string represents the wiring of a rotor.
/// - `notches`: A vector of characters, where each character represents the notch position of a rotor.
/// - `reflector`: A string representing the wiring of the reflector.
/// - `plugboard_pairs`: A vector of character pairs representing the plugboard connections.
#[derive(Deserialize)]
pub struct Config {
    rotors: Vec<String>,
    notches: Vec<char>,
    reflector: String,
    plugboard_pairs: Vec<(char, char)>,
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
    /// including rotor wirings, notch positions, reflector wiring, and plugboard pairings. It then
    /// constructs and returns an `EnigmaMachine` instance based on the provided configuration.
    ///
    /// # Arguments
    /// * `file_path` - The path to the JSON configuration file. The file should contain the following fields:
    ///   - `rotors`: A list of strings, where each string represents the wiring of a rotor.
    ///   - `notches`: A list of characters, where each character represents the notch position of a rotor.
    ///   - `reflector`: A string representing the wiring of the reflector.
    ///   - `plugboard_pairs`: A list of character pairs representing the plugboard connections.
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

        // Check that the number of rotors and notches is the same
        if config.rotors.len() != config.notches.len() {
            return Err("The number of rotors and notches must be the same".into());
        }

        let rotors = config
            .rotors
            .iter()
            .zip(&config.notches)
            .map(|(wiring, &notch)| Rotor::new(wiring, notch, 'A'))
            .collect::<Result<Vec<_>, _>>()?;

        let reflector = Reflector::new(&config.reflector)?;
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
            .filter(|c| c.is_ascii_uppercase()) // Filters out non-uppercase ASCII characters and spaces
            .collect::<Vec<_>>() // Collects the filtered characters into a vector
            .chunks(4) // Splits the vector into chunks of 4 characters each
            .map(|chunk| chunk.iter().collect::<String>()) // Converts each chunk into a string
            .collect::<Vec<_>>() // Collects the chunk strings into a vector
            .join("-") // Joins the chunk strings with dashes
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
        text.chars()
            .filter(|c| *c != '-') // Filters out dashes from the input text
            .collect::<String>() // Collects the remaining characters into a single string
    }

    /// Encrypts a message and formats the output based on the presence of dashes.
    ///
    /// # Arguments
    /// * `text` - The message to be encrypted.
    ///
    /// # Returns
    /// A `Result` containing the encrypted message formatted either as a continuous string (if dashes are present)
    /// or as dashed quartets (if no dashes are present). Returns an error if the message contains invalid characters.
    ///
    /// # Example
    /// ```rust
    /// let mut enigma = EnigmaMachine::from_config("config.json").unwrap();
    /// let encrypted = enigma.encrypt_message("HELLO WORLD").unwrap();
    /// println!("Encrypted: {}", encrypted); // Output: "SOME-ENCR-YPTE-DTEX-T"
    /// ```
    pub fn encrypt_message(&mut self, text: &str) -> Result<String, &'static str> {
        let mut result = String::new();
        let mut is_cyphred = false; // Flag to check if the input contains dashes

        for c in text.chars() {
            if c.is_ascii_alphabetic() {
                self.step_rotors(); // Rotates the rotors before encrypting
                let encrypted_char = self.encrypt(&c.to_ascii_uppercase().to_string())?;
                result.push(encrypted_char.chars().next().unwrap());
            } else if c == '-' {
                is_cyphred = true; // Sets the flag if a dash is encountered
            }
        }

        // Format the output based on the presence of dashes
        if is_cyphred {
            Ok(self.format_continuous(&result)) // Returns a continuous string if dashes were present
        } else {
            Ok(self.format_dashed(&result)) // Returns dashed quartets if no dashes were present
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
        let mut rotate_next = true; // The first rotor always rotates
        for i in 0..self.rotors.len() {
            if rotate_next {
                // Rotate the current rotor and check if the next rotor should also rotate
                rotate_next = self.rotors[i].rotate();
            } else {
                // If a rotor does not rotate, stop the process
                break;
            }
        }
    }
}

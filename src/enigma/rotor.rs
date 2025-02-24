/// Represents a rotor in the Enigma machine.
///
/// A rotor is a key component of the Enigma machine that performs a substitution cipher
/// on each character. It has a mapping for forward encryption, a reverse mapping for
/// decryption, a notch position that triggers the next rotor to rotate, and a current
/// position that determines its state.
///
/// # Fields
/// - `mapping`: An array of 26 characters representing the forward substitution mapping.
/// - `reverse_mapping`: An array of 26 characters representing the reverse substitution mapping.
/// - `notch`: The character at which the rotor triggers the next rotor to rotate.
/// - `position`: The current position of the rotor (0-25, corresponding to 'A'-'Z').
#[derive(Debug)]
pub struct Rotor {
    pub mapping: [char; 26],
    pub reverse_mapping: [char; 26],
    pub notch: char, // The character at which the rotor triggers the next rotor to rotate
    pub position: usize, // The current position of the rotor (0-25)
}

impl Rotor {
    /// Creates a new rotor with the specified wiring, notch, and initial position.
    ///
    /// # Arguments
    /// * `wiring` - A string of 26 characters representing the rotor's substitution mapping.
    /// * `notch` - The character at which the rotor triggers the next rotor to rotate.
    /// * `position` - The initial position of the rotor (must be an ASCII uppercase letter).
    ///
    /// # Errors
    /// Returns an error in the following cases:
    /// - The `wiring` string does not have exactly 26 characters.
    /// - The `notch` or `position` is not a valid ASCII uppercase letter (`A-Z`).
    ///
    /// # Example
    /// ```rust
    /// let rotor = Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", 'Q', 'A')?;
    /// println!("Rotor created: {:?}", rotor);
    /// ```
    pub fn new(wiring: &str, notch: char, position: char) -> Result<Self, &'static str> {
        if wiring.len() != 26 {
            return Err("The rotor wiring must have exactly 26 characters");
        }
        if notch < 'A' || notch > 'Z' || position < 'A' || position > 'Z' {
            return Err("The notch and position must be valid ASCII uppercase letters ('A'-'Z')");
        }

        let position_offset = (position as u8 - b'A') as usize;

        let mapping: [char; 26] = wiring.chars().collect::<Vec<char>>().try_into().unwrap();
        let mut reverse_mapping = ['A'; 26];

        for (i, &c) in mapping.iter().enumerate() {
            if let Some(index) = (c as u8).checked_sub(b'A') {
                reverse_mapping[index as usize] = (b'A' + i as u8) as char;
            }
        }

        Ok(Self {
            mapping,
            reverse_mapping,
            notch,
            position: position_offset,
        })
    }

    ///
    /// # Arguments
    /// * `c` - The character to encrypt (must be an ASCII uppercase letter).
    ///
    /// # Returns
    /// - `Ok(char)`: The encrypted character.
    /// - `Err(&'static str)`: An error if the character is not a valid ASCII uppercase letter.
    ///
    /// # Example
    /// ```rust
    /// let encrypted_char = rotor.forward('A')?;
    /// println!("Encrypted: {}", encrypted_char);
    /// ```
    pub fn forward(&self, c: char) -> Result<char, &'static str> {
        if !c.is_ascii_uppercase() {
            return Err("Invalid character: Must be an ASCII uppercase letter");
        }
        let index = (c as u8 - b'A') as usize;
        Ok(self.mapping[index])
    }

    /// Encrypts a character in the reverse direction (left to right).
    ///
    /// # Arguments
    /// * `c` - The character to encrypt (must be an ASCII uppercase letter).
    ///
    /// # Returns
    /// - `Ok(char)`: The encrypted character.
    /// - `Err(&'static str)`: An error if the character is not a valid ASCII uppercase letter.
    ///
    /// # Example
    /// ```rust
    /// let encrypted_char = rotor.reverse('A')?;
    /// println!("Encrypted: {}", encrypted_char);
    /// ```
    pub fn reverse(&self, c: char) -> Result<char, &'static str> {
        if !c.is_ascii_uppercase() {
            return Err("Invalid character: Must be an ASCII uppercase letter");
        }
        let index = (c as u8 - b'A') as usize;
        Ok(self.reverse_mapping[index])
    }

    /// Rotates the rotor by one position.
    ///
    /// # Returns
    /// - `true`: If the rotor is on its notch position after rotation.
    /// - `false`: Otherwise.
    ///
    /// # Example
    /// ```rust
    /// let should_rotate_next = rotor.rotate();
    /// println!("Should rotate next rotor: {}", should_rotate_next);
    /// ```
    pub fn rotate(&mut self) -> bool {
        self.position = (self.position + 1) % 26; // Advance by 1 position
        self.get_current_letter() == self.notch // Return true if the rotor is on its notch
    }

    /// Returns the current letter at the rotor's position.
    ///
    /// # Returns
    /// The current letter (an ASCII uppercase character).
    ///
    /// # Example
    /// ```rust
    /// let current_letter = rotor.get_current_letter();
    /// println!("Current letter: {}", current_letter);
    /// ```
    pub fn get_current_letter(&self) -> char {
        self.mapping[self.position] // Return the current letter
    }
}

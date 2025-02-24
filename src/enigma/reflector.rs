/// Represents a reflector in the Enigma machine.
///
/// A reflector is a fixed component of the Enigma machine that redirects the electrical signal
/// back through the rotors after they have performed their substitutions. It ensures that the
/// encryption process is reversible by mapping each character to another character in a fixed way.
///
/// # Fields
/// - `mapping`: A fixed-size array of 26 characters representing the reflector's substitution mapping.
#[derive(Debug)]
pub struct Reflector {
    mapping: [char; 26], // Uses a fixed-size array instead of a Vec
}

impl Reflector {
    /// Creates a new reflector with the specified wiring.
    ///
    /// The wiring must be a string of exactly 26 characters, where each character represents
    /// the mapping for a corresponding letter (`A` to `Z`).
    ///
    /// # Arguments
    /// * `wiring` - A string of 26 characters representing the reflector's substitution mapping.
    ///
    /// # Errors
    /// Returns an error if the `wiring` string does not have exactly 26 characters.
    ///
    /// # Example
    /// ```rust
    /// let reflector = Reflector::new("EJMZALYXVBWFCRQUONTSPIKHGD")?;
    /// println!("Reflector created: {:?}", reflector);
    /// ```
    pub fn new(wiring: &str) -> Result<Self, &'static str> {
        if wiring.len() != 26 {
            return Err("The reflector wiring must have exactly 26 characters");
        }

        let mut mapping = ['A'; 26]; // Fixed-size array of 26 characters

        for (i, c) in wiring.chars().enumerate() {
            if let Some(index) = (c as u8).checked_sub(b'A') {
                mapping[index as usize] = (b'A' + i as u8) as char;
            }
        }

        Ok(Self { mapping })
    }

    /// Reflects a character using the reflector's mapping.
    ///
    /// This method takes a character and returns the corresponding character based on the
    /// reflector's substitution mapping. The input character must be an ASCII uppercase letter.
    ///
    /// # Arguments
    /// * `c` - The character to reflect (must be an ASCII uppercase letter).
    ///
    /// # Returns
    /// - `Ok(char)`: The reflected character.
    /// - `Err(&'static str)`: An error if the character is not a valid ASCII uppercase letter.
    ///
    /// # Example
    /// ```rust
    /// let reflected_char = reflector.reflect('A')?;
    /// println!("Reflected: {}", reflected_char);
    /// ```
    pub fn reflect(&self, c: char) -> Result<char, &'static str> {
        if !c.is_ascii_uppercase() {
            return Err("Invalid character: Must be an ASCII uppercase letter");
        }
        let index = (c as u8 - b'A') as usize;
        Ok(self.mapping[index])
    }
}

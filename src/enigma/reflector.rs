use rand::rngs::StdRng;
use rand::{seq::SliceRandom, SeedableRng};

/// Represents a reflector in the Enigma machine.
///
/// A reflector is a fixed component that redirects the electrical signal back through the rotors
/// after they have performed their substitutions. It ensures that the encryption process is reversible.
#[derive(Debug)]
pub struct Reflector {
    /// The mapping of characters for reflection.
    mapping: [char; 26],
}

impl Reflector {
    /// Creates a new reflector with the specified wiring.
    ///
    /// If `seed` is provided, the wiring is generated randomly based on the seed.
    /// Otherwise, the provided `wiring` is used.
    ///
    /// # Arguments
    /// * `wiring` - A string of 26 characters representing the reflector's substitution mapping.
    ///              If `seed` is provided, this parameter is ignored.
    /// * `seed` - An optional seed for generating random wiring.
    ///
    /// # Returns
    /// - `Ok(Self)`: The reflector instance.
    /// - `Err(&'static str)`: An error if the wiring string does not have exactly 26 characters (if `seed` is `None`).
    ///
    /// # Example
    /// ```rust
    /// let reflector = Reflector::new(Some("EJMZALYXVBWFCRQUONTSPIKHGD"), None)?;
    /// ```
    pub fn new(wiring: Option<&str>, seed: Option<u64>) -> Result<Self, &'static str> {
        let wiring = match seed {
            Some(seed) => {
                // Generate random reflector wiring
                let mut rng = StdRng::seed_from_u64(seed);
                let mut alphabet: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
                alphabet.shuffle(&mut rng);

                let mut reflector = vec![' '; 26];
                for i in (0..26).step_by(2) {
                    if i + 1 < 26 {
                        reflector[alphabet[i] as usize - 'A' as usize] = alphabet[i + 1];
                        reflector[alphabet[i + 1] as usize - 'A' as usize] = alphabet[i];
                    }
                }
                reflector.into_iter().collect()
            }
            None => {
                if let Some(wiring) = wiring {
                    if wiring.len() != 26 {
                        return Err("The reflector wiring must have exactly 26 characters");
                    }
                    wiring.to_string()
                } else {
                    return Err("Either wiring or seed must be provided");
                }
            }
        };

        let mut mapping = ['A'; 26];
        for (i, c) in wiring.chars().enumerate() {
            if let Some(index) = (c as u8).checked_sub(b'A') {
                mapping[index as usize] = (b'A' + i as u8) as char;
            }
        }

        Ok(Self { mapping })
    }

    /// Reflects a character using the reflector's mapping.
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

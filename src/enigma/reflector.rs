use rand::rngs::StdRng;
use rand::{seq::SliceRandom, SeedableRng};

/// Represents a reflector in the Enigma machine.
///
/// A reflector is a fixed component that redirects the electrical signal back through the rotors
/// after they have performed their substitutions. It ensures that the encryption process is reversible.
#[derive(Debug)]
#[derive(PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_with_valid_wiring() {
        // Standard reflector wiring (example from Enigma I)
        let wiring = "EJMZALYXVBWFCRQUONTSPIKHGD";
        let reflector = Reflector::new(Some(wiring), None).unwrap();

        // Verify the mapping is correct
        assert_eq!(reflector.mapping, [
            'E', 'J', 'M', 'Z', 'A', 'L', 'Y', 'X', 'V', 'B',
            'W', 'F', 'C', 'R', 'Q', 'U', 'O', 'N', 'T', 'S',
            'P', 'I', 'K', 'H', 'G', 'D'
        ]);
    }

    #[test]
    fn test_new_with_invalid_wiring_length() {
        // Wiring too short
        let result = Reflector::new(Some("ABC"), None);
        assert_eq!(result, Err("The reflector wiring must have exactly 26 characters"));

        // Wiring too long
        let result = Reflector::new(Some("ABCDEFGHIJKLMNOPQRSTUVWXYZABC"), None);
        assert_eq!(result, Err("The reflector wiring must have exactly 26 characters"));
    }

    #[test]
    fn test_new_with_no_wiring_or_seed() {
        let result = Reflector::new(None, None);
        assert_eq!(result, Err("Either wiring or seed must be provided"));
    }

    #[test]
    fn test_new_with_seed() {
        // Test that the same seed produces the same wiring
        let reflector1 = Reflector::new(None, Some(42)).unwrap();
        let reflector2 = Reflector::new(None, Some(42)).unwrap();
        assert_eq!(reflector1.mapping, reflector2.mapping);

        // Test that different seeds produce different wirings
        let reflector3 = Reflector::new(None, Some(43)).unwrap();
        assert_ne!(reflector1.mapping, reflector3.mapping);

        // Verify the wiring is valid (each letter maps to another and back)
        for c in 'A'..='Z' {
            let reflected = reflector1.reflect(c).unwrap();
            let reflected_back = reflector1.reflect(reflected).unwrap();
            assert_eq!(c, reflected_back);
        }
    }

    #[test]
    fn test_reflect_valid_characters() {
        // Using a known wiring for predictable results
        let wiring = "EJMZALYXVBWFCRQUONTSPIKHGD";
        let reflector = Reflector::new(Some(wiring), None).unwrap();

        // Test some known mappings
        assert_eq!(reflector.reflect('A').unwrap(), 'E');
        assert_eq!(reflector.reflect('E').unwrap(), 'A');
        assert_eq!(reflector.reflect('B').unwrap(), 'J');
        assert_eq!(reflector.reflect('J').unwrap(), 'B');
    }

    #[test]
    fn test_reflect_invalid_characters() {
        let wiring = "EJMZALYXVBWFCRQUONTSPIKHGD";
        let reflector = Reflector::new(Some(wiring), None).unwrap();

        // Test non-uppercase ASCII
        assert_eq!(reflector.reflect('a'), Err("Invalid character: Must be an ASCII uppercase letter"));
        assert_eq!(reflector.reflect('1'), Err("Invalid character: Must be an ASCII uppercase letter"));
        assert_eq!(reflector.reflect(' '), Err("Invalid character: Must be an ASCII uppercase letter"));
        assert_eq!(reflector.reflect('Å'), Err("Invalid character: Must be an ASCII uppercase letter"));
    }

    #[test]
    fn test_reflect_all_characters() {
        // Test that every character reflects to another and back
        let wiring = "EJMZALYXVBWFCRQUONTSPIKHGD";
        let reflector = Reflector::new(Some(wiring), None).unwrap();

        for c in 'A'..='Z' {
            let reflected = reflector.reflect(c).unwrap();
            assert_ne!(c, reflected); // A character should never reflect to itself
            let reflected_back = reflector.reflect(reflected).unwrap();
            assert_eq!(c, reflected_back);
        }
    }

    #[test]
    fn test_random_reflector_validity() {
        // Test that randomly generated reflector follows proper pairing rules
        let reflector = Reflector::new(None, Some(12345)).unwrap();

        // Check all mappings are reciprocal
        for c in 'A'..='Z' {
            let reflected = reflector.reflect(c).unwrap();
            assert_ne!(c, reflected); // No character should map to itself
            assert_eq!(reflector.reflect(reflected).unwrap(), c);
        }

        // Check all characters are mapped (no duplicates)
        let mapped = reflector.mapping.iter().collect::<std::collections::HashSet<_>>();
        assert_eq!(mapped.len(), 26);
    }
}
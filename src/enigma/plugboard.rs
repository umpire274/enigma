use rand::rngs::StdRng;
use rand::{seq::SliceRandom, SeedableRng};
use std::collections::HashMap;

/// Represents a plugboard in the Enigma machine.
///
/// The plugboard swaps pairs of letters before and after they pass through the rotors,
/// adding an extra layer of substitution to the encryption process.
#[derive(Debug, PartialEq)]
pub struct Plugboard {
    /// A mapping of characters to their swapped counterparts.
    mapping: HashMap<char, char>,
}

impl Plugboard {
    /// Creates a new plugboard with the specified character pairs.
    ///
    /// If `seed` is provided, the plugboard pairs are generated randomly based on the seed.
    /// Otherwise, the provided `pairs` are used.
    ///
    /// # Arguments
    /// * `pairs` - A vector of character pairs to swap. Each pair is represented as a tuple `(char, char)`.
    ///             If `seed` is provided, this parameter is ignored.
    /// * `seed` - An optional seed for generating random plugboard pairs.
    ///
    /// # Returns
    /// - `Ok(Self)`: The plugboard instance.
    /// - `Err(&'static str)`: An error if the pairs contain invalid characters or duplicate mappings.
    ///
    /// # Example
    /// ```rust
    /// let plugboard = Plugboard::new(Some(vec![('A', 'B'), ('C', 'D')]), None)?;
    /// ```
    pub fn new(pairs: Option<Vec<(char, char)>>, seed: Option<u64>) -> Result<Self, &'static str> {
        let pairs = match seed {
            Some(seed) => {
                // Generate random plugboard pairs
                let mut rng = StdRng::seed_from_u64(seed);
                let mut alphabet: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
                alphabet.shuffle(&mut rng);

                let mut pairs = Vec::new();
                for i in (0..26).step_by(2) {
                    if i + 1 < 26 {
                        pairs.push((alphabet[i], alphabet[i + 1]));
                    }
                }
                pairs
            }
            None => {
                if let Some(pairs) = pairs {
                    pairs
                } else {
                    return Err("Either pairs or seed must be provided");
                }
            }
        };

        // Validate plugboard pairs
        Self::validate_plugboard_pairs(&pairs)?;

        let mut mapping = HashMap::new();
        for (a, b) in pairs {
            mapping.insert(a, b);
            mapping.insert(b, a);
        }

        Ok(Self { mapping })
    }

    /// Swaps a character based on the plugboard's mapping.
    ///
    /// # Arguments
    /// * `c` - The character to swap (must be an ASCII uppercase letter).
    ///
    /// # Returns
    /// - `Ok(char)`: The swapped character.
    /// - `Err(&'static str)`: An error if the character is not a valid ASCII uppercase letter.
    ///
    /// # Example
    /// ```rust
    /// let swapped_char = plugboard.swap('A')?;
    /// println!("Swapped: {}", swapped_char);
    /// ```
    pub fn swap(&self, c: char) -> Result<char, &'static str> {
        if !c.is_ascii_uppercase() {
            return Err("Invalid character: Must be an ASCII uppercase letter");
        }
        Ok(*self.mapping.get(&c).unwrap_or(&c))
    }

    /// Validates the plugboard pairs.
    ///
    /// # Arguments
    /// * `pairs` - A slice of character pairs to validate.
    ///
    /// # Returns
    /// - `Ok(())`: If all pairs are valid.
    /// - `Err(&'static str)`: An error if any pair contains invalid characters or duplicate mappings.
    ///
    /// # Example
    /// ```rust
    /// Plugboard::validate_plugboard_pairs(&[('A', 'B'), ('C', 'D')])?;
    /// ```
    pub fn validate_plugboard_pairs(pairs: &[(char, char)]) -> Result<(), &'static str> {
        let mut used_chars = std::collections::HashSet::new();
        for (a, b) in pairs {
            if !a.is_ascii_uppercase() || !b.is_ascii_uppercase() {
                return Err(
                    "Invalid character in plugboard pairs: Must be ASCII uppercase letters",
                );
            }
            if a == b {
                return Err("Self-mapping is not allowed in plugboard pairs");
            }
            if used_chars.contains(a) || used_chars.contains(b) {
                return Err("Duplicate character in plugboard pairs");
            }
            used_chars.insert(*a);
            used_chars.insert(*b);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_with_valid_pairs() {
        let pairs = vec![('A', 'B'), ('C', 'D'), ('E', 'F')];
        let plugboard = Plugboard::new(Some(pairs), None).unwrap();

        // Verify the mapping is correct
        assert_eq!(plugboard.swap('A').unwrap(), 'B');
        assert_eq!(plugboard.swap('B').unwrap(), 'A');
        assert_eq!(plugboard.swap('C').unwrap(), 'D');
        assert_eq!(plugboard.swap('D').unwrap(), 'C');
        assert_eq!(plugboard.swap('E').unwrap(), 'F');
        assert_eq!(plugboard.swap('F').unwrap(), 'E');

        // Verify unmapped characters return themselves
        assert_eq!(plugboard.swap('G').unwrap(), 'G');
        assert_eq!(plugboard.swap('Z').unwrap(), 'Z');
    }

    #[test]
    fn test_new_with_invalid_pairs() {
        // Test non-uppercase characters
        let result = Plugboard::new(Some(vec![('A', 'b')]), None);
        assert_eq!(
            result,
            Err("Invalid character in plugboard pairs: Must be ASCII uppercase letters")
        );

        // Test duplicate characters
        let result = Plugboard::new(Some(vec![('A', 'B'), ('B', 'C')]), None);
        assert_eq!(result, Err("Duplicate character in plugboard pairs"));

        // Test self-mapping (not allowed in Enigma)
        let result = Plugboard::new(Some(vec![('A', 'A')]), None);
        assert_eq!(
            result,
            Err("Self-mapping is not allowed in plugboard pairs")
        );
    }

    #[test]
    fn test_new_with_no_pairs_or_seed() {
        let result = Plugboard::new(None, None);
        assert_eq!(result, Err("Either pairs or seed must be provided"));
    }

    #[test]
    fn test_new_with_seed() {
        // Test that the same seed produces the same pairs
        let plugboard1 = Plugboard::new(None, Some(42)).unwrap();
        let plugboard2 = Plugboard::new(None, Some(42)).unwrap();

        // Verify all mappings are the same
        for c in 'A'..='Z' {
            assert_eq!(plugboard1.swap(c).unwrap(), plugboard2.swap(c).unwrap());
        }

        // Test that different seeds produce different pairs
        let plugboard3 = Plugboard::new(None, Some(43)).unwrap();
        let mut different = false;
        for c in 'A'..='Z' {
            if plugboard1.swap(c).unwrap() != plugboard3.swap(c).unwrap() {
                different = true;
                break;
            }
        }
        assert!(
            different,
            "Different seeds should produce different mappings"
        );
    }

    #[test]
    fn test_swap_valid_characters() {
        let pairs = vec![('A', 'B'), ('X', 'Y')];
        let plugboard = Plugboard::new(Some(pairs), None).unwrap();

        // Test mapped characters
        assert_eq!(plugboard.swap('A').unwrap(), 'B');
        assert_eq!(plugboard.swap('B').unwrap(), 'A');
        assert_eq!(plugboard.swap('X').unwrap(), 'Y');
        assert_eq!(plugboard.swap('Y').unwrap(), 'X');

        // Test unmapped characters
        assert_eq!(plugboard.swap('C').unwrap(), 'C');
        assert_eq!(plugboard.swap('Z').unwrap(), 'Z');
    }

    #[test]
    fn test_swap_invalid_characters() {
        let pairs = vec![('A', 'B')];
        let plugboard = Plugboard::new(Some(pairs), None).unwrap();

        // Test non-uppercase ASCII
        assert_eq!(
            plugboard.swap('a'),
            Err("Invalid character: Must be an ASCII uppercase letter")
        );
        assert_eq!(
            plugboard.swap('1'),
            Err("Invalid character: Must be an ASCII uppercase letter")
        );
        assert_eq!(
            plugboard.swap(' '),
            Err("Invalid character: Must be an ASCII uppercase letter")
        );
        assert_eq!(
            plugboard.swap('Å'),
            Err("Invalid character: Must be an ASCII uppercase letter")
        );
    }

    #[test]
    fn test_validate_plugboard_pairs() {
        // Test valid pairs
        assert!(Plugboard::validate_plugboard_pairs(&[('A', 'B'), ('C', 'D')]).is_ok());

        // Test invalid characters
        assert_eq!(
            Plugboard::validate_plugboard_pairs(&[('A', 'b')]),
            Err("Invalid character in plugboard pairs: Must be ASCII uppercase letters")
        );

        // Test duplicate characters
        assert_eq!(
            Plugboard::validate_plugboard_pairs(&[('A', 'B'), ('B', 'C')]),
            Err("Duplicate character in plugboard pairs")
        );

        // Test self-mapping
        assert_eq!(
            Plugboard::validate_plugboard_pairs(&[('A', 'A')]),
            Err("Self-mapping is not allowed in plugboard pairs")
        );
    }

    #[test]
    fn test_random_plugboard_validity() {
        // Test that randomly generated plugboard follows proper rules
        let plugboard = Plugboard::new(None, Some(12345)).unwrap();

        let mut mapped_chars = std::collections::HashSet::new();
        let mut has_mapping = false;

        for c in 'A'..='Z' {
            let swapped = plugboard.swap(c).unwrap();

            // Verify reciprocal mapping
            if swapped != c {
                has_mapping = true;
                assert_eq!(plugboard.swap(swapped).unwrap(), c);

                // Verify no character is mapped to itself
                assert_ne!(swapped, c);

                // Track mapped characters
                mapped_chars.insert(c);
                mapped_chars.insert(swapped);
            }
        }

        // Verify at least some mappings were created
        assert!(has_mapping, "Random plugboard should have some mappings");

        // Verify all mappings are unique
        assert_eq!(mapped_chars.len(), mapped_chars.iter().count());
    }
}

use rand::rngs::StdRng;
use rand::{seq::SliceRandom, SeedableRng};
use std::collections::HashMap;

/// Represents a plugboard in the Enigma machine.
#[derive(Debug)]
pub struct Plugboard {
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
    /// # Errors
    /// Returns an error in the following cases:
    /// - Any character in the pairs is not a valid ASCII uppercase letter.
    /// - A character is mapped more than once (duplicate mapping).
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
    pub fn swap(&self, c: char) -> Result<char, &'static str> {
        if !c.is_ascii_uppercase() {
            return Err("Invalid character: Must be an ASCII uppercase letter");
        }
        Ok(*self.mapping.get(&c).unwrap_or(&c))
    }

    /// Validates the plugboard pairs.
    pub fn validate_plugboard_pairs(pairs: &[(char, char)]) -> Result<(), &'static str> {
        let mut used_chars = std::collections::HashSet::new();
        for (a, b) in pairs {
            if !a.is_ascii_uppercase() || !b.is_ascii_uppercase() {
                return Err(
                    "Invalid character in plugboard pairs: Must be ASCII uppercase letters",
                );
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

use rand::rngs::StdRng;
use rand::{seq::SliceRandom, SeedableRng};

/// Represents a reflector in the Enigma machine.
#[derive(Debug)]
pub struct Reflector {
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
    /// # Errors
    /// Returns an error if the `wiring` string does not have exactly 26 characters (if `seed` is `None`).
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
    pub fn reflect(&self, c: char) -> Result<char, &'static str> {
        if !c.is_ascii_uppercase() {
            return Err("Invalid character: Must be an ASCII uppercase letter");
        }
        let index = (c as u8 - b'A') as usize;
        Ok(self.mapping[index])
    }
}

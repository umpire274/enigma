use rand::rngs::StdRng;
use rand::{seq::SliceRandom, SeedableRng};

/// Represents a rotor in the Enigma machine.
///
/// A rotor is a core component of the Enigma machine that performs a substitution cipher.
/// It has a mapping of characters for encryption in the forward and reverse directions,
/// a notch that triggers the next rotor to rotate, and a current position.
#[derive(Debug, PartialEq)]
pub struct Rotor {
    /// The mapping of characters for encryption in the forward direction.
    pub mapping: [char; 26],

    /// The mapping of characters for encryption in the reverse direction.
    pub reverse_mapping: [char; 26],

    /// The character at which the rotor triggers the next rotor to rotate.
    pub notch: char,

    /// The current position of the rotor (0-25).
    pub position: usize,
}

impl Rotor {
    /// Creates a new rotor with the specified wiring, notch, and initial position.
    ///
    /// If `seed` is provided, the wiring is generated randomly based on the seed.
    /// Otherwise, the provided `wiring` is used.
    ///
    /// # Arguments
    /// * `wiring` - A string of 26 characters representing the rotor's substitution mapping.
    ///              If `seed` is provided, this parameter is ignored.
    /// * `notch` - The character at which the rotor triggers the next rotor to rotate.
    /// * `position` - The initial position of the rotor (must be an ASCII uppercase letter).
    /// * `seed` - An optional seed for generating random wiring.
    ///
    /// # Returns
    /// - `Ok(Self)`: The rotor instance.
    /// - `Err(&'static str)`: An error if the wiring, notch, or position is invalid.
    ///
    /// # Example
    /// ```rust
    /// let rotor = Rotor::new(Some("EKMFLGDQVZNTOWYHXUSPAIBRCJ"), 'Q', 'A', None)?;
    /// ```
    pub fn new(
        wiring: Option<&str>,
        notch: char,
        position: char,
        seed: Option<u64>,
    ) -> Result<Self, &'static str> {
        if notch < 'A' || notch > 'Z' || position < 'A' || position > 'Z' {
            return Err("The notch and position must be valid ASCII uppercase letters ('A'-'Z')");
        }

        let position_offset = (position as u8 - b'A') as usize;

        // Generate wiring if a seed is provided
        let wiring = match seed {
            Some(seed) => {
                let mut rng = StdRng::seed_from_u64(seed);
                let mut alphabet: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
                alphabet.shuffle(&mut rng);
                alphabet.into_iter().collect()
            }
            None => {
                if let Some(wiring) = wiring {
                    if wiring.len() != 26 {
                        return Err("The rotor wiring must have exactly 26 characters");
                    }
                    wiring.to_string()
                } else {
                    return Err("Either wiring or seed must be provided");
                }
            }
        };

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

    /// Encrypts a character in the forward direction (right to left).
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
        // Calcola se la prossima rotazione PORTERÀ SULLA tacca
        let will_be_at_notch = (self.position + 1) % 26 == (self.notch as u8 - b'A') as usize;

        // Avanza la posizione
        self.position = (self.position + 1) % 26;

        // Restituisce true se STA PER PASSARE la tacca
        will_be_at_notch
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
    #[allow(dead_code)]
    pub fn get_current_letter(&self) -> char {
        self.mapping[self.position] // Return the current letter
    }

    /// Returns the current position of the rotor as an index (0-25)
    #[allow(dead_code)]
    pub fn current_position(&self) -> usize {
        (self.position as u8 - b'A') as usize
    }

    #[allow(dead_code)]
    pub fn set_position_before_notch(&mut self) {
        let notch_pos = (self.notch as u8 - b'A') as usize;
        self.position = notch_pos.checked_sub(1).unwrap_or(25);
    }

    /// Creates the reverse mapping from the forward wiring
    #[allow(dead_code)]
    pub fn create_reverse_mapping(wiring: &str) -> [char; 26] {
        let mut reverse = ['A'; 26];
        for (i, c) in wiring.chars().enumerate() {
            let pos = (c as u8 - b'A') as usize;
            reverse[pos] = (b'A' + i as u8) as char;
        }
        reverse
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_basic() {
        let mut rotor = Rotor {
            mapping: ['A'; 26],
            reverse_mapping: ['A'; 26],
            notch: 'B',  // Tacca in posizione 1
            position: 0, // 'A'
        };
        assert!(rotor.rotate()); // 'A' → 'B' (dovrebbe segnalare la tacca)
        assert_eq!(rotor.position, 1);
    }

    #[test]
    fn test_rotor_creation_with_wiring() {
        let wiring = "EKMFLGDQVZNTOWYHXUSPAIBRCJ";
        let rotor = Rotor::new(Some(wiring), 'Q', 'A', None).unwrap();

        // Verifica che il wiring sia corretto
        assert_eq!(
            rotor.mapping,
            [
                'E', 'K', 'M', 'F', 'L', 'G', 'D', 'Q', 'V', 'Z', 'N', 'T', 'O', 'W', 'Y', 'H',
                'X', 'U', 'S', 'P', 'A', 'I', 'B', 'R', 'C', 'J'
            ]
        );

        // Verifica che il reverse wiring sia corretto
        assert_eq!(
            rotor.reverse_mapping,
            [
                'U', 'W', 'Y', 'G', 'A', 'D', 'F', 'P', 'V', 'Z', 'B', 'E', 'C', 'K', 'M', 'T',
                'H', 'X', 'S', 'L', 'R', 'I', 'N', 'Q', 'O', 'J'
            ]
        );

        // Verifica la posizione iniziale
        assert_eq!(rotor.position, 0); // 'A' corrisponde a 0
    }

    #[test]
    fn test_rotor_creation_with_seed() {
        let seed = 12345;
        let rotor = Rotor::new(None, 'Q', 'A', Some(seed)).unwrap();

        // Verifica che il wiring sia una permutazione valida dell'alfabeto
        let mut sorted_mapping = rotor.mapping.to_vec();
        sorted_mapping.sort();
        assert_eq!(
            sorted_mapping,
            [
                'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
                'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
            ]
        );
    }

    #[test]
    fn test_forward_encryption() {
        let wiring = "EKMFLGDQVZNTOWYHXUSPAIBRCJ";
        let rotor = Rotor::new(Some(wiring), 'Q', 'A', None).unwrap();

        // Verifica la cifratura in avanti
        assert_eq!(rotor.forward('A').unwrap(), 'E');
        assert_eq!(rotor.forward('B').unwrap(), 'K');
        assert_eq!(rotor.forward('C').unwrap(), 'M');
    }

    #[test]
    fn test_reverse_encryption() {
        let wiring = "EKMFLGDQVZNTOWYHXUSPAIBRCJ";
        let rotor = Rotor::new(Some(wiring), 'Q', 'A', None).unwrap();

        // Verifica la cifratura all'indietro
        assert_eq!(rotor.reverse('E').unwrap(), 'A');
        assert_eq!(rotor.reverse('K').unwrap(), 'B');
        assert_eq!(rotor.reverse('M').unwrap(), 'C');
    }

    #[test]
    fn test_rotor_rotation() {
        let wiring = "EKMFLGDQVZNTOWYHXUSPAIBRCJ";
        let reverse = Rotor::create_reverse_mapping(wiring);
        let mut rotor = Rotor {
            mapping: wiring.chars().collect::<Vec<_>>().try_into().unwrap(),
            reverse_mapping: reverse,
            notch: 'Q',  // Tacca in posizione 16 ('Q')
            position: 0, // Posizione iniziale: 'A' (0)
        };

        // Ruota 15 volte (da 'A' a 'P')
        for _ in 0..15 {
            assert!(!rotor.rotate(), "Non dovrebbe segnalare la tacca");
        }

        // 16° rotazione (da 'P' a 'Q') - dovrebbe segnalare la tacca
        assert!(
            rotor.rotate(),
            "Dovrebbe segnalare il passaggio della tacca"
        );
        assert_eq!(rotor.position, 16); // 'Q'

        // Rotazione successiva (da 'Q' a 'R') - non dovrebbe segnalare
        assert!(!rotor.rotate(), "Non dovrebbe segnalare la tacca");
    }

    #[test]
    fn test_get_current_letter() {
        let wiring = "EKMFLGDQVZNTOWYHXUSPAIBRCJ";
        let rotor = Rotor::new(Some(wiring), 'Q', 'A', None).unwrap();

        // Verifica la lettera corrente
        assert_eq!(rotor.get_current_letter(), 'E'); // Posizione iniziale 'A' -> 'E'
    }

    #[test]
    fn test_invalid_wiring_length() {
        let wiring = "EKMFLGDQVZNTOWYHXUSPAIBRC"; // 25 caratteri (invalido)
        let result = Rotor::new(Some(wiring), 'Q', 'A', None);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "The rotor wiring must have exactly 26 characters"
        );
    }

    #[test]
    fn test_invalid_notch_or_position() {
        let wiring = "EKMFLGDQVZNTOWYHXUSPAIBRCJ";
        let result = Rotor::new(Some(wiring), 'q', 'A', None); // Notch non valido

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "The notch and position must be valid ASCII uppercase letters ('A'-'Z')"
        );
    }

    #[test]
    fn test_rotate_with_notch_at_a() {
        let wiring = "EKMFLGDQVZNTOWYHXUSPAIBRCJ";
        let reverse = Rotor::create_reverse_mapping(wiring);
        let mut rotor = Rotor {
            mapping: wiring.chars().collect::<Vec<_>>().try_into().unwrap(),
            reverse_mapping: reverse,
            notch: 'A',   // Tacca in posizione 0
            position: 25, // Partiamo da 'Z' (posizione 25)
        };

        // Rotazione 1: 'Z' → 'A' - deve segnalare il passaggio della tacca
        assert!(rotor.rotate(), "Dovrebbe segnalare passaggio tacca 'A'");
        assert_eq!(rotor.position, 0);

        // Rotazione 2: 'A' → 'B' - nessun passaggio tacca
        assert!(!rotor.rotate(), "Non dovrebbe segnalare passaggio tacca");
        assert_eq!(rotor.position, 1);
    }

    #[test]
    fn test_rotate_with_notch_at_b() {
        let mut rotor = Rotor {
            mapping: ['A'; 26],
            reverse_mapping: ['A'; 26],
            notch: 'B',  // Notch at position 1
            position: 0, // Starting at 'A'
        };

        // First rotation to 'B' - should trigger notch
        assert!(rotor.rotate());
        assert_eq!(rotor.position, 1);

        // Next rotation shouldn't trigger
        assert!(!rotor.rotate());
        assert_eq!(rotor.position, 2);
    }

    #[test]
    fn test_full_rotation() {
        let mut rotor = Rotor {
            mapping: ['A'; 26],
            reverse_mapping: ['A'; 26],
            notch: 'A',
            position: 0,
        };

        // Rotate 25 times (shouldn't trigger notch)
        for _ in 0..25 {
            assert!(!rotor.rotate());
        }

        // 26th rotation (back to 'A') - should trigger
        assert!(rotor.rotate());
        assert_eq!(rotor.position, 0);
    }

    #[test]
    fn test_notch_behavior_all_positions() {
        for notch_char in 'A'..='Z' {
            let notch_pos = (notch_char as u8 - b'A') as usize;
            let prev_pos = if notch_pos == 0 { 25 } else { notch_pos - 1 };

            let mut rotor = Rotor {
                mapping: ['A'; 26],
                reverse_mapping: ['A'; 26],
                notch: notch_char,
                position: prev_pos,
            };

            // Rotazione dovrebbe segnalare il passaggio della tacca
            assert!(rotor.rotate(), "Failed for notch at {}", notch_char);
            assert_eq!(rotor.position, notch_pos);
        }
    }

    #[test]
    fn test_notch_at_a() {
        let mut rotor = Rotor {
            mapping: ['A'; 26],
            reverse_mapping: ['A'; 26],
            notch: 'A',
            position: 25, // 'Z'
        };

        // Rotazione da 'Z' a 'A' - dovrebbe segnalare
        assert!(rotor.rotate());
        assert_eq!(rotor.position, 0);
    }

    #[test]
    fn test_multiple_rotations() {
        let mut rotor = Rotor {
            mapping: ['A'; 26],
            reverse_mapping: ['A'; 26],
            notch: 'C', // Posizione 2
            position: 0,
        };

        assert!(!rotor.rotate()); // A→B
        assert!(rotor.rotate()); // B→C (segnala)
        assert!(!rotor.rotate()); // C→D
    }
}

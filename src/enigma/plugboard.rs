use std::collections::HashMap;

/// Represents a plugboard in the Enigma machine.
///
/// The plugboard is a component that swaps pairs of letters before and after they pass through
/// the rotors. It enhances the security of the encryption by adding an extra layer of substitution.
///
/// # Fields
/// - `mapping`: A `HashMap` that stores the character swaps. Each key-value pair represents a swap.
#[derive(Debug)]
pub struct Plugboard {
    mapping: HashMap<char, char>,
}

impl Plugboard {
    /// Creates a new plugboard with the specified character pairs.
    ///
    /// The plugboard swaps each pair of characters provided in the `pairs` vector. Each character
    /// must be a valid ASCII uppercase letter (`A-Z`), and no character can be mapped more than once.
    ///
    /// # Arguments
    /// * `pairs` - A vector of character pairs to swap. Each pair is represented as a tuple `(char, char)`.
    ///
    /// # Errors
    /// Returns an error in the following cases:
    /// - Any character in the pairs is not a valid ASCII uppercase letter.
    /// - A character is mapped more than once (duplicate mapping).
    ///
    /// # Example
    /// ```rust
    /// let plugboard = Plugboard::new(vec![('A', 'B'), ('C', 'D')])?;
    /// println!("Plugboard created: {:?}", plugboard);
    /// ```
    pub fn new(pairs: Vec<(char, char)>) -> Result<Self, &'static str> {
        let mut mapping = HashMap::new();

        for (a, b) in pairs {
            // Verify that the characters are valid
            if !a.is_ascii_uppercase() || !b.is_ascii_uppercase() {
                return Err("Invalid characters: Must be ASCII uppercase letters");
            }

            // Verify that there are no duplicate mappings
            if mapping.contains_key(&a) || mapping.contains_key(&b) {
                return Err("Duplicate mapping: A character cannot be mapped more than once");
            }

            // Insert the mappings
            mapping.insert(a, b);
            mapping.insert(b, a);
        }

        Ok(Self { mapping })
    }

    /// Swaps a character based on the plugboard's mapping.
    ///
    /// If the character has a mapping in the plugboard, it is swapped with its corresponding pair.
    /// If no mapping exists, the character is returned unchanged.
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
    /// println!("Swapped: {}", swapped_char); // Output: 'B' if 'A' is mapped to 'B'
    /// ```
    pub fn swap(&self, c: char) -> Result<char, &'static str> {
        if !c.is_ascii_uppercase() {
            return Err("Invalid character: Must be an ASCII uppercase letter");
        }
        Ok(*self.mapping.get(&c).unwrap_or(&c))
    }
}

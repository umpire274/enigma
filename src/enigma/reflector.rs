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

	/// Riflette un carattere.
	///
	/// # Argomenti
	/// * `c` - Il carattere da riflettere.
	///
	/// # Restituisce
	/// Il carattere riflesso, o un errore se il carattere non è valido.
	pub fn reflect(&self, c: char) -> Result<char, &'static str> {
		if !c.is_ascii_uppercase() {
			return Err("Carattere non valido: deve essere una lettera maiuscola ASCII");
		}
		let index = (c as u8 - b'A') as usize;
		Ok(self.mapping[index])
	}
}

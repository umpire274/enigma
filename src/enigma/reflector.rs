#[derive(Debug)]
pub struct Reflector {
	mapping: [char; 26], // Usa un array fisso invece di un Vec
}

impl Reflector {
	/// Crea un nuovo riflettore.
	///
	/// # Argomenti
	/// * `wiring` - Una stringa di 26 caratteri che rappresenta la mappatura del riflettore.
	///
	/// # Errori
	/// Restituisce un errore se la `wiring` non ha 26 caratteri.
	pub fn new(wiring: &str) -> Result<Self, &'static str> {
		if wiring.len() != 26 {
			return Err("La mappatura del riflettore deve avere 26 caratteri");
		}

		let mut mapping = ['A'; 26]; // Array fisso di 26 caratteri

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
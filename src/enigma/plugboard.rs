use std::collections::HashMap;

#[derive(Debug)]
pub struct Plugboard {
	mapping: HashMap<char, char>,
}

impl Plugboard {
	/// Crea un nuovo plugboard.
	///
	/// # Argomenti
	/// * `pairs` - Un vettore di coppie di caratteri da scambiare.
	///
	/// # Errori
	/// Restituisce un errore se i caratteri non sono validi o se ci sono mappature duplicate.
	pub fn new(pairs: Vec<(char, char)>) -> Result<Self, &'static str> {
		let mut mapping = HashMap::new();

		for (a, b) in pairs {
			// Verifica che i caratteri siano validi
			if !a.is_ascii_uppercase() || !b.is_ascii_uppercase() {
				return Err("Caratteri non validi: devono essere lettere maiuscole ASCII");
			}

			// Verifica che non ci siano mappature duplicate
			if mapping.contains_key(&a) || mapping.contains_key(&b) {
				return Err("Mappatura duplicata: un carattere non può essere mappato più di una volta");
			}

			// Inserisci le mappature
			mapping.insert(a, b);
			mapping.insert(b, a);
		}

		Ok(Self { mapping })
	}

	/// Scambia un carattere in base alla mappatura del plugboard.
	///
	/// # Argomenti
	/// * `c` - Il carattere da scambiare.
	///
	/// # Restituisce
	/// Il carattere scambiato, o un errore se il carattere non è valido.
	pub fn swap(&self, c: char) -> Result<char, &'static str> {
		if !c.is_ascii_uppercase() {
			return Err("Carattere non valido: deve essere una lettera maiuscola ASCII");
		}
		Ok(*self.mapping.get(&c).unwrap_or(&c))
	}
}
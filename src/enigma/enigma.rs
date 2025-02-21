use super::{rotor::Rotor, reflector::Reflector, plugboard::Plugboard};
use std::fs;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
	rotors: Vec<String>,
	notches: Vec<char>,
	reflector: String,
	plugboard_pairs: Vec<(char, char)>,
}

pub struct EnigmaMachine {
	rotors: Vec<Rotor>,
	reflector: Reflector,
	plugboard: Plugboard,
}

impl EnigmaMachine {
	pub fn from_config(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
		let config_str = fs::read_to_string(file_path)?;
		let config: Config = serde_json::from_str(&config_str)?;

		let rotors = config.rotors.iter().zip(&config.notches)
			.map(|(wiring, &notch)| Rotor::new(wiring, notch, 'A'))
			.collect();

		let reflector = Reflector::new(&config.reflector);
		let plugboard = Plugboard::new(config.plugboard_pairs);

		Ok(Self { rotors, reflector, plugboard })
	}

	pub fn encrypt(&self, message: &str) -> String {
		message.chars().map(|c| {
			let mut c = self.plugboard.swap(c);
			for rotor in &self.rotors {
				c = rotor.forward(c);
			}
			c = self.reflector.reflect(c);
			for rotor in self.rotors.iter().rev() {
				c = rotor.reverse(c);
			}
			self.plugboard.swap(c)
		}).collect()
	}

	// Funzione per formattare l'output in quartine separate da '-'
	fn format_output(&self, text: &str) -> String {
		text.chars()
			.filter(|c| c.is_ascii_uppercase()) // Rimuove spazi e caratteri non validi
			.collect::<Vec<_>>() // Converte in un vettore di caratteri
			.chunks(4) // Divide il testo in quartine di 4 lettere
			.map(|chunk| chunk.iter().collect::<String>()) // Trasforma ogni gruppo in stringa
			.collect::<Vec<_>>() // Converte tutto in un vettore di stringhe
			.join("-") // Unisce le quartine con '-'
	}

	pub fn encrypt_message(&self, text: &str) -> String {
		let encrypted_text = self.encrypt(text); // Usa la funzione di cifratura già esistente
		self.format_output(&encrypted_text) // Formatta l'output
	}

}

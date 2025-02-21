use super::{plugboard::Plugboard, reflector::Reflector, rotor::Rotor};
use serde::Deserialize;
use std::fs;

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

        let rotors = config
            .rotors
            .iter()
            .zip(&config.notches)
            .map(|(wiring, &notch)| Rotor::new(wiring, notch, 'A'))
            .collect();

        let reflector = Reflector::new(&config.reflector);
        let plugboard = Plugboard::new(config.plugboard_pairs);

        Ok(Self {
            rotors,
            reflector,
            plugboard,
        })
    }

    pub fn encrypt(&self, message: &str) -> String {
        message
            .chars()
            .map(|c| {
                let mut c = self.plugboard.swap(c);
                for rotor in &self.rotors {
                    c = rotor.forward(c);
                }
                c = self.reflector.reflect(c);
                for rotor in self.rotors.iter().rev() {
                    c = rotor.reverse(c);
                }
                self.plugboard.swap(c)
            })
            .collect()
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

    pub fn encrypt_message(&mut self, text: &str) -> String {
        let mut result = String::new();
        for c in text.chars() {
            if c.is_ascii_alphabetic() {
                self.step_rotors(); // Ruota i rotori prima di cifrare
                result.push(self.encrypt(&c.to_string()).parse().unwrap()); // Cifra il carattere
            }
        }
        self.format_output(&result)
    }

    fn step_rotors(&mut self) {
        let mut rotate_next = true; // Il primo rotore ruota sempre
        for i in 0..self.rotors.len() {
            if rotate_next {
                rotate_next = self.rotors[i].rotate(); // Ruota il rotore corrente e controlla se deve passare il "notch" al prossimo
            } else {
                break; // Se un rotore non ruota, interrompiamo il processo
            }
        }
    }
}

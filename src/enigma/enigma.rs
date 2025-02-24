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
    /// Crea una nuova macchina Enigma da un file di configurazione.
    ///
    /// # Argomenti
    /// * `file_path` - Il percorso del file di configurazione.
    ///
    /// # Errori
    /// Restituisce un errore se il file non può essere letto o se la configurazione non è valida.
    pub fn from_config(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string(file_path)?;
        let config: Config = serde_json::from_str(&config_str)?;

        // Verifica che il numero di rotori e notches sia uguale
        if config.rotors.len() != config.notches.len() {
            return Err("Il numero di rotori e notches deve essere uguale".into());
        }

        let rotors = config
            .rotors
            .iter()
            .zip(&config.notches)
            .map(|(wiring, &notch)| Rotor::new(wiring, notch, 'A'))
            .collect::<Result<Vec<_>, _>>()?;

        let reflector = Reflector::new(&config.reflector)?;
        let plugboard = Plugboard::new(config.plugboard_pairs)?;

        Ok(Self {
            rotors,
            reflector,
            plugboard,
        })
    }

    /// Cifra un messaggio.
    ///
    /// # Argomenti
    /// * `message` - Il messaggio da cifrare.
    ///
    /// # Restituisce
    /// Il messaggio cifrato, o un errore se il messaggio contiene caratteri non validi.
    pub fn encrypt(&self, message: &str) -> Result<String, &'static str> {
        message
            .chars()
            .map(|c| {
                if !c.is_ascii_uppercase() {
                    return Err("Carattere non valido: deve essere una lettera maiuscola ASCII");
                }
                let mut c = self.plugboard.swap(c)?;
                for rotor in &self.rotors {
                    c = rotor.forward(c)?;
                }
                c = self.reflector.reflect(c)?;
                for rotor in self.rotors.iter().rev() {
                    c = rotor.reverse(c)?;
                }
                self.plugboard.swap(c)
            })
            .collect()
    }

    /// Formatta l'output in quartine separate da '-'.
    fn format_output(&self, text: &str) -> String {
        text.chars()
            .filter(|c| c.is_ascii_uppercase()) // Rimuove spazi e caratteri non validi
            .collect::<Vec<_>>() // Converte in un vettore di caratteri
            .chunks(4) // Divide il testo in quartine di 4 lettere
            .map(|chunk| chunk.iter().collect::<String>()) // Trasforma ogni gruppo in stringa
            .collect::<Vec<_>>() // Converte tutto in un vettore di stringhe
            .join("-") // Unisce le quartine con '-'
    }

    /// Cifra un messaggio e formatta l'output.
    ///
    /// # Argomenti
    /// * `text` - Il messaggio da cifrare.
    ///
    /// # Restituisce
    /// Il messaggio cifrato formattato, o un errore se il messaggio contiene caratteri non validi.
    pub fn encrypt_message(&mut self, text: &str) -> Result<String, &'static str> {
        let mut result = String::new();
        for c in text.chars() {
            if c.is_ascii_alphabetic() {
                self.step_rotors(); // Ruota i rotori prima di cifrare
                let encrypted_char = self.encrypt(&c.to_ascii_uppercase().to_string())?;
                result.push(encrypted_char.chars().next().unwrap());
            }
        }
        Ok(self.format_output(&result))
    }

    /// Ruota i rotori in base alle loro posizioni e notches.
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
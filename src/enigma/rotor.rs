#[derive(Debug)]
pub struct Rotor {
    pub mapping: Vec<char>,
    pub reverse_mapping: Vec<char>,
    pub notch: char,     // Il carattere della posizione in cui il rotore scatta
    pub position: usize, // Posizione attuale del rotore (0-25)
}

impl Rotor {
    pub fn new(wiring: &str, notch: char, position: char) -> Self {
        let position_offset = position as u8 - b'A';

        let mapping: Vec<char> = wiring.chars().collect();
        let mut reverse_mapping = vec!['A'; 26];

        for (i, &c) in mapping.iter().enumerate() {
            if let Some(index) = (c as u8).checked_sub(b'A') {
                reverse_mapping[index as usize] = (b'A' + i as u8) as char;
            }
        }

        Self {
            mapping,
            reverse_mapping,
            notch,
            position: position_offset as usize,
        }
    }

    pub fn forward(&self, c: char) -> char {
        if !c.is_ascii_uppercase() {
            return c; // Evita caratteri non validi
        }
        let index = (c as u8 - b'A') as usize;
        self.mapping[index]
    }

    pub fn reverse(&self, c: char) -> char {
        if !c.is_ascii_uppercase() {
            return c;
        }
        let index = (c as u8 - b'A') as usize;
        self.reverse_mapping[index]
    }

    pub fn rotate(&mut self) -> bool {
        self.position = (self.position + 1) % 26; // Avanza di 1 posizione
        self.get_current_letter() == self.notch // Ritorna true se il rotore è sul notch
    }

    pub fn get_current_letter(&self) -> char {
        self.mapping[self.position] // Restituisce la lettera attuale del rotore
    }
}

#[derive(Debug)]
pub struct Rotor {
    pub mapping: [char; 26],
    pub reverse_mapping: [char; 26],
    pub notch: char,     // Il carattere della posizione in cui il rotore scatta
    pub position: usize, // Posizione attuale del rotore (0-25)
}

impl Rotor {
    /// Crea un nuovo rotore.
    ///
    /// # Argomenti
    /// * `wiring` - Una stringa di 26 caratteri che rappresenta la mappatura del rotore.
    /// * `notch` - Il carattere che causa la rotazione del rotore successivo.
    /// * `position` - La posizione iniziale del rotore.
    ///
    /// # Errori
    /// Restituisce un errore se la `wiring` non ha 26 caratteri o se `notch`/`position` non sono validi.
    pub fn new(wiring: &str, notch: char, position: char) -> Result<Self, &'static str> {
        if wiring.len() != 26 {
            return Err("La mappatura del rotore deve avere 26 caratteri");
        }
        if notch < 'A' || notch > 'Z' || position < 'A' || position > 'Z' {
            return Err("Il notch e la posizione devono essere caratteri validi ('A'-'Z')");
        }

        let position_offset = (position as u8 - b'A') as usize;

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

    ///
    /// # Argomenti
    /// * `c` - Il carattere da codificare.
    ///
    /// # Restituisce
    /// Il carattere codificato, o un errore se il carattere non è valido.
    pub fn forward(&self, c: char) -> Result<char, &'static str> {
        if !c.is_ascii_uppercase() {
            return Err("Carattere non valido: deve essere una lettera maiuscola ASCII");
        }
        let index = (c as u8 - b'A') as usize;
        Ok(self.mapping[index])
    }

    /// Codifica un carattere all'indietro (da sinistra a destra).
    ///
    /// # Argomenti
    /// * `c` - Il carattere da codificare.
    ///
    /// # Restituisce
    /// Il carattere codificato, o un errore se il carattere non è valido.
    pub fn reverse(&self, c: char) -> Result<char, &'static str> {
        if !c.is_ascii_uppercase() {
            return Err("Carattere non valido: deve essere una lettera maiuscola ASCII");
        }
        let index = (c as u8 - b'A') as usize;
        Ok(self.reverse_mapping[index])
    }

    /// Ruota il rotore di una posizione.
    ///
    /// # Restituisce
    /// `true` se il rotore è sul notch dopo la rotazione, altrimenti `false`.
    pub fn rotate(&mut self) -> bool {
        self.position = (self.position + 1) % 26; // Avanza di 1 posizione
        self.get_current_letter() == self.notch // Ritorna true se il rotore è sul notch
    }

    /// Restituisce la lettera corrente del rotore.
    pub fn get_current_letter(&self) -> char {
        self.mapping[self.position] // Restituisce la lettera attuale del rotore
    }
}
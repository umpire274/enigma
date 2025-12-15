const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const ALPHABET_LEN: u8 = 36;

pub fn char_to_symbol(b: u8) -> Option<u8> {
    let b = b.to_ascii_uppercase();
    ALPHABET.iter().position(|&c| c == b).map(|i| i as u8)
}

pub fn symbol_to_char(s: u8) -> u8 {
    ALPHABET[(s % ALPHABET_LEN) as usize]
}

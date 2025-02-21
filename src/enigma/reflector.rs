#[derive(Debug)]
pub struct Reflector {
	mapping: Vec<char>,
}

impl Reflector {
	pub fn new(wiring: &str) -> Self {
		let mut mapping = vec!['A'; 26];

		for (i, c) in wiring.chars().enumerate() {
			if let Some(index) = (c as u8).checked_sub(b'A') {
				mapping[index as usize] = (b'A' + i as u8) as char;
			}
		}

		Self { mapping }
	}

	pub fn reflect(&self, c: char) -> char {
		if !c.is_ascii_uppercase() {
			return c; // Evita caratteri non validi
		}
		let index = (c as u8 - b'A') as usize;
		self.mapping[index]
	}
}

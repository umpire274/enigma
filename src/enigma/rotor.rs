#[derive(Debug)]
pub struct Rotor {
	mapping: Vec<char>,
	reverse_mapping: Vec<char>,
	notch: char,
	position: usize,
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
}

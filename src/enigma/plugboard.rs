use std::collections::HashMap;

#[derive(Debug)]
pub struct Plugboard {
	mapping: HashMap<char, char>,
}

impl Plugboard {
	pub fn new(pairs: Vec<(char, char)>) -> Self {
		let mut mapping = HashMap::new();
		for (a, b) in pairs {
			mapping.insert(a, b);
			mapping.insert(b, a);
		}
		Self { mapping }
	}

	pub fn swap(&self, c: char) -> char {
		*self.mapping.get(&c).unwrap_or(&c)
	}
}

use std::io::{self, Write};

pub fn get_user_input(prompt: &str) -> String {
	print!("{}", prompt);
	io::stdout().flush().unwrap();

	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();
	input.trim().to_uppercase() // Convertiamo tutto in maiuscolo
}

pub fn display_output(encrypted_text: &str) {
	println!("Testo cifrato: {}", encrypted_text);
}

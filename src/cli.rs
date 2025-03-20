use std::io::{self, Write};

/// Prompts the user for input and returns the entered text.
///
/// # Arguments
/// * `prompt` - The message displayed to the user as a prompt.
///
/// # Returns
/// The user's input as a `String`, trimmed of leading and trailing whitespace.
pub fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Ensures the prompt is printed immediately

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap(); // Reads the user's input

    // Removes leading and trailing whitespace, newlines, and carriage returns
    input.trim().to_string()
}

/// Displays the result of an encryption operation.
///
/// # Arguments
/// * `encrypted_text` - The encrypted text to display.
pub fn display_output(encrypted_text: &str) {
    println!("Encrypted text: {}", encrypted_text);
}

/// Preprocesses the input by converting numbers to a prefixed sequence and keeping letters in uppercase.
///
/// # Arguments
/// * `input` - The input string to preprocess.
///
/// # Returns
/// The preprocessed string.
pub fn preprocess_input(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_digit() {
                // Convert numbers to a prefixed sequence
                let num = c.to_digit(10).unwrap() as u8;
                let letter = (b'Z' - num) as char; // Use the formula to find the corresponding letter
                format!("X{}", letter)
            } else {
                // Keep original letters in uppercase
                c.to_ascii_uppercase().to_string()
            }
        })
        .collect()
}

/// Postprocesses the output by converting prefixed sequences back to numbers.
///
/// # Arguments
/// * `output` - The output string to postprocess.
///
/// # Returns
/// The postprocessed string.
pub fn postprocess_output(output: &str) -> String {
    let mut result = String::new();
    let mut chars = output.chars().peekable();

    while let Some(c) = chars.next() {
        if c == 'X' {
            // If we find an 'X', the next character represents a number
            if let Some(next_c) = chars.next() {
                if next_c >= 'R' && next_c <= 'Z' {
                    // Use the formula to find the corresponding number
                    let num = (b'Z' - next_c as u8).to_string();
                    result.push_str(&num);
                } else {
                    // If it's not a valid letter, keep the original characters
                    result.push('X');
                    result.push(next_c);
                }
            }
        } else {
            // Keep original letters
            result.push(c);
        }
    }

    result
}

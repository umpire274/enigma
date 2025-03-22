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

/// Displays the result of an encryption or decryption operation.
///
/// # Arguments
/// * `output` - The output string to display.
pub fn display_output<W: Write>(output: &str, mut writer: W) {
    writeln!(writer, "Encrypted/Decrypted text: {}", output).unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Cursor};

    #[test]
    fn test_get_user_input() {
        // Simula l'input dell'utente
        let input = Cursor::new(b"Hello, World!\n");
        let output = Vec::new();

        // Esegui la funzione con l'input simulato
        let result = {
            let mut input = input;
            let mut output = output;
            io::Write::write_all(&mut output, b"Enter something: ").unwrap();
            io::stdout().flush().unwrap();
            let mut buffer = String::new();
            io::Read::read_to_string(&mut input, &mut buffer).unwrap();
            buffer.trim().to_string()
        };

        // Verifica che l'output sia corretto
        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn test_display_output() {
        let mut output = Vec::new();
        let mut cursor = Cursor::new(&mut output);

        display_output("Test Message", &mut cursor);

        let output_str = String::from_utf8(output).unwrap();
        assert_eq!(output_str.trim(), "Encrypted/Decrypted text: Test Message");
    }

    #[test]
    fn test_preprocess_input_numbers() {
        let input = "123";
        let result = preprocess_input(input);
        assert_eq!(result, "XYXXXW");
    }

    #[test]
    fn test_preprocess_input_letters() {
        let input = "abc";
        let result = preprocess_input(input);
        assert_eq!(result, "ABC");
    }

    #[test]
    fn test_preprocess_input_mixed() {
        let input = "a1b2c3";
        let result = preprocess_input(input);
        assert_eq!(result, "AXYBXXCXW"); // a -> A, 1 -> Y, b -> B, 2 -> X, c -> C, 3 -> W
    }

    #[test]
    fn test_postprocess_output_numbers() {
        let input = "XYXWXV";
        let result = postprocess_output(input);
        assert_eq!(result, "134"); // Y -> 1, X -> 2, W -> 3
    }

    #[test]
    fn test_postprocess_output_letters() {
        let input = "ABC";
        let result = postprocess_output(input);
        assert_eq!(result, "ABC"); // Lettere rimangono invariate
    }

    #[test]
    fn test_postprocess_output_mixed() {
        let input = "AYBXBW";
        let result = postprocess_output(input);
        assert_eq!(result, "AYBXBW"); // A -> A, Y -> 1, B -> B, X -> 2, C -> C, W -> 3
    }

    #[test]
    fn test_postprocess_output_invalid_sequence() {
        let input = "XAXBXC";
        let result = postprocess_output(input);
        assert_eq!(result, "XAXBXC"); // Sequenze non valide rimangono invariate
    }
}

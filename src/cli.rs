use std::io::{self, Write};

/// Prompts the user for input and returns the entered text in uppercase.
///
/// # Arguments
/// * `prompt` - The message displayed to the user as a prompt.
///
/// # Returns
/// The user's input as a `String` in uppercase.
pub fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Assicura che il prompt venga stampato immediatamente

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap(); // Legge l'input dell'utente

    // Rimuove spazi, newline e carriage return all'inizio e alla fine
    input.trim().to_string()
}

/// Displays the result of an encryption operation.
///
/// This function takes a `Result<String, &str>` representing the encrypted text or an error
/// and prints the appropriate message to the console.
///
/// # Arguments
/// * `encrypted_text` - A `Result` containing either the encrypted text (`Ok`) or an error message (`Err`).
///
/// # Example
/// ```rust
/// let result = Ok("RFKTZ".to_string());
/// display_output(&result); // Prints: "Encrypted text: RFKTZ"
///
/// let error = Err("Invalid character");
/// display_output(&error); // Prints: "Error: Invalid character"
/// ```
pub fn display_output(encrypted_text: &str) {
    println!("Encrypted text: {}", encrypted_text);
}

pub fn preprocess_input(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_digit() {
                // Converti i numeri in sequenze con prefisso 'X'
                let num = c.to_digit(10).unwrap() as u8;
                let letter = (b'Z' - num) as char; // Usa la formula per trovare la lettera
                format!("X{}", letter)
            } else {
                // Mantieni le lettere originali (convertite in maiuscolo)
                c.to_ascii_uppercase().to_string()
            }
        })
        .collect()
}

pub fn postprocess_output(output: &str) -> String {
    let mut result = String::new();
    let mut chars = output.chars().peekable();

    while let Some(c) = chars.next() {
        if c == 'X' {
            // Se troviamo un 'X', la prossima lettera rappresenta un numero
            if let Some(next_c) = chars.next() {
                if next_c >= 'R' && next_c <= 'Z' {
                    // Usa la formula per trovare il numero
                    let num = (b'Z' - next_c as u8).to_string();
                    result.push_str(&num);
                } else {
                    // Se non è una lettera valida, mantieni i caratteri originali
                    result.push('X');
                    result.push(next_c);
                }
            }
        } else {
            // Mantieni le lettere originali
            result.push(c);
        }
    }

    result
}

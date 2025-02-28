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
pub fn display_output(encrypted_text: &Result<String, &str>) {
    match encrypted_text {
        Ok(text) => println!("Encrypted text: {}", text), // Print the encrypted text
        Err(err) => println!("Error: {}", err),           // Print the error message
    }
}

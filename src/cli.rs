use std::io::{self, Write};

/// Prompts the user for input and returns the entered text in uppercase.
///
/// This function displays a prompt to the user, reads their input from the standard input,
/// trims any leading or trailing whitespace, and converts the input to uppercase.
///
/// # Arguments
/// * `prompt` - The message displayed to the user as a prompt.
///
/// # Returns
/// The user's input as a `String` in uppercase.
///
/// # Example
/// ```rust
/// let user_input = get_user_input("Enter a message: ");
/// println!("You entered: {}", user_input);
/// ```
pub fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt); // Display the prompt
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap(); // Read the user's input
    input.trim().to_uppercase() // Trim whitespace and convert to uppercase
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

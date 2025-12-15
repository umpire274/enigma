//! Error types for the enigma-core crate.

use std::fmt;

/// Result type used throughout the enigma-core crate.
pub type EnigmaResult<T> = Result<T, EnigmaError>;

/// Errors that can occur while building or running an Enigma machine.
#[derive(Debug)]
pub enum EnigmaError {
    /// The machine configuration is invalid.
    InvalidConfiguration(String),

    /// The provided state is incompatible with the machine.
    InvalidState(String),

    /// A component failed to process input.
    ComponentError(String),

    /// A stepping strategy failed.
    SteppingError(String),
}

impl fmt::Display for EnigmaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnigmaError::InvalidConfiguration(msg) => {
                write!(f, "invalid configuration: {msg}")
            }
            EnigmaError::InvalidState(msg) => {
                write!(f, "invalid state: {msg}")
            }
            EnigmaError::ComponentError(msg) => {
                write!(f, "component error: {msg}")
            }
            EnigmaError::SteppingError(msg) => {
                write!(f, "stepping error: {msg}")
            }
        }
    }
}

impl std::error::Error for EnigmaError {}

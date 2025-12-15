//! Enigma-inspired transformation engine.
//!
//! This crate provides a modular, deterministic pipeline inspired by the
//! historical Enigma machine architecture.
//!
//! The focus is on explicit state management, composability, and testability.
//!
//! ⚠️ This crate does NOT provide cryptographic security guarantees.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod component;
pub mod error;
pub mod machine;
pub mod state;

// Core building blocks
pub mod plugboard;
pub mod rotor;
pub mod reflector;
pub mod stepping;

// Optional / future extensions
#[cfg(feature = "crypto")]
pub mod crypto;

// Public re-exports (stable surface)
pub use component::EnigmaComponent;
pub use error::{EnigmaError, EnigmaResult};
pub use machine::EnigmaMachine;
pub use state::EnigmaState;
pub use stepping::SteppingStrategy;

// Concrete components
pub use plugboard::Plugboard;
pub use reflector::Reflector;
pub use rotor::Rotor;

// Stepping strategies
pub use stepping::LinearStepping;

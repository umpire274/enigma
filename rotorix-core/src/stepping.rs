//! Stepping strategies for the Enigma machine.
//!
//! A stepping strategy defines how the Enigma state evolves after
//! each processed symbol.

use crate::state::EnigmaState;

/// Strategy that controls how the Enigma state advances.
///
/// Implementations must mutate only the provided `EnigmaState`
/// and must not keep internal mutable state.
pub trait SteppingStrategy {
    /// Advances the Enigma state by one step.
    ///
    /// This method is called exactly once after each processed symbol.
    ///
    /// # Errors
    ///
    /// Implementations may return an error if the state cannot be
    /// advanced (e.g. invalid configuration).
    fn step(&self, state: &mut EnigmaState) -> Result<(), String>;
}

/// A simple linear stepping strategy.
///
/// Each call increments the first rotor position and propagates
/// overflow to the next rotors (odometer-style).
pub struct LinearStepping {
    /// Modulus applied to each rotor position.
    pub modulus: u32,
}

impl LinearStepping {
    /// Creates a new `LinearStepping` strategy.
    pub fn new(modulus: u32) -> Self {
        Self { modulus }
    }
}

impl SteppingStrategy for LinearStepping {
    fn step(&self, state: &mut EnigmaState) -> Result<(), String> {
        if self.modulus == 0 {
            return Err("modulus must be greater than zero".into());
        }

        if state.rotor_positions.is_empty() {
            return Err("no rotors defined in state".into());
        }

        // Increment step counter
        state.step_counter += 1;

        // Odometer-style stepping
        for pos in &mut state.rotor_positions {
            *pos += 1;
            if *pos < self.modulus {
                break;
            }
            *pos = 0;
        }

        Ok(())
    }
}

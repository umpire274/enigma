//! Enigma machine orchestration.
//!
//! This module defines the `EnigmaMachine`, which wires together
//! components, state, and stepping strategy into a transformation pipeline.

use crate::{
    component::EnigmaComponent,
    error::{EnigmaError, EnigmaResult},
    state::EnigmaState,
    stepping::SteppingStrategy,
};

/// Core Enigma transformation machine.
///
/// The machine itself is stateless. All mutable data is contained
/// in the external `EnigmaState`.
pub struct EnigmaMachine {
    plugboard: Box<dyn EnigmaComponent>,
    rotors: Vec<Box<dyn EnigmaComponent>>,
    reflector: Box<dyn EnigmaComponent>,
    stepping: Box<dyn SteppingStrategy>,
}

impl EnigmaMachine {
    /// Creates a new `EnigmaMachine` from its components.
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration is invalid.
    pub fn new(
        plugboard: Box<dyn EnigmaComponent>,
        rotors: Vec<Box<dyn EnigmaComponent>>,
        reflector: Box<dyn EnigmaComponent>,
        stepping: Box<dyn SteppingStrategy>,
    ) -> EnigmaResult<Self> {
        if rotors.is_empty() {
            return Err(EnigmaError::InvalidConfiguration(
                "at least one rotor is required".into(),
            ));
        }

        Ok(Self {
            plugboard,
            rotors,
            reflector,
            stepping,
        })
    }

    /// Processes a single byte through the Enigma pipeline.
    ///
    /// The state is updated via the configured stepping strategy
    /// after the transformation.
    pub fn process_byte(
        &self,
        input: u8,
        state: &mut EnigmaState,
    ) -> EnigmaResult<u8> {
        if state.rotor_positions.len() != self.rotors.len() {
            return Err(EnigmaError::InvalidState(
                "rotor position count does not match rotor count".into(),
            ));
        }

        // Forward pass
        let mut value = self.plugboard.forward(input, state);

        for rotor in &self.rotors {
            value = rotor.forward(value, state);
        }

        // Reflect
        value = self.reflector.forward(value, state);

        // Reverse pass
        for rotor in self.rotors.iter().rev() {
            value = rotor.backward(value, state);
        }

        value = self.plugboard.backward(value, state);

        // Step state AFTER processing
        self.stepping
            .step(state)
            .map_err(|e| EnigmaError::SteppingError(e))?;

        Ok(value)
    }

    /// Processes a slice of bytes through the Enigma pipeline.
    pub fn process_bytes(
        &self,
        input: &[u8],
        state: &mut EnigmaState,
    ) -> EnigmaResult<Vec<u8>> {
        let mut output = Vec::with_capacity(input.len());

        for &byte in input {
            output.push(self.process_byte(byte, state)?);
        }

        Ok(output)
    }
}

//! Plugboard implementation.
//!
//! The plugboard performs a fixed, bidirectional permutation of bytes
//! before and after the rotor pipeline.

use crate::{
    component::EnigmaComponent,
    error::{EnigmaError, EnigmaResult},
    state::EnigmaState,
};

/// Plugboard component.
///
/// Internally, the plugboard stores a fixed permutation table of 256 bytes.
/// The permutation must be an involution (i.e. symmetric), so that
/// forward and backward transformations are identical.
#[derive(Debug, Clone)]
pub struct Plugboard {
    mapping: [u8; 256],
}

impl Plugboard {
    /// Creates a new `Plugboard` from a byte mapping.
    ///
    /// The mapping must be an involution:
    /// `mapping[mapping[x]] == x` for all `x`.
    pub fn new(mapping: [u8; 256]) -> EnigmaResult<Self> {
        for i in 0..256 {
            let j = mapping[i] as usize;
            if mapping[j] != i as u8 {
                return Err(EnigmaError::InvalidConfiguration(
                    "plugboard mapping must be symmetric (involution)".into(),
                ));
            }
        }

        Ok(Self { mapping })
    }

    /// Creates an identity plugboard (no transformation).
    pub fn identity() -> Self {
        let mut mapping = [0u8; 256];
        for (i, v) in mapping.iter_mut().enumerate() {
            *v = i as u8;
        }
        Self { mapping }
    }
}

impl EnigmaComponent for Plugboard {
    fn forward(&self, input: u8, _state: &EnigmaState) -> u8 {
        self.mapping[input as usize]
    }

    fn backward(&self, input: u8, _state: &EnigmaState) -> u8 {
        // Identical to forward for involutive mappings
        self.mapping[input as usize]
    }
}

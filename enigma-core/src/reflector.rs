//! Reflector implementation.
//!
//! The reflector is a symmetric, involutive transformation that
//! maps each byte to another byte such that applying it twice
//! yields the original value.

use crate::{
    component::EnigmaComponent,
    error::{EnigmaError, EnigmaResult},
    state::EnigmaState,
};

/// Reflector component.
///
/// Internally, the reflector stores a fixed involutive mapping:
/// `mapping[mapping[x]] == x`.
#[derive(Debug, Clone)]
pub struct Reflector {
    mapping: [u8; 256],
}

impl Reflector {
    /// Creates a new `Reflector` from a byte mapping.
    ///
    /// The mapping must be an involution:
    /// `mapping[mapping[x]] == x` for all `x`.
    pub fn new(mapping: [u8; 256]) -> EnigmaResult<Self> {
        for i in 0..256 {
            let j = mapping[i] as usize;
            if mapping[j] != i as u8 {
                return Err(EnigmaError::InvalidConfiguration(
                    "reflector mapping must be symmetric (involution)".into(),
                ));
            }
        }

        Ok(Self { mapping })
    }

    /// Creates an identity reflector.
    ///
    /// This is mostly useful for testing and debugging.
    pub fn identity() -> Self {
        let mut mapping = [0u8; 256];
        for (i, v) in mapping.iter_mut().enumerate() {
            *v = i as u8;
        }
        Self { mapping }
    }

    /// Creates a simple paired reflector.
    ///
    /// Bytes are paired as:
    /// 0 <-> 1, 2 <-> 3, ..., 254 <-> 255.
    ///
    /// This reflector is involutive and non-identity.
    pub fn paired() -> Self {
        let mut mapping = [0u8; 256];

        let mut i = 0;
        while i < 256 {
            mapping[i] = (i + 1) as u8;
            mapping[i + 1] = i as u8;
            i += 2;
        }

        Self { mapping }
    }

}

impl EnigmaComponent for Reflector {
    fn forward(&self, input: u8, _state: &EnigmaState) -> u8 {
        self.mapping[input as usize]
    }

    fn backward(&self, input: u8, _state: &EnigmaState) -> u8 {
        // Identical to forward for involutive mappings
        self.mapping[input as usize]
    }
}

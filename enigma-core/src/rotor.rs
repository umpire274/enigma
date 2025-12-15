//! Rotor implementations.
//!
//! A rotor performs a state-dependent, reversible transformation.
//! The current rotor position is read from `EnigmaState`.

use crate::{
    component::EnigmaComponent,
    error::{EnigmaError, EnigmaResult},
    state::EnigmaState,
};

/// A simple permutation-based rotor.
///
/// The rotor uses a fixed permutation table and applies an offset
/// derived from the rotor position stored in `EnigmaState`.
#[derive(Debug, Clone)]
pub struct Rotor {
    /// Forward permutation table.
    forward: [u8; 256],
    /// Reverse permutation table (inverse of `forward`).
    backward: [u8; 256],
    /// Index of this rotor in the EnigmaState rotor_positions vector.
    index: usize,
}

impl Rotor {
    /// Creates a new `Rotor` from a permutation table and a rotor index.
    ///
    /// The permutation must be bijective.
    pub fn new(
        permutation: [u8; 256],
        index: usize,
    ) -> EnigmaResult<Self> {
        let mut backward = [0u8; 256];
        let mut seen = [false; 256];

        for (i, &v) in permutation.iter().enumerate() {
            let j = v as usize;
            if seen[j] {
                return Err(EnigmaError::InvalidConfiguration(
                    "rotor permutation must be bijective".into(),
                ));
            }
            seen[j] = true;
            backward[j] = i as u8;
        }

        Ok(Self {
            forward: permutation,
            backward,
            index,
        })
    }

    /// Creates an identity rotor (no permutation).
    pub fn identity(index: usize) -> Self {
        let mut perm = [0u8; 256];
        for (i, v) in perm.iter_mut().enumerate() {
            *v = i as u8;
        }

        Self {
            forward: perm,
            backward: perm,
            index,
        }
    }

    fn position<'a>(&self, state: &'a EnigmaState) -> Result<u32, EnigmaError> {
        state
            .rotor_positions
            .get(self.index)
            .copied()
            .ok_or_else(|| {
                EnigmaError::InvalidState(format!(
                    "rotor index {} out of bounds",
                    self.index
                ))
            })
    }

    /// Creates a simple non-identity rotor with a fixed shift.
    ///
    /// This rotor applies a byte-wise rotation before and after
    /// the state-based offset.
    pub fn shifted(index: usize, shift: u8) -> Self {
        let mut forward = [0u8; 256];
        let mut backward = [0u8; 256];

        for i in 0..256 {
            let v = i as u8;
            let shifted = v.wrapping_add(shift);
            forward[i] = shifted;
            backward[shifted as usize] = v;
        }

        Self {
            forward,
            backward,
            index,
        }
    }
}

impl EnigmaComponent for Rotor {
    fn forward(&self, input: u8, state: &EnigmaState) -> u8 {
        let pos = self.position(state).unwrap_or(0);
        let shifted = input.wrapping_add(pos as u8);
        let mapped = self.forward[shifted as usize];
        mapped.wrapping_sub(pos as u8)
    }

    fn backward(&self, input: u8, state: &EnigmaState) -> u8 {
        let pos = self.position(state).unwrap_or(0);
        let shifted = input.wrapping_add(pos as u8);
        let mapped = self.backward[shifted as usize];
        mapped.wrapping_sub(pos as u8)
    }
}

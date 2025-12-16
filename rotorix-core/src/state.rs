//! Enigma transformation state.
//!
//! This module defines the explicit state used by the Enigma pipeline.
//! All state is external to the components and can be safely cloned,
//! snapshotted, and restored.

/// Represents the mutable state of an Enigma transformation session.
///
/// The state is intentionally kept simple and explicit to guarantee
/// deterministic behavior and ease of testing.
#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct EnigmaState {
    /// Current positions of the rotors.
    ///
    /// Each entry corresponds to a rotor in the machine, ordered
    /// from left to right.
    pub rotor_positions: Vec<u32>,

    /// Monotonic step counter.
    ///
    /// Incremented after each processed symbol.
    pub step_counter: u64,
}

impl EnigmaState {
    /// Creates a new `EnigmaState` with the given number of rotors,
    /// all initialized to position zero.
    pub fn new(rotor_count: usize) -> Self {
        Self {
            rotor_positions: vec![0; rotor_count],
            step_counter: 0,
        }
    }

    /// Resets all rotor positions and the step counter to zero.
    pub fn reset(&mut self) {
        for pos in &mut self.rotor_positions {
            *pos = 0;
        }
        self.step_counter = 0;
    }
}

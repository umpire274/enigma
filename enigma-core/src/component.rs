//! Core transformation component abstraction.
//!
//! All Enigma pipeline elements (plugboard, rotors, reflector)
//! implement this trait.

use crate::state::EnigmaState;

/// Trait implemented by all Enigma transformation components.
///
/// A component must be able to transform a single symbol in both
/// forward and reverse directions, using the current Enigma state.
///
/// Components are expected to be **pure**:
/// - no internal mutable state
/// - deterministic behavior
/// - all state is provided externally via `EnigmaState`
pub trait EnigmaComponent {
    /// Transform a symbol in the forward direction.
    ///
    /// This method is used during the forward pass through the pipeline.
    fn forward(&self, input: u8, state: &EnigmaState) -> u8;

    /// Transform a symbol in the reverse direction.
    ///
    /// This method is used during the reverse pass through the pipeline.
    fn backward(&self, input: u8, state: &EnigmaState) -> u8;
}

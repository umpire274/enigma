//! This module contains the core components for simulating the Enigma machine.

/// Module for generic utility functions.
pub mod utils;

/// Module for the Enigma machine's rotors.
pub mod rotor;

/// Module for the Enigma machine's reflector.
pub mod reflector;

/// Module for the Enigma machine's plugboard.
pub mod plugboard;

/// Module for the Enigma machine, integrating rotors, reflector, and plugboard.
pub mod enigma;
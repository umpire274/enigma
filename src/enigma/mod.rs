/// Module containing the implementation of the rotors for the Enigma machine.
///
/// Rotors are the core components of the Enigma machine that perform the substitution cipher.
/// This module defines the `Rotor` struct and its associated methods for encryption and rotation.
pub mod rotor;

/// Module containing the implementation of the reflector for the Enigma machine.
///
/// The reflector is a fixed component that redirects the electrical signal back through the rotors
/// after they have performed their substitutions. This module defines the `Reflector` struct and
/// its associated methods for reflecting characters.
pub mod reflector;

/// Module containing the implementation of the plugboard for the Enigma machine.
///
/// The plugboard swaps pairs of letters before and after they pass through the rotors, adding an
/// extra layer of substitution. This module defines the `Plugboard` struct and its associated
/// methods for swapping characters.
pub mod plugboard;

/// Module containing the implementation of the Enigma machine.
///
/// This module defines the `EnigmaMachine` struct, which encapsulates the rotors, reflector, and
/// plugboard, and provides methods for encrypting and decrypting messages.
pub mod enigma;
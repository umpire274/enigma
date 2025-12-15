use enigma_core::{
    EnigmaComponent,
    EnigmaMachine,
    LinearStepping,
    Reflector,
    Rotor,
};

use crate::plugboard::build_plugboard;

pub fn build_machine(
    rotor_count: usize,
    step_modulus: u32,
    swap: Option<String>,
) -> EnigmaMachine {
    let plugboard = Box::new(build_plugboard(swap));

    let mut rotors: Vec<Box<dyn EnigmaComponent>> = Vec::new();
    for i in 0..rotor_count {
        rotors.push(Box::new(Rotor::identity(i)));
    }

    let reflector = Box::new(Reflector::identity());
    let stepping = Box::new(LinearStepping::new(step_modulus));

    EnigmaMachine::new(
        plugboard,
        rotors,
        reflector,
        stepping,
    )
        .expect("invalid Enigma configuration")
}

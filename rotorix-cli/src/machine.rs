use rotorix_core::{EnigmaComponent, EnigmaMachine, LinearStepping, Reflector, Rotor};

use crate::plugboard::build_plugboard;

pub fn build_machine(
    rotor_count: usize,
    step_modulus: u32,
    swap: Option<String>,
    rotor_mode: String,
    reflector_mode: String,
    seed: Option<u64>,
) -> EnigmaMachine {
    let plugboard = Box::new(build_plugboard(swap));

    let mut rotors: Vec<Box<dyn EnigmaComponent>> = Vec::new();
    for i in 0..rotor_count {
        match rotor_mode.as_str() {
            "identity" => {
                rotors.push(Box::new(Rotor::identity(i)));
            }
            "shifted" => {
                rotors.push(Box::new(Rotor::shifted(i, 13)));
            }
            "seed" => {
                let seed = seed.expect("seed-based rotor requires --seed");
                rotors.push(Box::new(Rotor::from_seed(i, seed)));
            }
            _ => panic!("unknown rotor mode"),
        }
    }

    let reflector = match reflector_mode.as_str() {
        "identity" => Box::new(Reflector::identity()),
        "paired" => Box::new(Reflector::paired()),
        _ => panic!("unknown reflector mode"),
    };

    let stepping = Box::new(LinearStepping::new(step_modulus));

    EnigmaMachine::new(plugboard, rotors, reflector, stepping)
        .expect("invalid Enigma configuration")
}

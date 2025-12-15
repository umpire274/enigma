use enigma_core::{
    EnigmaMachine,
    EnigmaState,
    LinearStepping,
    Plugboard,
    Reflector,
    Rotor,
};

#[test]
fn roundtrip_identity_pipeline() {
    // --- Components ---
    let plugboard = Box::new(Plugboard::identity());

    let rotors: Vec<Box<dyn enigma_core::EnigmaComponent>> = vec![
        Box::new(Rotor::identity(0)),
    ];

    let reflector = Box::new(Reflector::identity());

    let stepping = Box::new(LinearStepping::new(256));

    // --- Machine ---
    let machine = EnigmaMachine::new(
        plugboard,
        rotors,
        reflector,
        stepping,
    )
        .expect("failed to build EnigmaMachine");

    // --- State ---
    let mut enc_state = EnigmaState::new(1);

    let plaintext = b"HELLO ENIGMA";
    let ciphertext = machine
        .process_bytes(plaintext, &mut enc_state)
        .expect("encryption failed");

    // Reset state for decryption
    let mut dec_state = EnigmaState::new(1);

    let decrypted = machine
        .process_bytes(&ciphertext, &mut dec_state)
        .expect("decryption failed");

    assert_eq!(decrypted, plaintext);
}

#[test]
fn stepping_advances_state() {
    let plugboard = Box::new(Plugboard::identity());
    let rotor = Box::new(Rotor::identity(0));
    let reflector = Box::new(Reflector::identity());
    let stepping = Box::new(LinearStepping::new(10));

    let machine = EnigmaMachine::new(
        plugboard,
        vec![rotor],
        reflector,
        stepping,
    )
        .unwrap();

    let mut state = EnigmaState::new(1);

    let _ = machine.process_byte(0x41, &mut state).unwrap();
    let _ = machine.process_byte(0x41, &mut state).unwrap();
    let _ = machine.process_byte(0x41, &mut state).unwrap();

    assert_eq!(state.step_counter, 3);
    assert_eq!(state.rotor_positions[0], 3);
}

#[test]
fn invalid_state_is_rejected() {
    let plugboard = Box::new(Plugboard::identity());
    let rotor = Box::new(Rotor::identity(0));
    let reflector = Box::new(Reflector::identity());
    let stepping = Box::new(LinearStepping::new(256));

    let machine = EnigmaMachine::new(
        plugboard,
        vec![rotor],
        reflector,
        stepping,
    )
        .unwrap();

    // State has zero rotors, machine expects one
    let mut bad_state = EnigmaState::default();

    let result = machine.process_byte(0x41, &mut bad_state);

    assert!(result.is_err());
}

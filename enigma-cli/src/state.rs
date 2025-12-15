use enigma_core::EnigmaState;

pub fn build_state(rotors: usize, seed: Option<u64>) -> EnigmaState {
    let mut state = EnigmaState::new(rotors);

    if let Some(seed) = seed {
        for (i, pos) in state.rotor_positions.iter_mut().enumerate() {
            // semplice derivazione deterministica
            *pos = ((seed >> (i * 8)) & 0xFF) as u32;
        }
    }

    state
}

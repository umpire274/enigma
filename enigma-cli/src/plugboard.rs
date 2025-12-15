use enigma_core::Plugboard;

pub fn build_plugboard(swap: Option<String>) -> Plugboard {
    let mut mapping = [0u8; 256];
    for (i, item) in mapping.iter_mut().enumerate() {
        *item = i as u8;
    }

    if let Some(s) = swap {
        let parts: Vec<_> = s.split(':').collect();
        if parts.len() == 2 {
            let a: u8 = parts[0].parse().expect("invalid swap value");
            let b: u8 = parts[1].parse().expect("invalid swap value");
            mapping[a as usize] = b;
            mapping[b as usize] = a;
        }
    }

    Plugboard::new(mapping).expect("invalid plugboard configuration")
}

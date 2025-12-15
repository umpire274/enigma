use data_encoding::BASE32HEX_NOPAD;

fn encode_ciphertext(bytes: &[u8]) -> String {
    // Base32 Hex (0-9 A-V). Se vuoi Crockford puro, possiamo fare mapping dopo.
    BASE32HEX_NOPAD.encode(bytes)
}

fn decode_ciphertext(s: &str) -> Vec<u8> {
    BASE32HEX_NOPAD
        .decode(s.as_bytes())
        .expect("invalid ciphertext encoding")
}

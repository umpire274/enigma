use data_encoding::{BASE32HEX_NOPAD, HEXUPPER};
use data_encoding::BASE64_NOPAD;

pub fn encode_ciphertext(bytes: &[u8], encoding: &str) -> String {
    match encoding {
        "hex" => HEXUPPER.encode(bytes),
        "base64" => BASE64_NOPAD.encode(bytes),
        "base32" => BASE32HEX_NOPAD.encode(bytes),
        _ => panic!("unsupported encoding: {}", encoding),
    }
}

pub fn decode_ciphertext(s: &str, encoding: &str) -> Vec<u8> {
    match encoding {
        "hex" => HEXUPPER
            .decode(s.as_bytes())
            .expect("invalid HEX ciphertext"),
        "base64" => BASE64_NOPAD
            .decode(s.as_bytes())
            .expect("invalid Base64 ciphertext"),
        "base32" => BASE32HEX_NOPAD
            .decode(s.as_bytes())
            .expect("invalid Base32 ciphertext"),
        _ => panic!("unsupported encoding: {}", encoding),
    }
}

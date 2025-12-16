use assert_cmd::cargo::cargo_bin_cmd;

fn encrypt_then_decrypt(input: &str, encoding: &str) -> String {
    // Encrypt
    let encrypt_output = cargo_bin_cmd!("rotorix")
        .args([
            "encrypt",
            input,
            "--rotors",
            "3",
            "--seed",
            "12345",
            "--rotor-mode",
            "seed",
            "--reflector-mode",
            "paired",
            "--encoding",
            encoding,
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let ciphertext = String::from_utf8_lossy(&encrypt_output).trim().to_string();

    // Decrypt
    let decrypt_output = cargo_bin_cmd!("rotorix")
        .args([
            "decrypt",
            &ciphertext,
            "--rotors",
            "3",
            "--seed",
            "12345",
            "--rotor-mode",
            "seed",
            "--reflector-mode",
            "paired",
            "--encoding",
            encoding,
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    String::from_utf8_lossy(&decrypt_output).trim().to_string()
}

#[test]
fn roundtrip_base32() {
    let input = "HELLOENIGMA123";
    let output = encrypt_then_decrypt(input, "base32");
    assert_eq!(output, input);
}

#[test]
fn roundtrip_hex() {
    let input = "HELLOENIGMA123";
    let output = encrypt_then_decrypt(input, "hex");
    assert_eq!(output, input);
}

#[test]
fn roundtrip_base64() {
    let input = "HELLOENIGMA123";
    let output = encrypt_then_decrypt(input, "base64");
    assert_eq!(output, input);
}

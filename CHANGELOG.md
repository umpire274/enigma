# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.1.0] - 2025-12-15

### Added
- Initial release of the **enigma-core** library.
- Enigma-inspired transformation pipeline with explicit state management.
- Core components:
    - Plugboard (involutive byte permutation).
    - Rotor (state-dependent reversible transformation).
    - Reflector (involutive symmetric mapping).
- Pluggable stepping strategy abstraction.
- Linear stepping strategy implementation.
- Stateless `EnigmaMachine` orchestrating the full pipeline.
- Explicit `EnigmaState` with rotor positions and step counter.
- Public error model (`EnigmaError`, `EnigmaResult`).
- Integration tests validating:
    - round-trip (encrypt → decrypt) correctness
    - stepping behavior
    - invalid state detection.

### Notes
- This release is **experimental** and intended for educational and research purposes.
- No cryptographic security guarantees are provided.
- The public API is considered **unstable** until version `1.0.0`.

---


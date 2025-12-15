# enigma-core

`enigma-core` is a Rust library that implements an **Enigma-inspired transformation engine**, modeled as a modular and
deterministic pipeline.

It reinterprets the conceptual architecture of the historical Enigma machine — *plugboard, rotors, reflector, and
stepping* — using modern software design principles, without making claims about cryptographic security.

---

## Design Goals

The primary goals of `enigma-core` are:

- **Architectural clarity**  
  Enigma is treated as a transformation *pipeline*, not as a cipher to be “fixed” or hardened.

- **Explicit state management**  
  All state is external, reproducible, and snapshot-friendly.

- **Deterministic behavior**  
  Given the same configuration and initial state, transformations are fully deterministic.

- **Modularity and extensibility**  
  Historical-style components and modern transformation strategies can coexist.

- **Library-first design**  
  No assumptions are made about transport, UI, or application layer.

---

## What `enigma-core` Is

- A reusable Rust **library**
- A framework for **Enigma-like pipelines**
- A foundation for:
    - CLI tools
    - chat or messaging experiments
    - protocol design research
    - educational projects

---

## What `enigma-core` Is Not

To avoid confusion, `enigma-core` is **not**:

- a secure encryption library
- a messaging protocol
- a TLS or SSL replacement
- a production-ready cryptographic system

No security guarantees are provided.

If you need secure encryption, use established and audited libraries.

---

## Core Concepts

### Pipeline Architecture

Data flows through the following stages:

```text
Input
 → Plugboard
 → Rotors (forward)
 → Reflector
 → Rotors (reverse)
 → Plugboard
 → Output
```

After each processed symbol, the internal state is updated via a stepping strategy.

---

### Explicit State

The transformation state is represented by a dedicated structure, separate from component logic.

This enables:

- snapshot and restore
- deterministic replay
- controlled resynchronization
- robust testing

---

### Components

All transformation stages implement a common interface, allowing them to be composed freely.

Examples include:

- classic substitution-based rotors
- dynamically derived rotors
- static or computed reflectors
- configurable plugboards

---

## Versioning and Stability

`enigma-core` follows **Semantic Versioning**.

- Versions `< 1.0.0` are considered **unstable**
- Breaking changes may occur between minor versions
- The API will be stabilized before `1.0.0`

Current development target: `v0.1.0`

---

## Usage (Preview)

⚠️ The API is still evolving.
The following example is illustrative only.

```rust
use enigma_core::{EnigmaMachine, EnigmaState};

let machine = EnigmaMachine::new(/* configuration */) ?;
let mut state = EnigmaState::default ();

let encrypted = machine.process_bytes(b"HELLO", & mut state);

// Reset state to decrypt
let mut state2 = EnigmaState::default ();
let decrypted = machine.process_bytes( & encrypted, & mut state2);

assert_eq!(decrypted, b"HELLO");
```

---

## License

This project is licensed under the [MIT License](LICENSE).

---

## Author

- Developed by
    - **Alessandro Maestri** ([https://github.com/umpire274](https://github.com/umpire274))

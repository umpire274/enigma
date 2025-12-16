# Rotorix

**Rotorix** is an experimental Rust project inspired by the architecture of the historical Rotorix machine, reimagined as
a modern, modular transformation pipeline.

The goal of this project is **not** to recreate Rotorix as a cryptographically secure system, but to explore how its
conceptual structure — plugboard, rotors, reflector, and stepping — can be modeled, extended, and reused with modern
software design principles.

This repository serves as the **main container** for the Rotorix ecosystem.

---

## Project Philosophy

Rotorix is built around a few core ideas:

- **Architecture over algorithms**  
  Rotorix is treated as a *pipeline* of transformations, not as a cipher to be made “secure”.

- **Separation of concerns**  
  The core logic is completely independent of networking, user interfaces, or transport protocols.

- **Deterministic and testable behavior**  
  State is explicit, reproducible, and snapshot-friendly.

- **Extensibility**  
  Historical components and modern crypto-based transformations can coexist.

This makes Rotorix suitable for:

- educational purposes
- cryptographic experimentation
- protocol design research
- reusable libraries and tools

---

## Repository Structure

```text
rotorix/
├── rotorix-core/     # Core Rotorix transformation library
├── README.md        # This file
└── (future crates)
```

### `rotorix-core`

The `rotorix-core` crate contains the core Rotorix-inspired engine:

- plugboard
- rotors
- reflector
- stepping strategies
- state management

It is designed to be used by:

- CLI tools
- chat or messaging clients
- other Rust applications
- research or educational projects

---

## Project Status

### 🚧 Early development

- Current version: `0.1.0` (**in progress**)
- Public API: **unstable**
- No security guarantees are provided

Breaking changes are expected until `1.0.0`.

---

## Non-Goals

To avoid ambiguity, Rotorix explicitly does **not** aim to be:

- a secure messaging system
- a replacement for modern cryptographic protocols
- a drop-in encryption library
- a production-ready security solution

If you need secure communication, use established tools and protocols such as TLS, Signal, or age.

---

## License

This project is licensed under the [MIT License](LICENSE).

---

## Author

- Developed by
    - **Alessandro Maestri** ([https://github.com/umpire274](https://github.com/umpire274))

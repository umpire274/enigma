// src/main.rs

mod cli;
mod enigma;
mod crypto;
mod gui;

use crate::enigma::utils;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "--cli" {
        // Modalità CLI
        cli::run_cli();
    } else {
        // Modalità GUI (default)
        if let Err(e) = gui::run_gui() {
            eprintln!("GUI error: {}", e);
        }
    }
}
// src/main.rs

mod cli;
mod enigma;
mod crypto;
mod gui;

use crate::enigma::utils;
use std::env;
use log::info;
use crate::cli::run_cli;

fn main() {
    env_logger::init();
    info!("Application start...");
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "--cli" {
        // Modalità CLI
        if let Err(e) = run_cli() {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    } else {
        // Modalità GUI (default)
        if let Err(e) = gui::run_gui() {
            eprintln!("GUI error: {}", e);
        }
    }
    info!("Application ended.");
}
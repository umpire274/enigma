mod enigma; // Modulo con la logica della macchina Enigma
mod cli;    // Modulo per la gestione della CLI

use enigma::enigma::EnigmaMachine;

fn main() {
    // Carichiamo la configurazione di Enigma da JSON
    let enigma = EnigmaMachine::from_config("config.json")
        .expect("Errore nel caricamento della configurazione!");

    // Chiediamo all'utente il messaggio da cifrare
    let input = cli::get_user_input("Inserisci il messaggio da cifrare: ");

    // Cifriamo il messaggio con Enigma
    let encrypted_text = enigma.encrypt_message(&input);

    // Mostriamo il risultato all'utente
    cli::display_output(&encrypted_text);
}

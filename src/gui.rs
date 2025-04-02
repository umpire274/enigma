// src/gui.rs
use crate::crypto::{decrypt_message, encrypt_message};
use crate::enigma;
use crate::enigma::utils::Config;
use eframe::egui;

pub struct EnigmaApp {
    input_text: String,
    output_text: String,
    operation: Operation,
    config: Config,
    show_copied_notice: bool,
    copy_time: f64,
}

#[derive(PartialEq)]
enum Operation {
    Encrypt,
    Decrypt,
}

impl Default for EnigmaApp {
    fn default() -> Self {
        Self {
            input_text: String::new(),
            output_text: String::new(),
            operation: Operation::Encrypt,
            config: Config {
                n_rt: 3,
                plugboard_pairs: vec![('A', 'B'), ('C', 'D')],
                sstk: 12345,
            },
            show_copied_notice: false,
            copy_time: 0.0,
        }
    }
}

impl eframe::App for EnigmaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Titolo
            ui.heading("Enigma Machine GUI");

            // Input Section
            ui.add_space(10.0);
            ui.label("Input Message:");
            let input_text_edit = egui::TextEdit::multiline(&mut self.input_text)
                .hint_text("Type your message here...")
                .desired_width(f32::INFINITY)
                .desired_rows(13);
            ui.add(input_text_edit);

            // Operation Section (radio buttons)
            ui.add_space(5.0);
            // Operation Section con radio button colorati
            ui.horizontal(|ui| {
                // Radio Encrypt (verde)
                let encrypt_text = egui::RichText::new("🔒 Encrypt")
                    .color(if self.operation == Operation::Encrypt {
                        egui::Color32::from_rgb(56, 142, 60) // Verde acceso quando selezionato
                    } else {
                        egui::Color32::GRAY // Grigio quando non selezionato
                    });

                if ui.radio_value(&mut self.operation, Operation::Encrypt, encrypt_text).clicked() {
                    // Forza ridisegno per feedback visivo immediato
                    ctx.request_repaint();
                }

                // Radio Decrypt (arancione)
                let decrypt_text = egui::RichText::new("🔓 Decrypt")
                    .color(if self.operation == Operation::Decrypt {
                        egui::Color32::from_rgb(245, 124, 0) // Arancione acceso quando selezionato
                    } else {
                        egui::Color32::GRAY // Grigio quando non selezionato
                    });

                if ui.radio_value(&mut self.operation, Operation::Decrypt, decrypt_text).clicked() {
                    ctx.request_repaint();
                }

                // Pulsante Process (blu)
                if ui.add(
                    egui::Button::new(rich_text("⚙ Process"))
                        .fill(egui::Color32::from_rgb(51, 103, 214))
                ).clicked() {
                    self.process_message();
                }
            });

            // Output Section
            ui.add_space(20.0);
            ui.label("Output Message:");
            let output_text_edit = egui::TextEdit::multiline(&mut self.output_text)
                .hint_text("Result will appear here...")
                .desired_width(f32::INFINITY)
                .desired_rows(13)
                .interactive(false);
            ui.add(output_text_edit);

            // Pulsante Copy (viola) e feedback
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                if ui.add(
                    egui::Button::new(rich_text("📋 Copy to clipboard"))
                        .fill(egui::Color32::from_rgb(123, 31, 162))
                ).clicked() && !self.output_text.is_empty() {
                    ctx.copy_text(self.output_text.clone());
                    self.show_copied_notice = true;
                    self.copy_time = ctx.input(|i| i.time);
                }

                if self.show_copied_notice {
                    ui.label(rich_text("✓ Copied!").color(egui::Color32::GREEN));
                }
            });

            // Pulsante Quit (rosso) in basso a destra
            ui.add_space(10.0);
            ui.separator();
            // Pulsante Quit con icona alternativa e stile garantito
            ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
                let quit_btn = egui::Button::new(
                    egui::RichText::new("🚪 Quit")  // Icona porta + testo
                        .color(egui::Color32::WHITE)
                )
                    .fill(egui::Color32::from_rgb(198, 40, 40));  // Rosso

                if ui.add(quit_btn).clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });

            ui.allocate_space(ui.available_size());
        });
    }
}

// Helper function per il testo dei pulsanti
fn rich_text(text: impl Into<String>) -> egui::RichText {
    egui::RichText::new(text)
        .color(egui::Color32::WHITE)
        .text_style(egui::TextStyle::Button)
}

impl EnigmaApp {
    fn process_message(&mut self) {
        let key = &enigma::utils::KEY[..]; // Ottieni una slice &[u8] da KEY
        let iv = &enigma::utils::IV[..]; // Ottieni una slice &[u8] da IV

        self.output_text = match self.operation {
            Operation::Encrypt => encrypt_message(&self.input_text, &self.config, key, iv)
                .unwrap_or_else(|e| format!("Error: {}", e)),
            Operation::Decrypt => decrypt_message(&self.input_text, &self.config, key, iv)
                .unwrap_or_else(|e| format!("Error: {}", e)),
        };
    }

    #[allow(dead_code)]
    fn clear_copy_notification(&mut self) {
        if self.output_text.ends_with(" (Copied!)") {
            self.output_text = self.output_text.replace(" (Copied!)", "");
        }
    }
}

pub fn run_gui() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([400.0, 300.0]) // Dimensione minima
            .with_resizable(true) // Abilita il ridimensionamento
            .with_title("Enigma Machine"),
        ..Default::default()
    };

    eframe::run_native(
        "Enigma Machine",
        options,
        Box::new(|_cc| Ok(Box::new(EnigmaApp::default()))),
    )
}

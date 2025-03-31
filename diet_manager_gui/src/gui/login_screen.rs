use eframe::egui;
use crate::models::Database;
use crate::app_state::AppState;
use crate::gui::styling;
use crate::gui::undo_manager::UndoManager;
pub struct LoginScreen {
    username: String,
    password: String,
    error_message: Option<String>,
}

impl LoginScreen {
    pub fn new() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
            error_message: None,
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, db: &mut Database, current_state: &mut AppState, undo_manager: &mut UndoManager) {
        ui.vertical_centered(|ui| {
            ui.add_space(40.0);
            ui.heading(egui::RichText::new("Welcome to Diet Manager")
                .size(32.0)
                .color(styling::AppTheme::default().accent_color)
                .strong());

            ui.add_space(8.0);
            ui.label(egui::RichText::new("Track your nutrition goals with ease")
                .size(16.0));

            ui.add_space(30.0);

            // Login form in a card
            let card = styling::card_frame();
            card.show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading(egui::RichText::new("Login").size(24.0));
                });
                ui.add_space(16.0);

                // Username field with improved styling
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("👤").size(20.0));
                    ui.add(egui::TextEdit::singleline(&mut self.username)
                        .hint_text("Enter your username")
                        .desired_width(250.0));
                });

                ui.add_space(8.0);

                // Password field with improved styling
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("🔒").size(20.0));
                    ui.add(egui::TextEdit::singleline(&mut self.password)
                        .password(true)
                        .hint_text("Enter your password")
                        .desired_width(250.0));
                });

                ui.add_space(20.0);

                // Login and Register buttons
                ui.vertical_centered(|ui| {
                    if styling::primary_button(ui, "Login").clicked() {
                        // Your existing login logic here
                        self.handle_login(db, current_state);
                    }

                    ui.add_space(8.0);

                    ui.label("Don't have an account?");
                    if ui.button(egui::RichText::new("Create Account").strong()).clicked() {
                        *current_state = AppState::Register;
                    }

                    // Show error message if any
                    if let Some(ref error) = self.error_message {
                        ui.add_space(16.0);
                        ui.colored_label(
                            styling::AppTheme::default().error_color,
                            egui::RichText::new(error).size(14.0).strong()
                        );
                    }
                });
            });
        });
    }

    // Add your existing login logic method here
    fn handle_login(&mut self, db: &mut Database, current_state: &mut AppState) {
        // Implement your login logic here
        // This is just a placeholder - replace with your actual login code
        if self.username.is_empty() || self.password.is_empty() {
            self.error_message = Some("Username and password are required".to_string());
            return;
        }

        // Check if user exists and password matches
        if let Some(user) = db.users.get(&self.username) {
            if user.password == self.password {
                db.current_user = user.user_id.clone();
                *current_state = AppState::Home;
            } else {
                self.error_message = Some("Invalid password".to_string());
            }
        } else {
            self.error_message = Some("User not found".to_string());
        }
    }
}
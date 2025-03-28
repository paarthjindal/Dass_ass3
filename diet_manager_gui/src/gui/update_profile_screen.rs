use eframe::egui;
use crate::models::{Database, Gender, ActivityLevel, CalorieCalculationMethod, User};
use crate::app_state::AppState;
use crate::gui::styling;

pub struct UpdateProfileScreen {
    success_message: Option<String>,
    error_message: Option<String>,
    editing_user: Option<User>, // Store the user we're editing
    initialized: bool,         // Track if we've loaded the user
    should_return_home: bool,  // Flag to track navigation
}

impl UpdateProfileScreen {
    pub fn new() -> Self {
        Self {
            success_message: None,
            error_message: None,
            editing_user: None,
            initialized: false,
            should_return_home: false,
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, db: &mut Database, current_state: &mut AppState) {
        // Check if we should return to home screen
        if self.should_return_home {
            self.initialized = false;
            self.editing_user = None;
            self.success_message = None;
            self.error_message = None;
            self.should_return_home = false;
            *current_state = AppState::Home;
            return;
        }

        // Initialize editing_user once when the screen is first shown
        if !self.initialized {
            if let Some(user) = db.users.values().find(|u| u.user_id == db.current_user) {
                self.editing_user = Some(user.clone());
            }
            self.initialized = true;
        }

        ui.vertical_centered(|ui| {
            ui.heading(egui::RichText::new("Update Profile").size(28.0).strong());
            ui.add_space(4.0);
            ui.label("Adjust your personal information and preferences");
            ui.add_space(20.0);
        });

        if let Some(ref mut user_clone) = self.editing_user {
            // Create a local flag to track cancel button press
            let mut cancel_clicked = false;
            let mut save_clicked = false;

            styling::card_frame().show(ui, |ui| {
                // User information section
                styling::section_header(ui, "Personal Information");

                // Username (read-only)
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("👤").size(20.0));
                    ui.vertical(|ui| {
                        ui.label("Username:");
                        ui.add_enabled(false, egui::TextEdit::singleline(&mut user_clone.username)
                            .desired_width(300.0));
                        ui.label(egui::RichText::new("Username cannot be changed").size(12.0).italics());
                    });
                });

                ui.add_space(16.0);

                // Physical profile section
                styling::section_header(ui, "Physical Profile");

                // Gender selection
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("⚧").size(20.0));
                    ui.label("Gender:");
                    ui.radio_value(&mut user_clone.profile.gender, Gender::Male, "Male");
                    ui.radio_value(&mut user_clone.profile.gender, Gender::Female, "Female");
                });

                ui.add_space(8.0);

                // Height slider
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("📏").size(20.0));
                    ui.vertical(|ui| {
                        ui.label("Height (cm):");
                        ui.add(egui::Slider::new(&mut user_clone.profile.height_cm, 100.0..=250.0)
                            .text("cm")
                            .clamp_to_range(true)
                            .smart_aim(false)
                            .fixed_decimals(1));
                    });
                });

                ui.add_space(8.0);

                // Age slider
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("🗓️").size(20.0));
                    ui.vertical(|ui| {
                        ui.label("Age:");
                        ui.add(egui::Slider::new(&mut user_clone.profile.age, 1..=120)
                            .text("years")
                            .clamp_to_range(true)
                            .smart_aim(false));
                    });
                });

                ui.add_space(8.0);

                // Weight slider
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("⚖️").size(20.0));
                    ui.vertical(|ui| {
                        ui.label("Weight (kg):");
                        ui.add(egui::Slider::new(&mut user_clone.profile.weight_kg, 30.0..=250.0)
                            .text("kg")
                            .clamp_to_range(true)
                            .smart_aim(false)
                            .fixed_decimals(1));
                    });
                });

                ui.add_space(16.0);

                // Activity level section
                styling::section_header(ui, "Activity Level");

                ui.vertical(|ui| {
                    ui.radio_value(&mut user_clone.profile.activity_level, ActivityLevel::Sedentary,
                        "📚 Sedentary (little or no exercise)");
                    ui.radio_value(&mut user_clone.profile.activity_level, ActivityLevel::Light,
                        "🚶 Light (exercise 1-3 days/week)");
                    ui.radio_value(&mut user_clone.profile.activity_level, ActivityLevel::Moderate,
                        "🏃 Moderate (exercise 3-5 days/week)");
                    ui.radio_value(&mut user_clone.profile.activity_level, ActivityLevel::VeryActive,
                        "🏋️ Very Active (exercise 6-7 days/week)");
                    ui.radio_value(&mut user_clone.profile.activity_level, ActivityLevel::ExtraActive,
                        "🏆 Extra Active (very intense exercise/physical job)");
                });

                ui.add_space(16.0);

                // Calorie calculation method
                styling::section_header(ui, "Calorie Calculation Method");

                ui.vertical(|ui| {
                    ui.radio_value(&mut user_clone.profile.calorie_method, CalorieCalculationMethod::HarrisBenedict,
                        "Harris-Benedict Formula");
                    ui.radio_value(&mut user_clone.profile.calorie_method, CalorieCalculationMethod::MifflinStJeor,
                        "Mifflin-St Jeor Formula");
                });

                // Calculate calorie target based on current settings
                let target_calories = user_clone.profile.calculate_target_calories();
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Daily Calorie Target:").strong());
                    ui.label(egui::RichText::new(format!("{:.0} kcal", target_calories))
                        .size(18.0)
                        .color(styling::AppTheme::default().accent_color)
                        .strong());
                });

                ui.add_space(16.0);

                // Status messages
                if let Some(ref success) = self.success_message {
                    ui.colored_label(
                        styling::AppTheme::default().success_color,
                        egui::RichText::new(success).size(14.0).strong()
                    );
                    ui.add_space(8.0);
                }

                if let Some(ref error) = self.error_message {
                    ui.colored_label(
                        styling::AppTheme::default().error_color,
                        egui::RichText::new(error).size(14.0).strong()
                    );
                    ui.add_space(8.0);
                }

                // Action buttons
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if styling::warning_button(ui, "Cancel").clicked() {
                            // Set flag to handle outside the closure
                            cancel_clicked = true;
                        }

                        ui.add_space(10.0);

                        if styling::success_button(ui, "Save Changes").clicked() {
                            // Set flag to handle outside the closure
                            save_clicked = true;
                        }
                    });
                });
            });

            // Handle button actions outside the closure to avoid borrowing conflicts
            if cancel_clicked {
                self.should_return_home = true;
            }

            if save_clicked {
                // Update the actual user in the database
                if let Some(user) = db.users.get_mut(&user_clone.username) {
                    user.profile = user_clone.profile.clone();
                    self.success_message = Some("Profile updated successfully!".to_string());
                    self.error_message = None;
                } else {
                    self.error_message = Some("Failed to update profile".to_string());
                    self.success_message = None;
                }
            }
        } else {
            // No user found
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);
                ui.label(egui::RichText::new("User not found").size(18.0).color(styling::AppTheme::default().error_color));
                ui.add_space(16.0);
                if styling::warning_button(ui, "Back to Home").clicked() {
                    // Set flag to return to home
                    self.should_return_home = true;
                }
                ui.add_space(40.0);
            });
        }
    }
}
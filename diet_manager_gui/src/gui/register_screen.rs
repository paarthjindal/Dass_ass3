use eframe::egui;
use crate::models::{Database, User, UserProfile, Gender, ActivityLevel, CalorieCalculationMethod};
use crate::app_state::AppState;
use uuid::Uuid;
use crate::gui::styling;

pub struct RegisterScreen {
    username: String,
    password: String,
    gender: Gender,
    height_cm: String,
    age: String,
    weight_kg: String,
    activity_level: ActivityLevel,
    calorie_method: CalorieCalculationMethod,
    error_message: Option<String>,
}

impl RegisterScreen {
    pub fn new() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
            gender: Gender::Male,
            height_cm: String::new(),
            age: String::new(),
            weight_kg: String::new(),
            activity_level: ActivityLevel::Moderate,
            calorie_method: CalorieCalculationMethod::HarrisBenedict,
            error_message: None,
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, db: &mut Database, current_state: &mut AppState) {
        ui.vertical_centered(|ui| {
            ui.heading(egui::RichText::new("Create Account").size(28.0).strong());
            ui.add_space(4.0);
            ui.label("Set up your profile for personalized nutrition tracking");
            ui.add_space(20.0);
        });

        styling::card_frame().show(ui, |ui| {
            // Account information section
            styling::section_header(ui, "Account Information");

            // Username field
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("👤").size(20.0));
                ui.vertical(|ui| {
                    ui.label("Username:");
                    ui.add(egui::TextEdit::singleline(&mut self.username)
                        .hint_text("Choose a username")
                        .desired_width(300.0));
                });
            });

            ui.add_space(8.0);

            // Password field
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("🔒").size(20.0));
                ui.vertical(|ui| {
                    ui.label("Password:");
                    ui.add(egui::TextEdit::singleline(&mut self.password)
                        .password(true)
                        .hint_text("Create a password")
                        .desired_width(300.0));
                });
            });

            ui.add_space(16.0);

            // Physical profile section
            styling::section_header(ui, "Physical Profile");

            // Gender selection
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("⚧").size(20.0));
                ui.label("Gender:");
                if ui.selectable_label(self.gender == Gender::Male, "Male").clicked() {
                    self.gender = Gender::Male;
                }
                if ui.selectable_label(self.gender == Gender::Female, "Female").clicked() {
                    self.gender = Gender::Female;
                }
            });

            ui.add_space(8.0);

            // Height field
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("📏").size(20.0));
                ui.vertical(|ui| {
                    ui.label("Height (cm):");
                    ui.add(egui::TextEdit::singleline(&mut self.height_cm)
                        .hint_text("Enter your height")
                        .desired_width(300.0));
                });
            });

            ui.add_space(8.0);

            // Age field
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("🗓️").size(20.0));
                ui.vertical(|ui| {
                    ui.label("Age:");
                    ui.add(egui::TextEdit::singleline(&mut self.age)
                        .hint_text("Enter your age")
                        .desired_width(300.0));
                });
            });

            ui.add_space(8.0);

            // Weight field
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("⚖️").size(20.0));
                ui.vertical(|ui| {
                    ui.label("Weight (kg):");
                    ui.add(egui::TextEdit::singleline(&mut self.weight_kg)
                        .hint_text("Enter your weight")
                        .desired_width(300.0));
                });
            });

            ui.add_space(16.0);

            // Activity level section
            styling::section_header(ui, "Activity Level");

            ui.horizontal_wrapped(|ui| {
                ui.selectable_value(&mut self.activity_level, ActivityLevel::Sedentary,
                    "Sedentary (little or no exercise)");
                ui.selectable_value(&mut self.activity_level, ActivityLevel::Light,
                    "Light (exercise 1-3 days/week)");
                ui.selectable_value(&mut self.activity_level, ActivityLevel::Moderate,
                    "Moderate (exercise 3-5 days/week)");
                ui.selectable_value(&mut self.activity_level, ActivityLevel::VeryActive,
                    "Very Active (exercise 6-7 days/week)");
                ui.selectable_value(&mut self.activity_level, ActivityLevel::ExtraActive,
                    "Extra Active (very intense exercise/physical job)");
            });

            ui.add_space(16.0);

            // Calculation method
            styling::section_header(ui, "Calorie Calculation Method");

            ui.horizontal_wrapped(|ui| {
                ui.selectable_value(&mut self.calorie_method, CalorieCalculationMethod::HarrisBenedict,
                    "Harris-Benedict");
                ui.selectable_value(&mut self.calorie_method, CalorieCalculationMethod::MifflinStJeor,
                    "Mifflin-St Jeor");
            });

            ui.add_space(20.0);

            // Display error message if any
            if let Some(ref error) = self.error_message {
                ui.colored_label(
                    styling::AppTheme::default().error_color,
                    egui::RichText::new(error).size(14.0).strong()
                );
                ui.add_space(8.0);
            }

            // Buttons
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if styling::warning_button(ui, "Cancel").clicked() {
                        *current_state = AppState::Login;
                    }

                    ui.add_space(10.0);

                    if styling::success_button(ui, "Register").clicked() {
                        self.handle_registration(db, current_state);
                    }
                });
            });
        });
    }

    fn handle_registration(&mut self, db: &mut Database, current_state: &mut AppState) {
        // Check for empty fields
        if self.username.is_empty() || self.password.is_empty() {
            self.error_message = Some("Username and password are required.".to_string());
            return;
        }

        // Check for existing username
        if db.users.contains_key(&self.username) {
            self.error_message = Some("Username already exists.".to_string());
            return;
        }

        // Parse numeric values
        let height_cm = self.height_cm.parse().unwrap_or(0.0);
        let age = self.age.parse().unwrap_or(0);
        let weight_kg = self.weight_kg.parse().unwrap_or(0.0);

        // Validate inputs
        if height_cm <= 0.0 || age <= 0 || weight_kg <= 0.0 {
            self.error_message = Some("Invalid height, age, or weight.".to_string());
            return;
        }

        // Create user
        let user_id = Uuid::new_v4().to_string();
        let profile = UserProfile {
            gender: self.gender.clone(),
            height_cm,
            age,
            calorie_method: self.calorie_method.clone(),
            weight_kg,
            activity_level: self.activity_level.clone(),
        };

        let user = User {
            user_id: user_id.clone(),
            username: self.username.clone(),
            password: self.password.clone(),
            profile,
        };

        // Insert new user and set as current user
        db.users.insert(self.username.clone(), user);
        db.current_user = user_id;

        *current_state = AppState::Home;
    }
}
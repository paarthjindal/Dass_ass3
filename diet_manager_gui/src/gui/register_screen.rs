// src/gui/register_screen.rs
use eframe::egui;
use crate::models::{Database, User, UserProfile, Gender, ActivityLevel, CalorieCalculationMethod};
use crate::app_state::AppState;
use uuid::Uuid;
use std::fs;

pub struct RegisterScreen {
    username: String,
    password: String,
    gender: Gender,
    height_cm: String,
    age: String,
    weight_kg: String, // Added weight field
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
            weight_kg: String::new(), // Initialize weight field
            activity_level: ActivityLevel::Sedentary,
            calorie_method: CalorieCalculationMethod::HarrisBenedict,
            error_message: None,
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, db: &mut Database, current_state: &mut AppState) {
        ui.heading("Register");

        if let Some(error) = &self.error_message {
            ui.label(error);
        }

        ui.horizontal(|ui| {
            ui.label("Username:");
            ui.text_edit_singleline(&mut self.username);
        });

        ui.horizontal(|ui| {
            ui.label("Password:");
            ui.text_edit_singleline(&mut self.password);
        });

        ui.horizontal(|ui| {
            ui.label("Gender:");
            ui.radio_value(&mut self.gender, Gender::Male, "Male");
            ui.radio_value(&mut self.gender, Gender::Female, "Female");
        });

        ui.horizontal(|ui| {
            ui.label("Height (cm):");
            ui.text_edit_singleline(&mut self.height_cm);
        });

        ui.horizontal(|ui| {
            ui.label("Age:");
            ui.text_edit_singleline(&mut self.age);
        });

        ui.horizontal(|ui| {
            ui.label("Weight (kg):"); // Added weight input
            ui.text_edit_singleline(&mut self.weight_kg);
        });

        ui.horizontal(|ui| {
            ui.label("Activity Level:");
            ui.radio_value(&mut self.activity_level, ActivityLevel::Sedentary, "Sedentary");
            ui.radio_value(&mut self.activity_level, ActivityLevel::Light, "Light");
            ui.radio_value(&mut self.activity_level, ActivityLevel::Moderate, "Moderate");
            ui.radio_value(&mut self.activity_level, ActivityLevel::VeryActive, "Very Active");
            ui.radio_value(&mut self.activity_level, ActivityLevel::ExtraActive, "Extra Active");
        });

        ui.horizontal(|ui| {
            ui.label("Calorie Calculation Method:");
            ui.radio_value(&mut self.calorie_method, CalorieCalculationMethod::HarrisBenedict, "Harris-Benedict");
            ui.radio_value(&mut self.calorie_method, CalorieCalculationMethod::MifflinStJeor, "Mifflin-St Jeor");
        });

        if ui.button("Register").clicked() {
            if self.username.is_empty() || self.password.is_empty() {
                self.error_message = Some("Username and password are required.".to_string());
            } else if db.users.contains_key(&self.username) {
                self.error_message = Some("Username already exists.".to_string());
            } else {
                let height_cm = self.height_cm.parse().unwrap_or(0.0);
                let age = self.age.parse().unwrap_or(0);
                let weight_kg = self.weight_kg.parse().unwrap_or(0.0);

                if height_cm <= 0.0 || age <= 0 || weight_kg <= 0.0 {
                    self.error_message = Some("Invalid height, age, or weight.".to_string());
                } else {
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

                    // Save the database to JSON file
                    if let Err(e) = self.save_database(db) {
                        self.error_message = Some(format!("Failed to save registration: {}", e));
                        return;
                    }

                    *current_state = AppState::Home;
                }
            }
        }

        if ui.button("Back to Login").clicked() {
            *current_state = AppState::Login;
        }
    }

    fn save_database(&self, db: &Database) -> Result<(), Box<dyn std::error::Error>> {
        // Load existing database to preserve food data
        let mut existing_db = if let Ok(data) = std::fs::read_to_string("database.json") {
            serde_json::from_str(&data).unwrap_or_else(|_| Database::default())
        } else {
            Database::default()
        };

        // Update only the users and current_user fields
        existing_db.users = db.users.clone();
        existing_db.current_user = db.current_user.clone();

        // Save the merged database back
        let json = serde_json::to_string_pretty(&existing_db)?;
        std::fs::write("database.json", json)?;
        Ok(())
    }
}
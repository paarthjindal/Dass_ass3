use eframe::egui;
use crate::models::{Database, Gender, ActivityLevel, CalorieCalculationMethod};
use crate::app_state::AppState;
use uuid::Uuid;

pub struct UpdateProfileScreen;

impl UpdateProfileScreen {
    pub fn new() -> Self {
        Self
    }

    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        db: &mut Database,
        current_state: &mut AppState,
    ) {
        ui.heading("Update Profile");

        if let Some(user) = db.users.get_mut(&db.current_user) {
            // Gender selection
            ui.label("Gender:");
            ui.horizontal(|ui| {
                if ui.button("Male").clicked() {
                    user.profile.gender = Gender::Male;
                }
                if ui.button("Female").clicked() {
                    user.profile.gender = Gender::Female;
                }
            });

            // Height input
            ui.label("Height (cm):");
            ui.add(egui::Slider::new(&mut user.profile.height_cm, 100.0..=250.0));

            // Age input
            ui.label("Age:");
            ui.add(egui::Slider::new(&mut user.profile.age, 1..=120));

            // Weight input
            ui.label("Weight (kg):");
            ui.add(egui::Slider::new(&mut user.profile.weight_kg, 30.0..=200.0));

            // Activity level selection
            ui.label("Activity Level:");
            ui.horizontal(|ui| {
                if ui.button("Sedentary").clicked() {
                    user.profile.activity_level = ActivityLevel::Sedentary;
                }
                if ui.button("Light").clicked() {
                    user.profile.activity_level = ActivityLevel::Light;
                }
                if ui.button("Moderate").clicked() {
                    user.profile.activity_level = ActivityLevel::Moderate;
                }
                if ui.button("Very Active").clicked() {
                    user.profile.activity_level = ActivityLevel::VeryActive;
                }
                if ui.button("Extra Active").clicked() {
                    user.profile.activity_level = ActivityLevel::ExtraActive;
                }
            });

            // Calorie calculation method selection
            ui.label("Calorie Calculation Method:");
            ui.horizontal(|ui| {
                if ui.button("Harris-Benedict").clicked() {
                    user.profile.calorie_method = CalorieCalculationMethod::HarrisBenedict;
                }
                if ui.button("Mifflin-St Jeor").clicked() {
                    user.profile.calorie_method = CalorieCalculationMethod::MifflinStJeor;
                }
            });
        }

        // Back button
        if ui.button("Back").clicked() {
            *current_state = AppState::Home;
        }

        // Save button
        if ui.button("Save").clicked() {
            // Save changes to the database
            // This might involve writing back to a persistent storage if needed
            println!("Profile updated for user: {}", db.current_user);
        }
    }
}
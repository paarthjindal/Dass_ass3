use eframe::egui;
use crate::models::{Database, BasicFood};
use crate::app_state::AppState;
use crate::gui::styling;

pub struct AddBasicFoodScreen {
    new_food_id: String,
    new_food_keywords: String,
    new_food_calories: String,
}

impl AddBasicFoodScreen {
    pub fn new() -> Self {
        Self {
            new_food_id: String::new(),
            new_food_keywords: String::new(),
            new_food_calories: String::new(),
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, db: &mut Database, current_state: &mut AppState) {
        ui.vertical_centered(|ui| {
            ui.heading(egui::RichText::new("Add Basic Food").size(28.0).strong());
            ui.add_space(4.0);
            ui.label("Create a new basic food item");
            ui.add_space(20.0);
        });

        styling::card_frame().show(ui, |ui| {
            // Food ID field with enhanced styling
            ui.add_space(8.0);
            ui.heading(egui::RichText::new("Food Details").size(20.0));
            ui.add_space(12.0);

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("🏷️").size(20.0));
                ui.vertical(|ui| {
                    ui.label("Food Identifier:");
                    ui.add(egui::TextEdit::singleline(&mut self.new_food_id)
                        .hint_text("Enter unique identifier (e.g., apple)")
                        .desired_width(300.0));
                });
            });

            ui.add_space(12.0);

            // Keywords field with enhanced styling
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("🔍").size(20.0));
                ui.vertical(|ui| {
                    ui.label("Keywords:");
                    ui.add(egui::TextEdit::singleline(&mut self.new_food_keywords)
                        .hint_text("Enter comma-separated keywords (e.g., fruit, sweet)")
                        .desired_width(300.0));
                    ui.label(egui::RichText::new("Separate keywords with commas").size(12.0));
                });
            });

            ui.add_space(12.0);

            // Calories field with enhanced styling
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("🔥").size(20.0));
                ui.vertical(|ui| {
                    ui.label("Calories per serving:");
                    ui.add(egui::TextEdit::singleline(&mut self.new_food_calories)
                        .hint_text("Enter calories per serving (e.g., 95)")
                        .desired_width(300.0));
                });
            });

            ui.add_space(24.0);

            // Save and Cancel buttons
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if styling::warning_button(ui, "Cancel").clicked() {
                        // Clear fields and return to home
                        self.new_food_id.clear();
                        self.new_food_keywords.clear();
                        self.new_food_calories.clear();
                        *current_state = AppState::Home;
                    }

                    ui.add_space(10.0);

                    if styling::success_button(ui, "Save Food").clicked() {
                        self.save_food(db, current_state);
                    }
                });
            });
        });
    }

    fn save_food(&mut self, db: &mut Database, current_state: &mut AppState) {
        let keywords = self.new_food_keywords
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        let calories = self.new_food_calories.parse().unwrap_or(0.0);

        let food = BasicFood {
            id: self.new_food_id.clone(),
            name: self.new_food_id.clone(), // Use identifier as name
            keywords,
            calories_per_serving: calories,
        };

        db.basic_foods.insert(self.new_food_id.clone(), food);

        // Clear fields for next use
        self.new_food_id.clear();
        self.new_food_keywords.clear();
        self.new_food_calories.clear();

        *current_state = AppState::Home;
    }
}
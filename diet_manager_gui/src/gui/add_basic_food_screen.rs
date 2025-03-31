use eframe::egui;
use crate::models::{Database, BasicFood};
use crate::app_state::AppState;
use crate::gui::styling;
use crate::gui::undo_manager::UndoManager;
pub struct AddBasicFoodScreen {
    new_food_id: String,
    new_food_keywords: String,
    new_food_calories: String,
    error_message: Option<String>,
}

impl AddBasicFoodScreen {
    pub fn new() -> Self {
        Self {
            new_food_id: String::new(),
            new_food_keywords: String::new(),
            new_food_calories: String::new(),
            error_message: None,
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, db: &mut Database, current_state: &mut AppState, undo_manager: &mut UndoManager) {
        ui.vertical_centered(|ui| {
            ui.heading(egui::RichText::new("Add Basic Food").size(28.0).strong());
            ui.add_space(4.0);
            ui.label("Create a new basic food item");
            ui.add_space(20.0);
        });

        // Display error message if any
        if let Some(error) = &self.error_message {
            ui.colored_label(egui::Color32::RED, error);
            ui.add_space(12.0);
        }

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
                        self.reset();
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

    fn reset(&mut self) {
        self.new_food_id.clear();
        self.new_food_keywords.clear();
        self.new_food_calories.clear();
        self.error_message = None;
    }

    fn save_food(&mut self, db: &mut Database, current_state: &mut AppState) {
        // Clear previous error
        self.error_message = None;

        // Validate food ID
        if self.new_food_id.trim().is_empty() {
            self.error_message = Some("Food Identifier cannot be empty".to_string());
            return;
        }

        // Validate keywords
        if self.new_food_keywords.trim().is_empty() {
            self.error_message = Some("Keywords cannot be empty".to_string());
            return;
        }

        // Validate calories
        let calories = match self.new_food_calories.parse::<f32>() {
            Ok(cal) => {
                if cal <= 0.0 {
                    self.error_message = Some("Calories must be a positive number".to_string());
                    return;
                }
                cal
            },
            Err(_) => {
                self.error_message = Some("Invalid calories value. Please enter a number".to_string());
                return;
            }
        };

        // Check for duplicate food ID
        if db.basic_foods.contains_key(&self.new_food_id) {
            self.error_message = Some("A food with this identifier already exists".to_string());
            return;
        }

        // Validate and process keywords
        let keywords = self.new_food_keywords
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<String>>();

        if keywords.is_empty() {
            self.error_message = Some("Please provide at least one valid keyword".to_string());
            return;
        }

        let food = BasicFood {
            id: self.new_food_id.clone(),
            name: self.new_food_id.clone(), // Use identifier as name
            keywords,
            calories_per_serving: calories,
        };

        db.basic_foods.insert(self.new_food_id.clone(), food);

        // Reset fields and return to home
        self.reset();
        *current_state = AppState::Home;
    }
}
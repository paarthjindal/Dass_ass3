use eframe::egui;
use crate::models::{Database, CompositeFood, FoodComponent};
use crate::app_state::AppState;

pub struct AddCompositeFoodScreen {
    new_food_id: String,
    new_food_name: String,
    new_food_keywords: String,
    selected_components: Vec<FoodComponent>,
}

impl AddCompositeFoodScreen {
    pub fn new() -> Self {
        Self {
            new_food_id: String::new(),
            new_food_name: String::new(),
            new_food_keywords: String::new(),
            selected_components: Vec::new(),
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, db: &mut Database, current_state: &mut AppState) {
        ui.heading("Add Composite Food");

        ui.horizontal(|ui| {
            ui.label("Food Identifier:");
            ui.text_edit_singleline(&mut self.new_food_id);
        });

        ui.horizontal(|ui| {
            ui.label("Food Name:");
            ui.text_edit_singleline(&mut self.new_food_name);
        });

        ui.horizontal(|ui| {
            ui.label("Keywords (comma-separated):");
            ui.text_edit_singleline(&mut self.new_food_keywords);
        });

        if ui.button("Add Component").clicked() {
            let food_id = "example_food_id".to_string(); // Replace with actual selection logic
            let servings = 1.0; // Replace with actual input logic
            self.selected_components.push(FoodComponent { food_id, servings });
        }

        if ui.button("Save").clicked() {
            let keywords = self.new_food_keywords.split(',').map(|s| s.trim().to_string()).collect();

            let food = CompositeFood {
                id: self.new_food_id.clone(),
                name: self.new_food_name.clone(),
                keywords,
                components: self.selected_components.clone(),
            };

            db.composite_foods.insert(self.new_food_id.clone(), food);
            *current_state = AppState::Home;
        }

        if ui.button("Cancel").clicked() {
            *current_state = AppState::Home;
        }
    }
}
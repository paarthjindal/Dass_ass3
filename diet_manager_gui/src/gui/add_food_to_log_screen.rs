use eframe::egui;
use crate::models::{Database, FoodLogEntry};
use crate::app_state::AppState;

pub struct AddFoodToLogScreen {
    selected_food_id: String,
    servings: f32,
    keywords: String,
    match_all_keywords: bool,
    selected_date: String, // ISO 8601 date format (e.g., "2023-10-01")
}

impl AddFoodToLogScreen {
    pub fn new() -> Self {
        Self {
            selected_food_id: String::new(),
            servings: 1.0,
            keywords: String::new(),
            match_all_keywords: true,
            selected_date: chrono::Local::now().format("%Y-%m-%d").to_string(),
        }
    }

    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        db: &mut Database,
        current_state: &mut AppState,
    ) {
        ui.heading("Add Food to Log");

        // Date selection
        ui.label("Select Date:");
        ui.text_edit_singleline(&mut self.selected_date);
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        if self.selected_date > today {
            ui.label(egui::RichText::new("Cannot select a future date.").color(egui::Color32::RED));
        }

        // Display selected item
        if !self.selected_food_id.is_empty() {
            ui.label(format!("Selected Food: {}", self.selected_food_id));
        }

        // Keyword filtering
        ui.label("Filter by Keywords:");
        ui.text_edit_singleline(&mut self.keywords);
        ui.checkbox(&mut self.match_all_keywords, "Match all keywords");

        // Food selection
        ui.label("Select Food:");
        let keywords: Vec<&str> = self.keywords.split_whitespace().collect();
        for (id, food) in &db.basic_foods {
            if self.matches_keywords(&food.keywords, &keywords) {
                if ui.button(&food.name).clicked() {
                    self.selected_food_id = id.clone();
                }
            }
        }
        for (id, food) in &db.composite_foods {
            if self.matches_keywords(&food.keywords, &keywords) {
                if ui.button(&food.name).clicked() {
                    self.selected_food_id = id.clone();
                }
            }
        }

        // Servings input
        ui.label("Servings:");
        ui.add(egui::Slider::new(&mut self.servings, 0.1..=10.0));

        // Add to log button
        if ui.button("Add to Log").clicked() {
            if self.selected_food_id.is_empty() || self.servings <= 0.0 {
                ui.label(egui::RichText::new("Please select a food and enter a valid serving size.").color(egui::Color32::RED));
            } else if self.selected_date > today {
                ui.label(egui::RichText::new("Cannot add to a future date.").color(egui::Color32::RED));
            } else {
                let entry = FoodLogEntry {
                    date: self.selected_date.clone(),
                    food_id: self.selected_food_id.clone(),
                    servings: self.servings,
                    user_id: db.current_user.clone(), // Add user_id to entry
                };
                db.food_logs
                    .entry(db.current_user.clone())
                    .or_insert_with(Vec::new)
                    .push(entry);
                *current_state = AppState::Home; // Return to home screen
            }
        }
    }

    fn matches_keywords(&self, food_keywords: &[String], filter_keywords: &[&str]) -> bool {
        if filter_keywords.is_empty() {
            return true;
        }
        if self.match_all_keywords {
            filter_keywords
                .iter()
                .all(|kw| food_keywords.iter().any(|fk| fk.contains(kw)))
        } else {
            filter_keywords
                .iter()
                .any(|kw| food_keywords.iter().any(|fk| fk.contains(kw)))
        }
    }
}
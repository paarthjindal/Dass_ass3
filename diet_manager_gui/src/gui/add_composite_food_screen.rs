use eframe::egui;
use crate::models::{Database, CompositeFood, FoodComponent};
use crate::app_state::AppState;
use crate::gui::styling;

pub struct AddCompositeFoodScreen {
    new_food_id: String,
    new_food_name: String,
    new_food_keywords: String,
    selected_components: Vec<FoodComponent>,
    current_food_id: String,
    current_servings: String,
    search_term: String,
    error_message: Option<String>,
}

impl AddCompositeFoodScreen {
    pub fn new() -> Self {
        Self {
            new_food_id: String::new(),
            new_food_name: String::new(),
            new_food_keywords: String::new(),
            selected_components: Vec::new(),
            current_food_id: String::new(),
            current_servings: "1.0".to_string(),
            search_term: String::new(),
            error_message: None,
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, db: &mut Database, current_state: &mut AppState) {
        
        ui.vertical_centered(|ui| {
            ui.heading(egui::RichText::new("Create Composite Food").size(28.0).strong());
            ui.add_space(4.0);
            ui.label("Combine multiple foods into a recipe or meal");
            ui.add_space(20.0);
        });
        egui::ScrollArea::vertical().show(ui, |ui| {

        styling::card_frame().show(ui, |ui| {
            // Basic food information
            styling::section_header(ui, "Food Details");

            // Food ID field
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("🆔").size(20.0));
                ui.vertical(|ui| {
                    ui.label("Food Identifier:");
                    ui.add(egui::TextEdit::singleline(&mut self.new_food_id)
                        .hint_text("Enter unique identifier (e.g., chicken_salad)")
                        .desired_width(300.0));
                });
            });

            ui.add_space(8.0);

            // Food Name field
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("📝").size(20.0));
                ui.vertical(|ui| {
                    ui.label("Food Name:");
                    ui.add(egui::TextEdit::singleline(&mut self.new_food_name)
                        .hint_text("Enter descriptive name (e.g., Chicken Caesar Salad)")
                        .desired_width(300.0));
                });
            });

            ui.add_space(8.0);

            // Keywords field
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("🔍").size(20.0));
                ui.vertical(|ui| {
                    ui.label("Keywords:");
                    ui.add(egui::TextEdit::singleline(&mut self.new_food_keywords)
                        .hint_text("Enter comma-separated keywords (e.g., salad, protein, lunch)")
                        .desired_width(300.0));
                });
            });

            ui.add_space(16.0);

            // Components section
            styling::section_header(ui, "Food Components");

            if self.selected_components.is_empty() {
                ui.label(egui::RichText::new("No components added yet").italics());
            } else {
                let mut to_remove = None;

                ui.push_id("components_list", |ui| {
                    for (index, component) in self.selected_components.iter().enumerate() {
                        let food_name = self.get_food_name(db, &component.food_id);
                        let calories = self.get_food_calories(db, &component.food_id) * component.servings;

                        ui.horizontal(|ui| {
                            ui.label(format!("{}. {} (x{:.1} servings, {:.0} kcal)",
                                index + 1,
                                food_name,
                                component.servings,
                                calories));

                            if ui.button(egui::RichText::new("❌").color(styling::AppTheme::default().error_color)).clicked() {
                                to_remove = Some(index);
                            }
                        });
                    }
                });

                // Remove item outside the iterator if needed
                if let Some(index) = to_remove {
                    self.selected_components.remove(index);
                }

                // Show total calories
                let total_calories: f32 = self.selected_components.iter()
                    .map(|component| self.get_food_calories(db, &component.food_id) * component.servings)
                    .sum();

                ui.add_space(8.0);
                ui.label(egui::RichText::new(format!("Total calories: {:.0} kcal", total_calories))
                    .strong()
                    .size(16.0));
            }

            ui.add_space(12.0);

            // Add new component section
            ui.push_id("add_component", |ui| {
                ui.label(egui::RichText::new("Add Component").strong());

                // Search field
                ui.horizontal(|ui| {
                    ui.label("Search:");
                    ui.add(egui::TextEdit::singleline(&mut self.search_term)
                        .hint_text("Search foods")
                        .desired_width(200.0));
                });

                // Food selection grid
                let mut available_foods: Vec<(&String, &str, f32)> = Vec::new();

                // Add basic foods to selection
                for (id, food) in &db.basic_foods {
                    if self.search_term.is_empty() ||
                       id.to_lowercase().contains(&self.search_term.to_lowercase()) ||
                       food.name.to_lowercase().contains(&self.search_term.to_lowercase()) {
                        available_foods.push((id, &food.name, food.calories_per_serving));
                    }
                }

                ui.label("Select food:");
                egui::ScrollArea::vertical()
                    .max_height(150.0)
                    .show(ui, |ui| {
                        egui::Grid::new("foods_grid")
                            .striped(true)
                            .spacing([8.0, 4.0])
                            .show(ui, |ui| {
                                for (id, name, calories) in available_foods {
                                    ui.radio_value(&mut self.current_food_id, id.clone(), format!("{} ({:.0} kcal)", name, calories));
                                    ui.end_row();
                                }
                            });
                    });

                // Servings input
                ui.horizontal(|ui| {
                    ui.label("Servings:");
                    ui.add(egui::TextEdit::singleline(&mut self.current_servings)
                        .desired_width(100.0));
                });

                // Add button
                ui.horizontal(|ui| {
                    if styling::primary_button(ui, "Add to Recipe").clicked() {
                        self.add_component(ui);
                    }
                });
            });

            ui.add_space(20.0);

            // Error message
            if let Some(ref error) = self.error_message {
                ui.colored_label(
                    styling::AppTheme::default().error_color,
                    egui::RichText::new(error).size(14.0).strong()
                );
                ui.add_space(8.0);
            }

            // Save and Cancel buttons
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if styling::warning_button(ui, "Cancel").clicked() {
                        self.clear_fields();
                        *current_state = AppState::Home;
                    }

                    ui.add_space(10.0);

                    if styling::success_button(ui, "Save Food").clicked() {
                        self.save_composite_food(db, current_state);
                    }
                });
            });
        });
        });
    }


    fn add_component(&mut self, _ui: &mut egui::Ui) {
        // Clear previous error
        self.error_message = None;

        // Validate food selection
        if self.current_food_id.is_empty() {
            self.error_message = Some("Please select a food to add".to_string());
            return;
        }

        // Validate servings
        let servings = match self.current_servings.parse::<f32>() {
            Ok(value) if value > 0.0 => value,
            _ => {
                self.error_message = Some("Please enter a valid number of servings (must be positive)".to_string());
                return;
            }
        };

        // Check if component already exists, if so update servings
        if let Some(component) = self.selected_components.iter_mut()
            .find(|c| c.food_id == self.current_food_id) {
            component.servings += servings;
        } else {
            self.selected_components.push(FoodComponent {
                food_id: self.current_food_id.clone(),
                servings,
            });
        }

        // Reset selection fields
        self.current_food_id.clear();
        self.current_servings = "1.0".to_string();
    }


     fn save_composite_food(&mut self, db: &mut Database, current_state: &mut AppState) {
        // Clear previous error
        self.error_message = None;

        // Validate food ID
        if self.new_food_id.trim().is_empty() {
            self.error_message = Some("Food Identifier cannot be empty".to_string());
            return;
        }

        // Check for duplicate food ID
        if db.basic_foods.contains_key(&self.new_food_id) || 
           db.composite_foods.contains_key(&self.new_food_id) {
            self.error_message = Some("A food with this identifier already exists".to_string());
            return;
        }

        // Validate food name
        if self.new_food_name.trim().is_empty() {
            self.error_message = Some("Food Name cannot be empty".to_string());
            return;
        }

        // Validate keywords
        let keywords = self.new_food_keywords
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<String>>();

        if keywords.is_empty() {
            self.error_message = Some("Please provide at least one valid keyword".to_string());
            return;
        }

        // Validate components
        if self.selected_components.is_empty() {
            self.error_message = Some("At least one component is required".to_string());
            return;
        }

        // Create and save the composite food
        let food = CompositeFood {
            id: self.new_food_id.clone(),
            name: self.new_food_name.clone(),
            keywords,
            components: self.selected_components.clone(),
        };

        db.composite_foods.insert(self.new_food_id.clone(), food);

        // Reset form and return to home
        self.clear_fields();
        *current_state = AppState::Home;
    }
     fn clear_fields(&mut self) {
        self.new_food_id.clear();
        self.new_food_name.clear();
        self.new_food_keywords.clear();
        self.selected_components.clear();
        self.current_food_id.clear();
        self.current_servings = "1.0".to_string();
        self.search_term.clear();
        self.error_message = None;
    }

    // Helper to get food name
    fn get_food_name(&self, db: &Database, food_id: &str) -> String {
        if let Some(food) = db.basic_foods.get(food_id) {
            food.name.clone()
        } else if let Some(food) = db.composite_foods.get(food_id) {
            food.name.clone()
        } else {
            food_id.to_string()
        }
    }

    // Helper to get food calories
    fn get_food_calories(&self, db: &Database, food_id: &str) -> f32 {
        db.get_food_calories(food_id).unwrap_or(0.0)
    }
}
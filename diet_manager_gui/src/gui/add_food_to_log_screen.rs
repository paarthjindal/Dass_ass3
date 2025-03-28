use eframe::egui;
use chrono::{ Local, NaiveDate };
use crate::models::{ Database, FoodLogEntry };
use crate::app_state::AppState;
use crate::gui::styling;
use crate::models::{ BasicFood, CompositeFood};
pub struct AddFoodToLogScreen {
    selected_food_id: String,
    servings: f32,
    keywords: String,
    match_all_keywords: bool,
    selected_date: NaiveDate,
    error_message: Option<String>,
    show_basic_foods: bool,
    show_composite_foods: bool,
}

impl AddFoodToLogScreen {
    pub fn new() -> Self {
        Self {
            selected_food_id: String::new(),
            servings: 1.0,
            keywords: String::new(),
            match_all_keywords: false,
            selected_date: Local::now().date_naive(),
            error_message: None,
            show_basic_foods: true,
            show_composite_foods: true,
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, db: &mut Database, current_state: &mut AppState) {
        ui.vertical_centered(|ui| {
            ui.heading(egui::RichText::new("Add Food to Log").size(28.0).strong());
            ui.add_space(4.0);
            ui.label("Record what you've eaten today");
            ui.add_space(20.0);
        });

        styling::card_frame().show(ui, |ui| {
            // Date selection
            styling::section_header(ui, "Date");
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("📅").size(20.0));

                if ui.button("◀").clicked() {
                    self.selected_date = self.selected_date
                        .pred_opt()
                        .unwrap_or(self.selected_date);
                }

                ui.label(
                    egui::RichText
                        ::new(self.selected_date.format("%A, %B %d, %Y").to_string())
                        .size(16.0)
                );

                let today = Local::now().date_naive();
                let is_future = self.selected_date >= today.succ_opt().unwrap_or(today);
                let next_button = ui.button("▶");

                if next_button.clicked() && !is_future {
                    self.selected_date = self.selected_date
                        .succ_opt()
                        .unwrap_or(self.selected_date);
                }

                if is_future {
                    ui.label(
                        egui::RichText
                            ::new("Cannot select future dates")
                            .color(styling::AppTheme::default().error_color)
                            .size(14.0)
                    );
                }
            });

            ui.add_space(16.0);

            // Display selected item
            if !self.selected_food_id.is_empty() {
                styling::section_header(ui, "Selected Food");
                ui.horizontal(|ui| {
                    let food_name = self.get_food_name(db, &self.selected_food_id);
                    let calories_per_serving = db
                        .get_food_calories(&self.selected_food_id)
                        .unwrap_or(0.0);
                    let total_calories = calories_per_serving * self.servings;

                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new(&food_name).size(18.0).strong());
                        ui.label(format!("{:.0} kcal per serving", calories_per_serving));
                        ui.label(
                            egui::RichText
                                ::new(format!("Total: {:.0} kcal", total_calories))
                                .color(styling::AppTheme::default().accent_color)
                                .strong()
                        );
                    });
                });

                ui.add_space(8.0);

                // Servings input
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Servings:").strong());
                    ui.add(
                        egui::Slider
                            ::new(&mut self.servings, 0.1..=10.0)
                            .text("servings")
                            .clamp_to_range(true)
                            .smart_aim(false)
                            .step_by(0.1)
                    );
                });

                ui.add_space(16.0);
            }

            // Food search and selection
            styling::section_header(ui, "Find Food");

            // Filter options
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("🔍").size(20.0));
                ui.add(
                    egui::TextEdit
                        ::singleline(&mut self.keywords)
                        .hint_text("Search by name or keywords")
                        .desired_width(300.0)
                );
            });

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.match_all_keywords, "Match all search terms");
                ui.add_space(16.0);
                ui.checkbox(&mut self.show_basic_foods, "Basic Foods");
                ui.add_space(8.0);
                ui.checkbox(&mut self.show_composite_foods, "Composite Foods");
            });

            ui.add_space(8.0);
            // Replace the food search code with this approach:

            // Food selection
            styling::section_header(ui, "Available Foods");

            egui::ScrollArea
                ::vertical()
                .max_height(300.0)
                .show(ui, |ui| {
                    let keywords_vec: Vec<String> = self.keywords
                        .split_whitespace()
                        .map(|s| s.to_string())
                        .collect();
                    let mut found_foods = false;

                    // Show basic foods - precompute matches
                    if self.show_basic_foods {
                        // Create a vector of matching foods to avoid borrowing issues
                        let matching_foods: Vec<(&String, &BasicFood)> = db.basic_foods
                            .iter()
                            .filter(|(_, food)| {
                                if keywords_vec.is_empty() {
                                    return true;
                                }

                                if self.match_all_keywords {
                                    keywords_vec
                                        .iter()
                                        .all(|kw|
                                            food.keywords
                                                .iter()
                                                .any(|fk|
                                                    fk.to_lowercase().contains(&kw.to_lowercase())
                                                )
                                        )
                                } else {
                                    keywords_vec
                                        .iter()
                                        .any(|kw|
                                            food.keywords
                                                .iter()
                                                .any(|fk|
                                                    fk.to_lowercase().contains(&kw.to_lowercase())
                                                )
                                        )
                                }
                            })
                            .collect();

                        ui.collapsing(egui::RichText::new("Basic Foods").size(16.0).strong(), |ui| {
                            egui::Grid
                                ::new("basic_foods_grid")
                                .striped(true)
                                .spacing([10.0, 6.0])
                                .show(ui, |ui| {
                                    // Header row
                                    ui.label(egui::RichText::new("Name").strong());
                                    ui.label(egui::RichText::new("Calories").strong());
                                    ui.label(egui::RichText::new("Keywords").strong());
                                    ui.label("");
                                    ui.end_row();

                                    // Food rows
                                    let mut foods_shown = false;
                                    for (id, food) in matching_foods {
                                        foods_shown = true;
                                        found_foods = true;

                                        ui.label(&food.name);
                                        ui.label(format!("{:.0} kcal", food.calories_per_serving));
                                        ui.label(food.keywords.join(", "));

                                        let is_selected = self.selected_food_id == *id;
                                        if
                                            ui
                                                .selectable_label(is_selected, if is_selected {
                                                    "Selected"
                                                } else {
                                                    "Select"
                                                })
                                                .clicked()
                                        {
                                            self.selected_food_id = id.clone();
                                        }
                                        ui.end_row();
                                    }

                                    if !foods_shown {
                                        ui.label("No matching basic foods found");
                                        ui.end_row();
                                    }
                                });
                        });
                    }

                    // Show composite foods - same approach
                    if self.show_composite_foods {
                        // Create a vector of matching foods to avoid borrowing issues
                        let matching_foods: Vec<(&String, &CompositeFood)> = db.composite_foods
                            .iter()
                            .filter(|(_, food)| {
                                if keywords_vec.is_empty() {
                                    return true;
                                }

                                if self.match_all_keywords {
                                    keywords_vec
                                        .iter()
                                        .all(|kw|
                                            food.keywords
                                                .iter()
                                                .any(|fk|
                                                    fk.to_lowercase().contains(&kw.to_lowercase())
                                                )
                                        )
                                } else {
                                    keywords_vec
                                        .iter()
                                        .any(|kw|
                                            food.keywords
                                                .iter()
                                                .any(|fk|
                                                    fk.to_lowercase().contains(&kw.to_lowercase())
                                                )
                                        )
                                }
                            })
                            .collect();

                        ui.collapsing(
                            egui::RichText::new("Composite Foods").size(16.0).strong(),
                            |ui| {
                                // ... same grid code as above but for composite foods
                                egui::Grid
                                    ::new("composite_foods_grid")
                                    .striped(true)
                                    .spacing([10.0, 6.0])
                                    .show(ui, |ui| {
                                        // Header row
                                        ui.label(egui::RichText::new("Name").strong());
                                        ui.label(egui::RichText::new("Calories").strong());
                                        ui.label(egui::RichText::new("Keywords").strong());
                                        ui.label("");
                                        ui.end_row();

                                        // Food rows
                                        let mut foods_shown = false;
                                        for (id, food) in matching_foods {
                                            foods_shown = true;
                                            found_foods = true;

                                            ui.label(&food.name);

                                            // Calculate total calories for composite food
                                            let calories = db.get_food_calories(id).unwrap_or(0.0);
                                            ui.label(format!("{:.0} kcal", calories));

                                            ui.label(food.keywords.join(", "));

                                            let is_selected = self.selected_food_id == *id;
                                            if
                                                ui
                                                    .selectable_label(is_selected, if is_selected {
                                                        "Selected"
                                                    } else {
                                                        "Select"
                                                    })
                                                    .clicked()
                                            {
                                                self.selected_food_id = id.clone();
                                            }
                                            ui.end_row();
                                        }

                                        if !foods_shown {
                                            ui.label("No matching composite foods found");
                                            ui.end_row();
                                        }
                                    });
                            }
                        );
                    }

                    if !found_foods {
                        ui.vertical_centered(|ui| {
                            ui.add_space(20.0);
                            ui.label(egui::RichText::new("No matching foods found").size(16.0));
                            ui.add_space(4.0);
                            ui.label("Try different search terms or create a new food");
                            ui.add_space(12.0);
                            if styling::primary_button(ui, "Create New Basic Food").clicked() {
                                *current_state = AppState::AddBasicFood;
                            }
                            ui.add_space(20.0);
                        });
                    }
                });

            // Remove the matches_keywords method as we've inlined the logic
            ui.add_space(16.0);

            // Error message
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
                        *current_state = AppState::Home;
                    }

                    ui.add_space(10.0);

                    if styling::success_button(ui, "Add to Log").clicked() {
                        self.add_to_log(db, current_state);
                    }
                });
            });
        });
    }

    fn matches_keywords(&self, food_keywords: &[String], filter_keywords: &[&str]) -> bool {
        if filter_keywords.is_empty() {
            return true;
        }

        if self.match_all_keywords {
            filter_keywords
                .iter()
                .all(|kw|
                    food_keywords.iter().any(|fk| fk.to_lowercase().contains(&kw.to_lowercase()))
                )
        } else {
            filter_keywords
                .iter()
                .any(|kw|
                    food_keywords.iter().any(|fk| fk.to_lowercase().contains(&kw.to_lowercase()))
                )
        }
    }

    fn add_to_log(&mut self, db: &mut Database, current_state: &mut AppState) {
        if self.selected_food_id.is_empty() {
            self.error_message = Some("Please select a food to add".to_string());
            return;
        }

        if self.servings <= 0.0 {
            self.error_message = Some("Please enter a valid number of servings".to_string());
            return;
        }

        let selected_date_str = self.selected_date.format("%Y-%m-%d").to_string();

        let entry = FoodLogEntry {
            date: selected_date_str,
            food_id: self.selected_food_id.clone(),
            servings: self.servings,
            user_id: db.current_user.clone(),
        };

        db.food_logs.entry(db.current_user.clone()).or_insert_with(Vec::new).push(entry);

        *current_state = AppState::Home;
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
}

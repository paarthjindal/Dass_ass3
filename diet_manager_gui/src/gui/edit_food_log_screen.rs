use eframe::egui;
use chrono::{ NaiveDate, Local };
use crate::models::{ Database, FoodLogEntry };
use crate::app_state::AppState;
use crate::gui::styling;
use crate::gui::undo_manager::UndoManager;

pub struct EditFoodLogScreen {
    selected_date: NaiveDate,
    editing_servings: f32,
    selected_entry_index: Option<usize>,
    editing_entry_index: Option<usize>, // Added to track which entry is being edited
}

impl EditFoodLogScreen {
    pub fn new() -> Self {
        Self {
            selected_date: Local::now().date_naive(),
            editing_servings: 1.0,
            selected_entry_index: None,
            editing_entry_index: None, // Initialize the new field
        }
    }

    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        db: &mut Database,
        current_state: &mut AppState,
        undo_manager: &mut UndoManager
    ) {
        ui.vertical_centered(|ui| {
            ui.heading(egui::RichText::new("Edit Food Log").size(28.0).strong());
            ui.add_space(4.0);
            ui.label("Modify or remove food entries from your log");
            ui.add_space(20.0);
        });

        styling::card_frame().show(ui, |ui| {
            // Date selection
            styling::section_header(ui, "Select Date");
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("📅").size(20.0));

                if ui.button("◀").clicked() {
                    self.selected_date = self.selected_date
                        .pred_opt()
                        .unwrap_or(self.selected_date);
                    self.editing_entry_index = None; // Reset when changing date
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
                    self.editing_entry_index = None; // Reset when changing date
                }
            });

            ui.add_space(16.0);

            // Display food entries
            styling::section_header(ui, "Food Entries");

            let selected_date_str = self.selected_date.format("%Y-%m-%d").to_string();

            // Check if there are any entries for this date
            let has_entries = if let Some(entries) = db.food_logs.get(&db.current_user) {
                entries.iter().any(|entry| entry.date == selected_date_str)
            } else {
                false
            };

            if !has_entries {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    ui.label(egui::RichText::new("No entries for this date").size(16.0).italics());
                    ui.add_space(8.0);
                    ui.label("Add some food to your log to edit it here");
                    ui.add_space(12.0);
                    if styling::primary_button(ui, "Add Food to Log").clicked() {
                        *current_state = AppState::AddFoodToLog;
                    }
                    ui.add_space(20.0);
                });
            } else {
                // First, collect all data we need to display
                // This avoids borrowing db while iterating
                let mut entries_data: Vec<(usize, FoodLogEntry, String, f32)> = Vec::new();

                if let Some(entries) = db.food_logs.get(&db.current_user) {
                    for (idx, entry) in entries.iter().enumerate() {
                        if entry.date == selected_date_str {
                            let food_name = self.get_food_name(db, &entry.food_id);
                            let calories =
                                db.get_food_calories(&entry.food_id).unwrap_or(0.0) *
                                entry.servings;
                            entries_data.push((idx, entry.clone(), food_name, calories));
                        }
                    }
                }

                // Track entries to remove
                let mut to_remove = Vec::new();

                // Display entries
                egui::ScrollArea
                    ::vertical()
                    .max_height(300.0)
                    .show(ui, |ui| {
                        // Create a table-like header
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Food").strong());
                            ui.add_space(40.0);
                            ui.label(egui::RichText::new("Servings").strong());
                            ui.add_space(40.0);
                            ui.label(egui::RichText::new("Calories").strong());
                            ui.add_space(40.0);
                            ui.label(egui::RichText::new("Actions").strong());
                        });

                        ui.separator();

                        // Use for-loop to avoid nested mutable references
                        for i in 0..entries_data.len() {
                            let db_index = entries_data[i].0;
                            let calories_per_serving = db
                                .get_food_calories(&entries_data[i].1.food_id)
                                .unwrap_or(0.0);

                            ui.push_id(db_index, |ui| {
                                ui.horizontal(|ui| {
                                    // Food name
                                    ui.label(egui::RichText::new(&entries_data[i].2).strong());

                                    ui.add_space(40.0);

                                    // Check if this is the entry we're currently editing
                                    if self.editing_entry_index != Some(i) {
                                        self.editing_servings = entries_data[i].1.servings;
                                    }

                                    // Servings editing - using DragValue for more precision
                                    ui.horizontal(|ui| {
                                        // Show the current value
                                        let response = ui.add(
                                            egui::DragValue::new(&mut self.editing_servings)
                                                .speed(0.1)
                                                .clamp_range(0.1..=10.0)
                                                .fixed_decimals(1)
                                                .suffix(" servings")
                                        );

                                        // If value changed, mark this as the active editing entry
                                        if response.changed() {
                                            self.editing_entry_index = Some(i);
                                        }

                                        // Update button
                                        if ui.button("✓").clicked() {
                                            // First gather all the info we need
                                            let food_name = entries_data[i].2.clone();
                                            let old_servings = entries_data[i].1.servings;
                                            let new_servings = self.editing_servings;

                                            if old_servings != new_servings {
                                                // Record state before change
                                                undo_manager.record_action(
                                                    db.clone(),
                                                    &format!(
                                                        "Changed servings of {} from {:.1} to {:.1}",
                                                        food_name, old_servings, new_servings
                                                    )
                                                );

                                                // Update the database
                                                if let Some(entries) = db.food_logs.get_mut(&db.current_user) {
                                                    if let Some(entry) = entries.get_mut(db_index) {
                                                        entry.servings = new_servings;

                                                        // Update our local copy too
                                                        entries_data[i].1.servings = new_servings;

                                                        // Save immediately
                                                        if let Err(e) = crate::database::save_database(db) {
                                                            eprintln!("Failed to save database: {}", e);
                                                        }
                                                    }
                                                }
                                            }

                                            // Done editing
                                            self.editing_entry_index = None;
                                        }
                                    });

                                    ui.add_space(40.0);

                                    // Calories - always show updated calculation
                                    let current_servings = if self.editing_entry_index == Some(i) {
                                        self.editing_servings
                                    } else {
                                        entries_data[i].1.servings
                                    };

                                    let updated_calories = calories_per_serving * current_servings;
                                    ui.label(format!("{:.0} kcal", updated_calories));

                                    ui.add_space(40.0);

                                    // Delete button
                                    if ui.button(
                                            egui::RichText::new("❌")
                                                .color(styling::AppTheme::default().error_color)
                                        ).clicked()
                                    {
                                        // Get food name for better description
                                        let food_name = entries_data[i].2.clone();

                                        // Record state before deletion
                                        undo_manager.record_action(
                                            db.clone(),
                                            &format!("Removed {} from food log", food_name)
                                        );

                                        to_remove.push(db_index);
                                    }
                                });

                                ui.separator();
                            });
                        }
                    });

                // Remove deleted entries (reverse order to avoid index issues)
                if !to_remove.is_empty() {
                    if let Some(entries) = db.food_logs.get_mut(&db.current_user) {
                        // Sort indices in descending order to safely remove
                        to_remove.sort_by(|a, b| b.cmp(a));
                        for index in to_remove {
                            entries.remove(index);
                        }

                        // Save after deletions
                        if let Err(e) = crate::database::save_database(db) {
                            eprintln!("Failed to save database after deletions: {}", e);
                        }
                    }
                }

                ui.add_space(16.0);
            }

            // Action buttons
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if styling::warning_button(ui, "Back to Home").clicked() {
                        *current_state = AppState::Home;
                    }

                    ui.add_space(10.0);

                    if styling::primary_button(ui, "Add More Food").clicked() {
                        *current_state = AppState::AddFoodToLog;
                    }
                });
            });
        });
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
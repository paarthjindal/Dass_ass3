use eframe::egui;
use chrono::{NaiveDate, Local, Duration};
// use crate::models::Database;
use crate::models::{Database, FoodLogEntry};
use crate::app_state::AppState;
use crate::gui::styling;
use crate::gui::undo_manager::UndoManager;
pub struct ViewDailyLogScreen {
    selected_date: NaiveDate,
}

impl ViewDailyLogScreen {
    pub fn new() -> Self {
        Self {
            selected_date: Local::now().date_naive(),
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, db: &mut Database, current_state: &mut AppState, undo_manager: &mut UndoManager) {
        ui.vertical_centered(|ui| {
            ui.heading(egui::RichText::new("Daily Nutrition Log").size(28.0).strong());
            ui.add_space(4.0);

            // Date navigation
            ui.horizontal(|ui| {
                if styling::primary_button(ui, "◀ Previous").clicked() {
                    self.selected_date -= Duration::days(1);
                }

                ui.label(egui::RichText::new(self.selected_date.format("%A, %B %d, %Y").to_string())
                    .size(18.0)
                    .strong());

                let tomorrow = self.selected_date + Duration::days(1);
                let today = Local::now().date_naive();
                let button = styling::primary_button(ui, "Next ▶");
                let is_future = tomorrow > today;

                if button.clicked() && !is_future {
                    self.selected_date += Duration::days(1);
                }

                if is_future {
                    ui.label(egui::RichText::new("Cannot view future dates")
                        .size(12.0)
                        .color(styling::AppTheme::default().warning_color));
                }
            });

            ui.add_space(20.0);
        });

        // Calculate nutrition data for the selected date
        let selected_date_str = self.selected_date.format("%Y-%m-%d").to_string();
        let (total_calories, calories_goal, calories_remaining) =
            self.calculate_daily_nutrition(db, &db.current_user, &selected_date_str);

        // Nutrition Summary Card
        styling::card_frame().show(ui, |ui| {
            styling::section_header(ui, "Nutrition Summary");

            ui.horizontal(|ui| {
                // Create nutrition summary with 3 columns
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new("Consumed").size(14.0));
                    ui.label(egui::RichText::new(format!("{:.0} kcal", total_calories)).size(24.0));
                });

                ui.add_space(40.0);

                ui.vertical(|ui| {
                    ui.label(egui::RichText::new("Goal").size(14.0));
                    ui.label(egui::RichText::new(format!("{:.0} kcal", calories_goal)).size(24.0));
                });

                ui.add_space(40.0);

                ui.vertical(|ui| {
                    let label = if calories_remaining >= 0.0 { "Remaining" } else { "Exceeded" };
                    let color = if calories_remaining >= 0.0 {
                        styling::AppTheme::default().success_color
                    } else {
                        styling::AppTheme::default().error_color
                    };

                    ui.label(egui::RichText::new(label).size(14.0));
                    ui.label(egui::RichText::new(format!("{:.0} kcal", calories_remaining.abs()))
                        .size(24.0)
                        .color(color));
                });
            });

            ui.add_space(12.0);

            // Progress bar showing percentage of daily goal consumed
            if calories_goal > 0.0 {
                let progress = (total_calories / calories_goal).min(1.0);
                let progress_text = format!("{:.1}% of daily goal", progress * 100.0);
                let progress_bar = egui::ProgressBar::new(progress)
                    .text(progress_text)
                    .animate(true);
                ui.add(progress_bar);
            }
        });

        ui.add_space(16.0);

        // Food Entries Card
        styling::card_frame().show(ui, |ui| {
            styling::section_header(ui, "Food Entries");

            // Collect entries first to avoid borrowing issues
            let entries: Vec<FoodLogEntry> = db.food_logs
                .get(&db.current_user)
                .map_or(Vec::new(), |entries| {
                    entries.iter()
                        .filter(|entry| entry.date == selected_date_str)
                        .cloned()
                        .collect()
                });

            if entries.is_empty() {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    ui.label(egui::RichText::new("No food entries for this date")
                        .size(16.0)
                        .italics());
                    ui.add_space(20.0);

                    if styling::primary_button(ui, "Add Food to Log").clicked() {
                        *current_state = AppState::AddFoodToLog;
                    }
                    ui.add_space(20.0);
                });
            } else {
                egui::ScrollArea::vertical()
                    .max_height(300.0)
                    .show(ui, |ui| {
                        // Create a table-like header
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("#").strong());
                            ui.add_space(20.0);
                            ui.label(egui::RichText::new("Food").strong());
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(egui::RichText::new("Delete").strong());
                                ui.add_space(10.0);
                                ui.label(egui::RichText::new("Calories").strong());
                                ui.add_space(20.0);
                                ui.label(egui::RichText::new("Servings").strong());
                            });
                        });

                        ui.separator();

                        for (i, entry) in entries.iter().enumerate() {
                            let food_name = db.basic_foods.get(&entry.food_id)
                                .map(|f| f.name.clone())
                                .or_else(|| db.composite_foods.get(&entry.food_id).map(|f| f.name.clone()))
                                .unwrap_or_else(|| entry.food_id.clone());

                            let calories = db.get_food_calories(&entry.food_id).unwrap_or(0.0) * entry.servings;

                            ui.horizontal(|ui| {
                                ui.label(format!("{}", i + 1));
                                ui.add_space(20.0);
                                ui.label(food_name);
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if ui.button(egui::RichText::new("❌").color(styling::AppTheme::default().error_color)).clicked() {
                                        // Get food name for better description
                                        let food_name = db.basic_foods.get(&entry.food_id)
                                            .map(|f| f.name.clone())
                                            .or_else(|| db.composite_foods.get(&entry.food_id).map(|f| f.name.clone()))
                                            .unwrap_or_else(|| entry.food_id.clone());

                                        // Record state before deletion
                                        undo_manager.record_action(db.clone(), &format!("Removed {} from food log", food_name));

                                        if let Some(entries) = db.food_logs.get_mut(&db.current_user) {
                                            if let Some(pos) = entries.iter().position(|e|
                                                e.date == entry.date &&
                                                e.food_id == entry.food_id &&
                                                e.servings == entry.servings &&
                                                e.user_id == entry.user_id
                                            ) {
                                                entries.remove(pos);
                                            }
                                        }
                                    }
                                    ui.add_space(10.0);
                                    ui.label(format!("{:.0} kcal", calories));
                                    ui.add_space(20.0);
                                    ui.label(format!("{:.1}", entry.servings));
                                });
                            });

                            if i < entries.len() - 1 {
                                ui.separator();
                            }
                        }
                    });

                ui.add_space(16.0);
                ui.horizontal(|ui| {
                    if styling::primary_button(ui, "Add More Food").clicked() {
                        *current_state = AppState::AddFoodToLog;
                    }
                });
            }
        });

        ui.add_space(16.0);

        // Navigation buttons
        ui.horizontal(|ui| {
            if styling::warning_button(ui, "Back to Home").clicked() {
                *current_state = AppState::Home;
            }
        });
    }

    fn calculate_daily_nutrition(
        &self,
        db: &Database,
        user_id: &str,
        date: &str
    ) -> (f32, f32, f32) {
        // Calculate total calories consumed
        let total_calories = db.food_logs
            .get(user_id)
            .map_or(0.0, |entries| {
                entries.iter()
                    .filter(|entry| entry.date == date)
                    .map(|entry| db.get_food_calories(&entry.food_id).unwrap_or(0.0) * entry.servings)
                    .sum()
            });

        // Get daily calorie goal from user profile
        let calories_goal = db.users.values()
            .find(|u| u.user_id == user_id)
            .map(|user| user.profile.calculate_target_calories())
            .unwrap_or(0.0);

        let calories_remaining = calories_goal - total_calories;

        (total_calories, calories_goal, calories_remaining)
    }
}
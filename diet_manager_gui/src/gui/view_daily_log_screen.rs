use eframe::egui;
use chrono::NaiveDate;
use crate::models::{Database, FoodLogEntry};
use crate::app_state::AppState;

pub struct ViewDailyLogScreen {
    selected_date: NaiveDate,
}

impl ViewDailyLogScreen {
    pub fn new() -> Self {
        Self {
            selected_date: chrono::Local::now().date_naive(),
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, db: &mut Database, current_state: &mut AppState) {
        ui.heading("Daily Log");

        // Date selection with bounds checking
        ui.horizontal(|ui| {
            ui.label("Select Date:");
            if ui.button("◄").clicked() {
                self.selected_date -= chrono::Duration::days(1);
            }
            ui.label(self.selected_date.format("%Y-%m-%d").to_string());
            if ui.button("►").clicked() {
                let tomorrow = self.selected_date.succ_opt().unwrap_or(self.selected_date);
                if tomorrow <= chrono::Local::now().date_naive() {
                    self.selected_date = tomorrow;
                }
            }
        });

        // Calculate nutrition data for the selected date
        let selected_date_str = self.selected_date.format("%Y-%m-%d").to_string();
        let (total_calories, calories_goal, calories_remaining) = self.calculate_daily_nutrition(db, &db.current_user, &selected_date_str);

        // Display nutrition summary
        ui.separator();
        ui.heading("Nutrition Summary");
        ui.label(format!("Calories Consumed: {:.1}", total_calories));
        ui.label(format!("Daily Goal: {:.1}", calories_goal));
        
        if calories_remaining >= 0.0 {
            ui.label(egui::RichText::new(format!("Remaining: {:.1}", calories_remaining))
                .color(egui::Color32::GREEN));
        } else {
            ui.label(egui::RichText::new(format!("Over by: {:.1}", calories_remaining.abs()))
                .color(egui::Color32::RED));
        }

        // Display food log entries
        ui.separator();
        ui.heading("Food Entries");
        
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
            ui.label("No entries for this date.");
        } else {
            for (i, entry) in entries.iter().enumerate() {
                let food_name = db.basic_foods.get(&entry.food_id)
                    .map(|f| f.name.clone())
                    .or_else(|| db.composite_foods.get(&entry.food_id).map(|f| f.name.clone()))
                    .unwrap_or_else(|| entry.food_id.clone());

                let calories = db.get_food_calories(&entry.food_id).unwrap_or(0.0) * entry.servings;
                
                ui.horizontal(|ui| {
                    ui.label(format!("{}. {} ({} servings) - {:.1} kcal", 
                        i + 1, 
                        food_name,
                        entry.servings,
                        calories
                    ));
                    
                    if ui.button("❌").clicked() {
                        if let Some(entries) = db.food_logs.get_mut(&db.current_user) {
                            if let Some(pos) = entries.iter().position(|e| 
                                e.date == entry.date && 
                                e.food_id == entry.food_id && 
                                e.servings == entry.servings
                            ) {
                                entries.remove(pos);
                            }
                        }
                    }
                });
            }
        }

        // Navigation button
        ui.separator();
        if ui.button("Back to Home").clicked() {
            *current_state = AppState::Home;
        }
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
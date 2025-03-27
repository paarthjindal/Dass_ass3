use eframe::egui;
use crate::models::Database;
use crate::app_state::AppState;

pub struct EditFoodLogScreen {
    selected_date: String,
}

impl EditFoodLogScreen {
    pub fn new() -> Self {
        Self {
            selected_date: chrono::Local::now().format("%Y-%m-%d").to_string(),
        }
    }

    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        db: &mut Database,
        current_state: &mut AppState,
    ) {
        ui.heading("Edit Food Log");

        // Date selection
        ui.label("Select Date:");
        ui.text_edit_singleline(&mut self.selected_date);

        // Display food entries for the selected date
        if let Some(entries) = db.food_logs.get_mut(&db.current_user) { // Use get_mut here
            let mut to_remove = Vec::new();
            for (index, entry) in entries.iter_mut().enumerate() {
                if entry.date == self.selected_date {
                    ui.horizontal(|ui| {
                        ui.label(&entry.food_id);
                        ui.add(egui::Slider::new(&mut entry.servings, 0.1..=10.0).text("Servings"));
                        if ui.button("Delete").clicked() {
                            to_remove.push(index);
                        }
                    });
                }
            }
            // Remove deleted entries
            for index in to_remove.into_iter().rev() {
                entries.remove(index);
            }
        }

        // Back button
        if ui.button("Back").clicked() {
            *current_state = AppState::Home;
        }
    }
}
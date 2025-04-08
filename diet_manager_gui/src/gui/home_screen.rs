use eframe::egui;
use crate::models::Database;
use crate::app_state::AppState;
use crate::gui::undo_manager::UndoManager;
use crate::gui::styling;

pub struct HomeScreen {
    show_notification: Option<(String, std::time::Instant)>,
}

impl Default for HomeScreen {
    fn default() -> Self {
        Self {
            show_notification: None,
        }
    }
}

impl HomeScreen {
    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        db: &mut Database,
        current_state: &mut AppState,
        undo_manager: &mut UndoManager
    ) {
        // Header with welcome message
        ui.vertical_centered(|ui| {
            ui.heading(egui::RichText::new("Diet Manager Dashboard").size(28.0).strong());
            ui.add_space(8.0);

            if let Some(user) = db.users.values().find(|u| u.user_id == db.current_user) {
                ui.label(
                    egui::RichText::new(format!("Welcome, {}!", user.username))
                        .size(20.0)
                        .color(styling::AppTheme::default().accent_color)
                );
            }
            ui.add_space(16.0);
        });

        // Calculate and display calorie information
        if !db.current_user.is_empty() {
            let date = chrono::Local::now().format("%Y-%m-%d").to_string();
            let (total_calories, target_calories, difference) = self.calculate_user_calories(
                db,
                &db.current_user,
                &date
            );

            // Nutrition summary card
            styling::card_frame().show(ui, |ui| {
                styling::section_header(ui, "Today's Nutrition Summary");
                ui.add_space(8.0);

                ui.columns(3, |cols| {
                    // Consumed calories
                    cols[0].vertical_centered(|ui| {
                        ui.label(egui::RichText::new("Consumed").size(14.0));
                        ui.label(
                            egui::RichText::new(format!("{:.0} kcal", total_calories))
                                .size(22.0)
                                .color(styling::AppTheme::default().text_color)
                        );
                    });

                    // Target calories
                    cols[1].vertical_centered(|ui| {
                        ui.label(egui::RichText::new("Target").size(14.0));
                        ui.label(
                            egui::RichText::new(format!("{:.0} kcal", target_calories))
                                .size(22.0)
                                .color(styling::AppTheme::default().text_color)
                        );
                    });

                    // Remaining calories
                    cols[2].vertical_centered(|ui| {
                        ui.label(
                            egui::RichText::new(if difference >= 0.0 { "Remaining" } else { "Exceeded" })
                                .size(14.0)
                        );
                        let color = if difference >= 0.0 {
                            styling::AppTheme::default().success_color
                        } else {
                            styling::AppTheme::default().error_color
                        };
                        ui.label(
                            egui::RichText::new(format!("{:.0} kcal", difference.abs()))
                                .size(22.0)
                                .color(color)
                        );
                    });
                });

                ui.add_space(10.0);

                // Progress bar
                if target_calories > 0.0 {
                    let progress = total_calories / target_calories;
                    let progress_text = format!("{:.1}% of daily goal", progress * 100.0);
                    let progress_bar = egui::ProgressBar::new(progress.clamp(0.0, 1.0))
                        .text(progress_text)
                        .animate(true);
                    ui.add(progress_bar);
                }
            });

            ui.add_space(16.0);
        }

        // Menu buttons in a grid layout
        styling::section_header(ui, "Quick Actions");
        ui.add_space(8.0);

        egui::Grid::new("home_buttons")
            .num_columns(3)
            .spacing([16.0, 16.0])
            .show(ui, |ui| {
                self.menu_button(ui, "Add Basic Food", "🥗", || {
                    *current_state = AppState::AddBasicFood;
                });
                self.menu_button(ui, "Add Composite Food", "🍲", || {
                    *current_state = AppState::AddCompositeFood;
                });
                self.menu_button(ui, "View Daily Log", "📊", || {
                    *current_state = AppState::ViewDailyLog;
                });
                ui.end_row();

                self.menu_button(ui, "Add Food to Log", "➕", || {
                    *current_state = AppState::AddFoodToLog;
                });
                self.menu_button(ui, "Edit Food Log", "✏️", || {
                    *current_state = AppState::EditFoodLog;
                });
                self.menu_button(ui, "Update Profile", "👤", || {
                    *current_state = AppState::UpdateProfile;
                });
                ui.end_row();

                self.menu_button(ui, "Download Food Data", "🌐", || {
                    *current_state = AppState::DownloadFoodData;
                });
                ui.end_row();
            });

        ui.add_space(16.0);

        // Bottom buttons for logout and undo
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if styling::warning_button(ui, "Logout").clicked() {
                    db.current_user.clear();
                    *current_state = AppState::Login;
                }

                let undo_label = if let Some(desc) = undo_manager.last_action_description() {
                    format!("Undo: {}", desc)
                } else {
                    "Undo".to_string()
                };

                let undo_button = styling::primary_button(ui, &undo_label);
                if undo_button.clicked() && undo_manager.can_undo() {
                    println!("[DEBUG] Undo button clicked - current state before undo:");
                    undo_manager.debug_print_state();

                    if let Some((previous_db, action)) = undo_manager.undo() {
                        println!("[DEBUG] Applying undo for: {}", action);
                        *db = previous_db;

                        if let Err(e) = crate::database::save_database(db) {
                            eprintln!("Failed to save database after undo: {}", e);
                        }

                        self.show_notification = Some((format!("Undid: {}", action), std::time::Instant::now()));
                    }
                }

                if !undo_manager.can_undo() {
                    undo_button.on_disabled_hover_text("Nothing to undo");
                }

                if let Some((message, time)) = &self.show_notification {
                    if time.elapsed() < std::time::Duration::from_secs(3) {
                        ui.add_space(10.0);
                        ui.colored_label(
                            styling::AppTheme::default().success_color,
                            format!("✓ {}", message)
                        );
                    } else {
                        self.show_notification = None;
                    }
                }
            });
        });
    }

    fn menu_button(
        &self,
        ui: &mut egui::Ui,
        title: &str,
        icon: &str,
        on_click: impl FnOnce()
    ) -> egui::Response {
        let theme = styling::AppTheme::default();
        let button_height = 90.0;

        let button = ui.add_sized(
            [120.0, button_height],
            egui::Button::new(egui::RichText::new(format!("{}\n{}", icon, title)).size(18.0).strong())
                .fill(theme.primary_color.gamma_multiply(0.7))
        );

        if button.clicked() {
            on_click();
        }

        button
    }

    fn calculate_user_calories(&self, db: &Database, user_id: &str, date: &str) -> (f32, f32, f32) {
        let total_calories = db.food_logs.get(user_id).map_or(0.0, |entries| {
            entries
                .iter()
                .filter(|entry| entry.date == date)
                .map(|entry| db.get_food_calories(&entry.food_id).unwrap_or(0.0) * entry.servings)
                .sum()
        });

        let target_calories = db.users
            .values()
            .find(|u| u.user_id == user_id)
            .map(|user| user.profile.calculate_target_calories())
            .unwrap_or(0.0);

        let difference = target_calories - total_calories;
        (total_calories, target_calories, difference)
    }
}
use eframe::egui;
use crate::models::Database;
use crate::database::{ load_database, save_database };
use crate::gui::{
    LoginScreen,
    RegisterScreen,
    HomeScreen,
    AddBasicFoodScreen,
    AddCompositeFoodScreen,
    ViewDailyLogScreen,
    AddFoodToLogScreen,
    EditFoodLogScreen,
    UpdateProfileScreen,
};
use crate::app_state::AppState;
use crate::gui::undo_manager::UndoManager;
use crate::gui::styling;
mod models;
mod database;
mod app_state;
mod gui;

struct DietManagerApp {
    db: Database,
    current_state: AppState,
    login_screen: LoginScreen,
    register_screen: RegisterScreen,
    home_screen: HomeScreen,
    add_basic_food_screen: AddBasicFoodScreen,
    add_composite_food_screen: AddCompositeFoodScreen,
    view_daily_log_screen: ViewDailyLogScreen,
    add_food_to_log_screen: AddFoodToLogScreen,
    edit_food_log_screen: EditFoodLogScreen,
    update_profile_screen: UpdateProfileScreen,
    undo_manager: UndoManager,
}

impl Default for DietManagerApp {
    fn default() -> Self {
        Self {
            db: load_database(),
            current_state: AppState::Login,
            login_screen: LoginScreen::new(),
            register_screen: RegisterScreen::new(),
            home_screen: HomeScreen,
            add_basic_food_screen: AddBasicFoodScreen::new(),
            add_composite_food_screen: AddCompositeFoodScreen::new(),
            view_daily_log_screen: ViewDailyLogScreen::new(),
            add_food_to_log_screen: AddFoodToLogScreen::new(),
            edit_food_log_screen: EditFoodLogScreen::new(),
            update_profile_screen: UpdateProfileScreen::new(),
            undo_manager: UndoManager::new(100),
        }
    }
}

impl eframe::App for DietManagerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        styling::apply_theme(ctx);
        // Add a top bar with app title and user info
      // Add a top bar with app title and user info
      egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.heading("Diet Manager");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if !self.db.current_user.is_empty() {
                    // Find username directly by looking up the user_id
                    if let Some(user) = self.db.users.values().find(|u| u.user_id == self.db.current_user) {
                        ui.label(format!("Logged in as: {}", user.username));
                    }
                }
            });
        });
        ui.add_space(8.0);
    });

        egui::CentralPanel::default().show(ctx, |ui| {
            styling::card_frame().show(ui, |ui| {
                match self.current_state {
                    AppState::Login =>
                        self.login_screen.render(ui, &mut self.db, &mut self.current_state),
                    AppState::Register =>
                        self.register_screen.render(ui, &mut self.db, &mut self.current_state),
                    AppState::Home =>
                        self.home_screen.render(
                            ui,
                            &mut self.db,
                            &mut self.current_state,
                            &mut self.undo_manager
                        ),
                    AppState::AddBasicFood =>
                        self.add_basic_food_screen.render(
                            ui,
                            &mut self.db,
                            &mut self.current_state
                        ),
                    AppState::AddCompositeFood =>
                        self.add_composite_food_screen.render(
                            ui,
                            &mut self.db,
                            &mut self.current_state
                        ),
                    AppState::ViewDailyLog =>
                        self.view_daily_log_screen.render(
                            ui,
                            &mut self.db,
                            &mut self.current_state
                        ),
                    AppState::AddFoodToLog =>
                        self.add_food_to_log_screen.render(
                            ui,
                            &mut self.db,
                            &mut self.current_state
                        ),
                    AppState::EditFoodLog =>
                        self.edit_food_log_screen.render(ui, &mut self.db, &mut self.current_state),
                    AppState::UpdateProfile =>
                        self.update_profile_screen.render(
                            ui,
                            &mut self.db,
                            &mut self.current_state
                        ),
                }
            });
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        if let Err(e) = save_database(&self.db) {
            eprintln!("Failed to save database: {}", e);
        }
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Diet Manager",
        options,
        Box::new(|_cc| Box::new(DietManagerApp::default()))
    );
}

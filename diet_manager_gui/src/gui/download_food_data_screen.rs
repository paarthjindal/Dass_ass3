use eframe::egui;
use crate::models::Database;
use crate::app_state::AppState;
use crate::gui::styling;

#[derive(Debug, PartialEq, Clone)]
enum FoodDataSource {
    McDonalds,
    USDA,
    MyFitnessPal,
    Custom,
}

impl std::fmt::Display for FoodDataSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FoodDataSource::McDonalds => write!(f, "McDonald's"),
            FoodDataSource::USDA => write!(f, "USDA"),
            FoodDataSource::MyFitnessPal => write!(f, "MyFitnessPal"),
            FoodDataSource::Custom => write!(f, "Custom"),
        }
    }
}

pub struct DownloadFoodDataScreen {
    url_input: String,
    selected_source: FoodDataSource,
    status_message: Option<String>,
}

impl Default for DownloadFoodDataScreen {
    fn default() -> Self {
        Self {
            url_input: String::new(),
            selected_source: FoodDataSource::McDonalds,
            status_message: None,
        }
    }
}

impl DownloadFoodDataScreen {
    pub fn render(&mut self, ui: &mut egui::Ui, db: &mut Database, current_state: &mut AppState) {
        ui.vertical_centered(|ui| {
            ui.heading(egui::RichText::new("Download Food Data").size(28.0).strong());
            ui.add_space(16.0);
        });

        styling::card_frame().show(ui, |ui| {
            styling::section_header(ui, "Select Data Source");
            
            ui.horizontal(|ui| {
                ui.radio_value(&mut self.selected_source, FoodDataSource::McDonalds, "McDonald's");
                ui.radio_value(&mut self.selected_source, FoodDataSource::USDA, "USDA");
                ui.radio_value(&mut self.selected_source, FoodDataSource::MyFitnessPal, "MyFitnessPal");
                ui.radio_value(&mut self.selected_source, FoodDataSource::Custom, "Custom");
            });

            ui.add_space(16.0);

            match self.selected_source {
                FoodDataSource::Custom => {
                    ui.label("Enter food data URL:");
                    ui.text_edit_singleline(&mut self.url_input);
                }
                _ => {
                    ui.label(format!("Will use standard {} API", self.selected_source));
                }
            }

            ui.add_space(16.0);

            if styling::primary_button(ui, "Download").clicked() {
                self.status_message = Some(self.download_food_data(db));
            }

            if let Some(msg) = &self.status_message {
                ui.add_space(8.0);
                ui.label(egui::RichText::new(msg).color(styling::AppTheme::default().text_color));
            }
        });

        ui.add_space(16.0);

        if styling::warning_button(ui, "Back to Home").clicked() {
            *current_state = AppState::Home;
        }
    }

    fn download_food_data(&mut self, _db: &mut Database) -> String {
        // TODO: Implement actual download logic for each source
        match self.selected_source {
            FoodDataSource::McDonalds => {
                "Downloading from McDonald's... (Not implemented yet)".to_string()
            }
            FoodDataSource::USDA => {
                "Downloading from USDA database... (Not implemented yet)".to_string()
            }
            FoodDataSource::MyFitnessPal => {
                "Downloading from MyFitnessPal... (Not implemented yet)".to_string()
            }
            FoodDataSource::Custom => {
                if self.url_input.is_empty() {
                    return "Please enter a URL".to_string();
                }
                format!("Downloading from custom URL: {} (Not implemented yet)", self.url_input)
            }
        }
    }
}
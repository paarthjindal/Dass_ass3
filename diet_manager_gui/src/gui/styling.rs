use eframe::egui::{self, Color32, Rounding, Stroke, Vec2};

pub struct AppTheme {
    pub primary_color: Color32,
    pub secondary_color: Color32,
    pub accent_color: Color32,
    pub text_color: Color32,
    pub bg_color: Color32,
    pub success_color: Color32,
    pub warning_color: Color32,
    pub error_color: Color32,
    pub card_rounding: Rounding,
    pub button_rounding: Rounding,
}

impl Default for AppTheme {
    fn default() -> Self {
        Self {
            // More vibrant colors with better contrast
            primary_color: Color32::from_rgb(25, 118, 210),    // Brighter blue
            secondary_color: Color32::from_rgb(46, 125, 50),   // Stronger green
            accent_color: Color32::from_rgb(255, 193, 7),      // Brighter yellow
            text_color: Color32::from_rgb(245, 245, 245),      // Near white for text
            bg_color: Color32::from_rgb(33, 33, 33),           // Dark background
            success_color: Color32::from_rgb(76, 175, 80),     // Brighter green
            warning_color: Color32::from_rgb(255, 152, 0),     // Bright orange
            error_color: Color32::from_rgb(244, 67, 54),       // Bright red
            card_rounding: Rounding::same(8.0),
            button_rounding: Rounding::same(4.0),
        }
    }
}

pub fn apply_theme(ctx: &egui::Context) {
    let theme = AppTheme::default();

    let mut style = (*ctx.style()).clone();

    // Set global colors with better contrast
    style.visuals.override_text_color = Some(theme.text_color);
    style.visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, theme.text_color);
    style.visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, theme.text_color);
    style.visuals.widgets.hovered.fg_stroke = Stroke::new(1.5, theme.text_color);
    style.visuals.widgets.active.fg_stroke = Stroke::new(2.0, theme.text_color);

    // Set background colors
    style.visuals.extreme_bg_color = theme.bg_color;
    style.visuals.widgets.noninteractive.bg_fill = theme.bg_color.gamma_multiply(1.2); // Slightly lighter

    // Button colors - much more visible
    style.visuals.widgets.inactive.bg_fill = theme.primary_color;
    style.visuals.widgets.hovered.bg_fill = theme.primary_color.gamma_multiply(1.3); // Brighter on hover
    style.visuals.widgets.active.bg_fill = theme.primary_color.gamma_multiply(0.9); // Slightly darker when clicked

    // Set spacing
    style.spacing.item_spacing = Vec2::new(10.0, 10.0);
    style.spacing.window_margin = egui::Margin::same(16.0);

    // Set rounding
    style.visuals.widgets.noninteractive.rounding = theme.card_rounding;
    style.visuals.widgets.inactive.rounding = theme.button_rounding;
    style.visuals.widgets.hovered.rounding = theme.button_rounding;
    style.visuals.widgets.active.rounding = theme.button_rounding;

    // Increase default text size for better readability
    style.text_styles.get_mut(&egui::TextStyle::Body).unwrap().size = 16.0;
    style.text_styles.get_mut(&egui::TextStyle::Button).unwrap().size = 16.0;
    style.text_styles.get_mut(&egui::TextStyle::Heading).unwrap().size = 24.0;

    ctx.set_style(style);
}

pub fn heading_style(ui: &mut egui::Ui) -> egui::TextStyle {
    let heading = egui::TextStyle::Heading;
    heading
}

pub fn styled_button(ui: &mut egui::Ui, text: &str) -> egui::Response {
    ui.add_sized([140.0, 36.0], egui::Button::new(egui::RichText::new(text).size(16.0)))
}

pub fn primary_button(ui: &mut egui::Ui, text: &str) -> egui::Response {
    let theme = AppTheme::default();
    let button = egui::Button::new(egui::RichText::new(text).size(16.0))
        .fill(theme.primary_color)
        .stroke(Stroke::new(1.0, Color32::WHITE));

    ui.add_sized([140.0, 36.0], button)
}

pub fn success_button(ui: &mut egui::Ui, text: &str) -> egui::Response {
    let theme = AppTheme::default();
    let button = egui::Button::new(egui::RichText::new(text).size(16.0))
        .fill(theme.success_color)
        .stroke(Stroke::new(1.0, Color32::WHITE));

    ui.add_sized([140.0, 36.0], button)
}

pub fn warning_button(ui: &mut egui::Ui, text: &str) -> egui::Response {
    let theme = AppTheme::default();
    let button = egui::Button::new(egui::RichText::new(text).size(16.0))
        .fill(theme.warning_color)
        .stroke(Stroke::new(1.0, Color32::WHITE));

    ui.add_sized([140.0, 36.0], button)
}

pub fn error_button(ui: &mut egui::Ui, text: &str) -> egui::Response {
    let theme = AppTheme::default();
    let button = egui::Button::new(egui::RichText::new(text).size(16.0))
        .fill(theme.error_color)
        .stroke(Stroke::new(1.0, Color32::WHITE));

    ui.add_sized([140.0, 36.0], button)
}

pub fn card_frame() -> egui::Frame {
    let theme = AppTheme::default();
    egui::Frame::none()
        .fill(Color32::from_rgb(50, 50, 50)) // Darker card background
        .stroke(Stroke::new(1.0, Color32::from_gray(100)))
        .rounding(theme.card_rounding)
        .inner_margin(egui::Margin::same(16.0))
        .outer_margin(egui::Margin::same(8.0))
}

pub fn highlight_text(text: &str) -> egui::RichText {
    let theme = AppTheme::default();
    egui::RichText::new(text)
        .color(theme.accent_color)
        .size(18.0)
        .strong()
}

pub fn label_style(ui: &mut egui::Ui, text: &str) {
    ui.label(egui::RichText::new(text).size(16.0));
}

pub fn section_header(ui: &mut egui::Ui, text: &str) {
    ui.add_space(8.0);
    ui.heading(egui::RichText::new(text).size(20.0).strong());
    ui.add_space(4.0);
}
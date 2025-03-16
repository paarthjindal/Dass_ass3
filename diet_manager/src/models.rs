use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::any::Any;

// Food trait that must be implemented by all food types
pub trait FoodTrait: Send + Sync {
    fn name(&self) -> &str;
    fn keywords(&self) -> &Vec<String>;
    fn calories_per_serving(&self) -> f32;
    fn as_any(&self) -> &dyn Any;
}

// Basic food model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicFood {
    pub name: String, // ID and name are the same for simplicity
    pub keywords: Vec<String>,
    pub calories_per_serving: f32,
}

// Composite food model consisting of other foods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositeFood {
    pub name: String, // ID and name are the same for simplicity
    pub keywords: Vec<String>,
    pub components: Vec<FoodComponent>,
}

// Component of a composite food
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodComponent {
    pub food_name: String, // Using name as identifier
    pub servings: f32,
}

// Food entry in daily log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub food_name: String, // Using name as identifier
    pub servings: f32,
    pub timestamp: Option<chrono::DateTime<Utc>>,
}

impl LogEntry {
    pub fn new(food_name: String, servings: f32) -> Self {
        LogEntry {
            food_name,
            servings,
            timestamp: Some(Utc::now()),
        }
    }
}

// Daily log for a specific date
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyLog {
    pub date: NaiveDate,
    pub food_entries: Vec<LogEntry>, // Renamed to match CLI usage
    pub weight: Option<f32>,
    pub activity_level: Option<ActivityLevel>,
}

// User gender
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Gender {
    Male,
    Female,
}

// Activity level for calorie calculations
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ActivityLevel {
    Sedentary,
    Light, // Match CLI usage
    Moderate, // Match CLI usage
    VeryActive,
    ExtraActive, // Match CLI usage
}

// User profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub gender: Gender,
    pub height_cm: f32,
    pub age: u32, // Changed to u32 to match CLI
    pub weight_kg: f32,
    pub activity_level: ActivityLevel,
    pub calorie_method: CalorieCalculationMethod, // Match CLI usage
}

// Available methods for calculating daily calorie needs
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum CalorieCalculationMethod {
    HarrisBenedict,
    MifflinStJeor,
}

// Command type for undo functionality
pub type Command = Box<dyn FnOnce(&mut crate::log::daily::DailyLogManager) -> Result<(), String> + Send>;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BasicFood {
    pub id: String,
    pub name: String,
    pub keywords: Vec<String>,
    pub calories_per_serving: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompositeFood {
    pub id: String,
    pub name: String,
    pub keywords: Vec<String>,
    pub components: Vec<FoodComponent>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FoodComponent {
    pub food_id: String,
    pub servings: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FoodLogEntry {
    pub date: String, // ISO 8601 date format (e.g., "2023-10-01")
    pub food_id: String,
    pub servings: f32,
    pub user_id: String, // Add user_id to associate with a user
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ActivityLevel {
    Sedentary,
    Light,
    Moderate,
    VeryActive,
    ExtraActive,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CalorieCalculationMethod {
    HarrisBenedict,
    MifflinStJeor,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserProfile {
    pub gender: Gender,
    pub height_cm: f32,
    pub age: u32,
    pub calorie_method: CalorieCalculationMethod,
    pub weight_kg: f32,
    pub activity_level: ActivityLevel,
}

impl UserProfile {
    pub fn calculate_target_calories(&self) -> f32 {
        let bmr = match self.gender {
            Gender::Male => 88.362 + (13.397 * self.weight_kg) + (4.799 * self.height_cm) - (5.677 * self.age as f32),
            Gender::Female => 447.593 + (9.247 * self.weight_kg) + (3.098 * self.height_cm) - (4.330 * self.age as f32),
        };
        bmr * match self.activity_level {
            ActivityLevel::Sedentary => 1.2,
            ActivityLevel::Light => 1.375,
            ActivityLevel::Moderate => 1.55,
            ActivityLevel::VeryActive => 1.725,
            ActivityLevel::ExtraActive => 1.9,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub user_id: String,
    pub username: String,
    pub password: String, // In a real application, this should be hashed
    pub profile: UserProfile,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Database {
    pub users: HashMap<String, User>, // Key: username, Value: User
    pub basic_foods: HashMap<String, BasicFood>,
    pub composite_foods: HashMap<String, CompositeFood>,
    pub food_logs: HashMap<String, Vec<FoodLogEntry>>, // Key: username, Value: logs
    pub current_user: String, // Track the currently logged-in user
}

impl Database {
    pub fn get_food_calories(&self, food_id: &str) -> Option<f32> {
        if let Some(basic_food) = self.basic_foods.get(food_id) {
            Some(basic_food.calories_per_serving)
        } else if let Some(composite_food) = self.composite_foods.get(food_id) {
            let mut total_calories = 0.0;
            for component in &composite_food.components {
                if let Some(calories) = self.get_food_calories(&component.food_id) {
                    total_calories += calories * component.servings;
                }
            }
            Some(total_calories)
        } else {
            None
        }
    }

    pub fn calculate_calories(&self, username: &str, date: &str) -> (f32, f32, f32) {
        let mut total_calories = 0.0;
        if let Some(entries) = self.food_logs.get(username) {
            for entry in entries {
                if entry.date == date {
                    if let Some(calories) = self.get_food_calories(&entry.food_id) {
                        total_calories += calories * entry.servings;
                    }
                }
            }
        }

        let target_calories = if let Some(user) = self.users.get(username) {
            user.profile.calculate_target_calories()
        } else {
            0.0
        };

        let difference = target_calories - total_calories;
        (total_calories, target_calories, difference)
    }
}

// ...existing code...

// ...existing code...

impl Default for Database {
    fn default() -> Self {
        let mut basic_foods = std::collections::HashMap::new();
        let mut composite_foods = std::collections::HashMap::new();

        // Add hardcoded basic foods
        basic_foods.insert("apple".to_string(), BasicFood {
            id: "apple".to_string(),
            name: "Apple".to_string(),
            keywords: vec!["fruit".to_string(), "fresh".to_string(), "snack".to_string()],
            calories_per_serving: 95.0,
        });

        basic_foods.insert("banana".to_string(), BasicFood {
            id: "banana".to_string(),
            name: "Banana".to_string(),
            keywords: vec!["fruit".to_string(), "fresh".to_string(), "potassium".to_string()],
            calories_per_serving: 105.0,
        });

        basic_foods.insert("chicken_breast".to_string(), BasicFood {
            id: "chicken_breast".to_string(),
            name: "Chicken Breast".to_string(),
            keywords: vec!["meat".to_string(), "protein".to_string(), "lean".to_string()],
            calories_per_serving: 165.0,
        });

        basic_foods.insert("brown_rice".to_string(), BasicFood {
            id: "brown_rice".to_string(),
            name: "Brown Rice".to_string(),
            keywords: vec!["grain".to_string(), "carbs".to_string(), "whole grain".to_string()],
            calories_per_serving: 215.0,
        });

        basic_foods.insert("egg".to_string(), BasicFood {
            id: "egg".to_string(),
            name: "Egg".to_string(),
            keywords: vec!["protein".to_string(), "breakfast".to_string()],
            calories_per_serving: 78.0,
        });

        // Add bread as well for the sandwich
        basic_foods.insert("bread".to_string(), BasicFood {
            id: "bread".to_string(),
            name: "Bread Slice".to_string(),
            keywords: vec!["grain".to_string(), "carbs".to_string()],
            calories_per_serving: 80.0,
        });

        // Add a hardcoded composite food - using Vec<FoodComponent> instead of HashMap
        let sandwich_components = vec![
            FoodComponent {
                food_id: "chicken_breast".to_string(),
                servings: 0.5
            },
            FoodComponent {
                food_id: "bread".to_string(),
                servings: 2.0
            },
        ];

        composite_foods.insert("sandwich".to_string(), CompositeFood {
            id: "sandwich".to_string(),
            name: "Basic Sandwich".to_string(),
            keywords: vec!["lunch".to_string(), "quick".to_string(), "easy".to_string()],
            components: sandwich_components,
        });

        // Add another hardcoded composite food
        let oatmeal_components = vec![
            FoodComponent {
                food_id: "banana".to_string(),
                servings: 1.0
            },
        ];

        composite_foods.insert("banana_oatmeal".to_string(), CompositeFood {
            id: "banana_oatmeal".to_string(),
            name: "Banana Oatmeal".to_string(),
            keywords: vec!["breakfast".to_string(), "healthy".to_string()],
            components: oatmeal_components,
        });

        Self {
            users: std::collections::HashMap::new(),
            basic_foods,
            composite_foods,
            food_logs: std::collections::HashMap::new(),
            current_user: String::new(),
        }
    }
}
use crate::models::{UserProfile, Gender, ActivityLevel, CalorieCalculationMethod};

impl UserProfile {
    pub fn new(
        gender: Gender,
        height_cm: f32,
        age: u32,
        weight_kg: f32,
        activity_level: ActivityLevel,
        calorie_method: CalorieCalculationMethod,
    ) -> Self {
        UserProfile {
            gender,
            height_cm,
            age,
            weight_kg,
            activity_level,
            calorie_method, // Changed to match the struct field
        }
    }

    pub fn update_weight(&mut self, weight_kg: f32) {
        self.weight_kg = weight_kg;
    }

    pub fn update_age(&mut self, age: u32) {
        self.age = age;
    }

    pub fn update_activity_level(&mut self, level: ActivityLevel) {
        self.activity_level = level;
    }

    pub fn update_calculation_method(&mut self, method: CalorieCalculationMethod) {
        self.calorie_method = method;
    }
}
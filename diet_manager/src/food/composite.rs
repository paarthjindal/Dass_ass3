use crate::models::{CompositeFood, FoodComponent, FoodTrait};
use std::any::Any;

impl CompositeFood {
    pub fn new(name: String, keywords: Vec<String>, components: Vec<FoodComponent>) -> Self {
        CompositeFood {
            name,
            keywords,
            components,
        }
    }

    // This is for the estimated calculation without database access
    pub fn calculate_calories_per_serving(&self, foods: &Vec<Box<dyn FoodTrait>>) -> f32 {
        self.components
            .iter()
            .map(|component| {
                let calories_per_serving = foods
                    .iter()
                    .find(|food| food.name() == component.food_name)
                    .map_or(0.0, |food| food.calories_per_serving());

                component.servings * calories_per_serving
            })
            .sum()
    }

    pub fn matches_keywords(&self, keywords: &[String], match_all: bool) -> bool {
        if keywords.is_empty() {
            return true;
        }

        if match_all {
            keywords.iter().all(|keyword| {
                let keyword_lower = keyword.to_lowercase();
                self.keywords.iter().any(|k| k.to_lowercase().contains(&keyword_lower)) ||
                    self.name.to_lowercase().contains(&keyword_lower)
            })
        } else {
            keywords.iter().any(|keyword| {
                let keyword_lower = keyword.to_lowercase();
                self.keywords.iter().any(|k| k.to_lowercase().contains(&keyword_lower)) ||
                    self.name.to_lowercase().contains(&keyword_lower)
            })
        }
    }
}

// Implement FoodTrait for CompositeFood
impl FoodTrait for CompositeFood {
    fn name(&self) -> &str {
        &self.name
    }

    fn keywords(&self) -> &Vec<String> {
        &self.keywords
    }

    fn calories_per_serving(&self) -> f32 {
        // This is a placeholder, the actual calculation is done in calculate_calories_per_serving
        // which requires access to the database
        0.0
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
use crate::models::{BasicFood, FoodTrait};
use std::any::Any;

impl BasicFood {
    pub fn new(name: String, keywords: Vec<String>, calories: f32) -> Self {
        BasicFood {
            name,
            keywords,
            calories_per_serving: calories,
        }
    }

    pub fn matches_keywords(&self, keywords: &[String], match_all: bool) -> bool {
        if keywords.is_empty() {
            return true;
        }

        if match_all {
            keywords.iter().all(|keyword| {
                let keyword_lower = keyword.to_lowercase();
                self.keywords.iter().any(|k| k.to_lowercase().contains(&keyword_lower))
                    || self.name.to_lowercase().contains(&keyword_lower)
            })
        } else {
            keywords.iter().any(|keyword| {
                let keyword_lower = keyword.to_lowercase();
                self.keywords.iter().any(|k| k.to_lowercase().contains(&keyword_lower))
                    || self.name.to_lowercase().contains(&keyword_lower)
            })
        }
    }
}

// Implement FoodTrait for BasicFood
impl FoodTrait for BasicFood {
    fn name(&self) -> &str {
        &self.name
    }

    fn keywords(&self) -> &Vec<String> {
        &self.keywords
    }

    fn calories_per_serving(&self) -> f32 {
        self.calories_per_serving
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
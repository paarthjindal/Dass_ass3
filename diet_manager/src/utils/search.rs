use crate::models::FoodTrait;
use std::boxed::Box;

pub fn search_foods<'a>(
    foods: &'a Vec<Box<dyn FoodTrait>>,
    keywords: &[&str],
    match_all: bool,
) -> Vec<&'a Box<dyn FoodTrait>> {
    // If no keywords, return all foods
    if keywords.is_empty() {
        return foods.iter().collect();
    }

    foods
        .iter()
        .filter(|food| {
            let food_keywords: Vec<&str> = food
                .keywords()
                .iter()
                .map(|k| k.as_str())
                .collect();

            if match_all {
                // All keywords must match
                keywords.iter().all(|k| {
                    food.name().to_lowercase().contains(&k.to_lowercase())
                        || food_keywords.iter().any(|fk| fk.to_lowercase().contains(&k.to_lowercase()))
                })
            } else {
                // Any keyword can match
                keywords.iter().any(|k| {
                    food.name().to_lowercase().contains(&k.to_lowercase())
                        || food_keywords.iter().any(|fk| fk.to_lowercase().contains(&k.to_lowercase()))
                })
            }
        })
        .collect()
}
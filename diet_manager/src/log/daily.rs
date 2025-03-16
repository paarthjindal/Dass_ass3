use std::collections::HashMap;
use chrono::{NaiveDate, Utc};
use crate::models::{DailyLog, LogEntry, ActivityLevel, FoodTrait};

pub struct DailyLogManager {
    logs: HashMap<String, DailyLog>,
    current_date: NaiveDate,
}

impl DailyLogManager {
    pub fn new(logs: HashMap<String, DailyLog>) -> Self {
        let current_date = Utc::now().date_naive();

        // Create a default log for today if it doesn't exist
        let mut logs = logs;
        if !logs.contains_key(&current_date.to_string()) {
            logs.insert(current_date.to_string(), DailyLog {
                date: current_date,
                food_entries: Vec::new(),
                weight: None,
                activity_level: None,
            });
        }

        DailyLogManager {
            logs,
            current_date,
        }
    }

    pub fn get_current_date(&self) -> NaiveDate {
        self.current_date
    }

    pub fn set_current_date(&mut self, date: NaiveDate) {
        self.current_date = date;
        self.ensure_log_exists();
    }

    pub fn get_current_log(&self) -> &DailyLog {
        let date_str = self.current_date.to_string();
        self.logs.get(&date_str).unwrap()
    }

    pub fn get_current_log_mut(&mut self) -> &mut DailyLog {
        let date_str = self.current_date.to_string();
        self.logs.get_mut(&date_str).unwrap()
    }

    pub fn update_current_log(&mut self, log: &DailyLog) {
        let date_str = self.current_date.to_string();
        self.logs.insert(date_str, log.clone());
    }

    pub fn add_food_entry(&mut self, food_name: &str, servings: f32) {
        self.ensure_log_exists();
        let date_str = self.current_date.to_string();

        let log = self.logs.get_mut(&date_str).unwrap();
        log.food_entries.push(LogEntry::new(food_name.to_string(), servings));
    }

    pub fn delete_food_entry(&mut self, index: usize) {
        let date_str = self.current_date.to_string();

        if let Some(log) = self.logs.get_mut(&date_str) {
            if index < log.food_entries.len() {
                log.food_entries.remove(index);
            }
        }
    }

    pub fn update_weight(&mut self, weight: f32) -> Option<f32> {
        self.ensure_log_exists();
        let log = self.get_current_log_mut();
        let old_weight = log.weight;
        log.weight = Some(weight);
        old_weight
    }

    pub fn update_activity_level(&mut self, activity: ActivityLevel) -> Option<ActivityLevel> {
        self.ensure_log_exists();
        let log = self.get_current_log_mut();
        let old_level = log.activity_level;
        log.activity_level = Some(activity);
        old_level
    }

    pub fn calculate_calories_consumed(&self, foods: &Vec<Box<dyn FoodTrait>>) -> f32 {
        let log = self.get_current_log();

        log.food_entries.iter()
            .map(|entry| {
                let food = foods.iter()
                    .find(|f| f.name() == entry.food_name);

                match food {
                    Some(f) => entry.servings * f.calories_per_serving(),
                    None => 0.0,
                }
            })
            .sum()
    }

    pub fn get_all_logs(&self) -> &HashMap<String, DailyLog> {
        &self.logs
    }

    fn ensure_log_exists(&mut self) {
        let date_str = self.current_date.to_string();

        if !self.logs.contains_key(&date_str) {
            self.logs.insert(date_str, DailyLog {
                date: self.current_date,
                food_entries: Vec::new(),
                weight: None,
                activity_level: None,
            });
        }
    }
}
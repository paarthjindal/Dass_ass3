use std::io::{ self, Write };
use chrono::{ NaiveDate, Utc };
use crate::models::FoodTrait;
use crate::models::{
    BasicFood,
    CompositeFood,
    FoodComponent,
    LogEntry,
    UserProfile,
    Gender,
    ActivityLevel,
    CalorieCalculationMethod,
};
use crate::database::DatabaseManager;
use crate::log::daily::DailyLogManager;
use crate::log::undo::UndoManager;
use crate::user::goals::{ calculate_daily_goal };
use crate::utils::search::search_foods;

pub struct CLI {
    db_manager: DatabaseManager,
    log_manager: DailyLogManager,
    undo_manager: UndoManager,
}

impl CLI {
    pub fn new(db_manager: DatabaseManager) -> Self {
        let log_manager = DailyLogManager::new(db_manager.logs.clone());
        let undo_manager = UndoManager::new(100); // Store up to 100 commands

        CLI {
            db_manager,
            log_manager,
            undo_manager,
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        if self.db_manager.profile.is_none() {
            println!("No user profile found. Let's set one up.");
            self.create_user_profile()?;
        }

        loop {
            println!("\n===== YADA (Yet Another Diet Assistant) =====");
            println!("Current date: {}", self.log_manager.get_current_date());

            let current_log = self.log_manager.get_current_log();
            let weight = current_log.weight.unwrap_or(0.0);
            let activity_level = current_log.activity_level.unwrap_or(ActivityLevel::Sedentary);
            let profile = self.db_manager.profile.as_ref().unwrap();

            let calories_goal = calculate_daily_goal(profile, weight, activity_level);
            let calories_consumed = self.log_manager.calculate_calories_consumed(
                &self.db_manager.food_db
            );
            let calories_remaining = calories_goal - calories_consumed;

            println!("Calories goal: {:.1} kcal", calories_goal);
            println!("Calories consumed: {:.1} kcal", calories_consumed);
            println!("Calories remaining: {:.1} kcal", calories_remaining);

            println!("\nMenu Options:");
            println!("1. Add food to log");
            println!("2. View today's log");
            println!("3. Delete food from log");
            println!("4. Change date");
            println!("5. Update weight/activity level");
            println!("6. Add new basic food");
            println!("7. Create composite food");
            println!("8. Search foods");
            println!("9. Update user profile");
            println!("10. Save data");
            println!("11. Undo last action");
            println!("0. Exit");

            print!("Enter your choice: ");
            io::stdout().flush().unwrap();

            let mut choice = String::new();
            io::stdin().read_line(&mut choice).expect("Failed to read line");

            match choice.trim().parse::<u32>() {
                Ok(1) => self.add_food_to_log()?,
                Ok(2) => self.view_daily_log(),
                Ok(3) => self.delete_food_from_log()?,
                Ok(4) => self.change_date()?,
                Ok(5) => self.update_weight_activity()?,
                Ok(6) => self.add_basic_food()?,
                Ok(7) => self.create_composite_food()?,
                Ok(8) => self.search_food_database()?,
                Ok(9) => self.update_user_profile()?,
                Ok(10) => self.save_data()?,
                Ok(11) => self.undo_last_action()?,
                Ok(0) => {
                    self.save_data()?;
                    println!("Exiting YADA. Have a healthy day!");
                    break;
                }
                _ => println!("Invalid choice. Please try again."),
            }
        }

        Ok(())
    }

    fn add_food_to_log(&mut self) -> Result<(), String> {
        println!("\n===== Add Food to Log =====");
        println!("1. Search by name");
        println!("2. Search by keywords");
        println!("3. View all foods");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        let food_list = match choice.trim().parse::<u32>() {
            Ok(1) => {
                print!("Enter food name to search: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read line");
                let name = name.trim();
                self.db_manager.search_foods_by_name(name)
            }
            Ok(2) => {
                println!("Enter keywords separated by space: ");
                let mut keywords = String::new();
                io::stdin().read_line(&mut keywords).expect("Failed to read line");
                let keywords: Vec<&str> = keywords.trim().split_whitespace().collect();

                println!("Match: 1. All keywords 2. Any keyword");
                let mut match_type = String::new();
                io::stdin().read_line(&mut match_type).expect("Failed to read line");

                let match_all = match match_type.trim().parse::<u32>() {
                    Ok(1) => true,
                    _ => false,
                };

                search_foods(&self.db_manager.food_db, &keywords, match_all)
            }
            _ => self.db_manager.food_db.iter().collect(),
        };

        if food_list.is_empty() {
            println!("No foods found.");
            return Ok(());
        }

        println!("\nAvailable foods:");
        for (i, food) in food_list.iter().enumerate() {
            println!(
                "{}. {} ({:.1} kcal/serving)",
                i + 1,
                food.name(),
                food.calories_per_serving()
            );
        }

        print!("Select food number (0 to cancel): ");
        io::stdout().flush().unwrap();
        let mut food_index = String::new();
        io::stdin().read_line(&mut food_index).expect("Failed to read line");

        let food_index = match food_index.trim().parse::<usize>() {
            Ok(0) => {
                return Ok(());
            }
            Ok(i) if i <= food_list.len() => i - 1,
            _ => {
                println!("Invalid selection.");
                return Ok(());
            }
        };

        print!("Enter number of servings: ");
        io::stdout().flush().unwrap();
        let mut servings = String::new();
        io::stdin().read_line(&mut servings).expect("Failed to read line");

        let servings = match servings.trim().parse::<f32>() {
            Ok(s) if s > 0.0 => s,
            _ => {
                println!("Invalid serving amount.");
                return Ok(());
            }
        };

        // Create a command for undo functionality
        let old_log = self.log_manager.get_current_log().clone();

        // Add the selected food to the log
        let selected_food = &food_list[food_index];
        self.log_manager.add_food_entry(selected_food.name(), servings);
        println!("Added {} ({} servings) to log", selected_food.name(), servings);

        // Save the undo command
        let _new_log = self.log_manager.get_current_log().clone();
        self.undo_manager.add_command(
            Box::new(move |log_manager: &mut DailyLogManager| {
                log_manager.update_current_log(&old_log);
                Ok(())
            })
        );

        Ok(())
    }

    fn view_daily_log(&self) {
        println!("\n===== Daily Log for {} =====", self.log_manager.get_current_date());
        let log = self.log_manager.get_current_log();

        if log.food_entries.is_empty() {
            println!("No food entries for today.");
        } else {
            println!("Food entries:");
            for (i, entry) in log.food_entries.iter().enumerate() {
                let food = self.db_manager.get_food_by_name(&entry.food_name);
                match food {
                    Some(f) => {
                        let calories = entry.servings * f.calories_per_serving();
                        println!(
                            "{}. {} - {:.1} servings ({:.1} kcal)",
                            i + 1,
                            entry.food_name,
                            entry.servings,
                            calories
                        );
                    }
                    None =>
                        println!(
                            "{}. {} - {:.1} servings (unknown calories)",
                            i + 1,
                            entry.food_name,
                            entry.servings
                        ),
                }
            }
        }

        if let Some(weight) = log.weight {
            println!("Weight: {:.1} kg", weight);
        }

        if let Some(activity) = &log.activity_level {
            println!("Activity level: {:?}", activity);
        }
    }

    fn delete_food_from_log(&mut self) -> Result<(), String> {
        println!("\n===== Delete Food from Log =====");
        let log = self.log_manager.get_current_log();

        if log.food_entries.is_empty() {
            println!("No food entries to delete.");
            return Ok(());
        }

        println!("Current entries:");
        for (i, entry) in log.food_entries.iter().enumerate() {
            println!("{}. {} - {:.1} servings", i + 1, entry.food_name, entry.servings);
        }

        print!("Select entry to delete (0 to cancel): ");
        io::stdout().flush().unwrap();
        let mut index = String::new();
        io::stdin().read_line(&mut index).expect("Failed to read line");

        let index = match index.trim().parse::<usize>() {
            Ok(0) => {
                return Ok(());
            }
            Ok(i) if i <= log.food_entries.len() => i - 1,
            _ => {
                println!("Invalid selection.");
                return Ok(());
            }
        };

        // Create a command for undo functionality
        let old_log = self.log_manager.get_current_log().clone();

        // Delete the entry
        let entry = &log.food_entries[index];
        println!("Deleted {} ({:.1} servings)", entry.food_name, entry.servings);
        self.log_manager.delete_food_entry(index);

        // Save the undo command
        let _new_log = self.log_manager.get_current_log().clone();
        self.undo_manager.add_command(
            Box::new(move |log_manager: &mut DailyLogManager| {
                log_manager.update_current_log(&old_log);
                Ok(())
            })
        );

        Ok(())
    }

    fn change_date(&mut self) -> Result<(), String> {
        println!("\n===== Change Date =====");
        println!("Current date: {}", self.log_manager.get_current_date());
        println!("Enter new date (YYYY-MM-DD), or press Enter for today:");

        let mut date_str = String::new();
        io::stdin().read_line(&mut date_str).expect("Failed to read line");

        let date_str = date_str.trim();
        if date_str.is_empty() {
            // Set to today
            self.log_manager.set_current_date(Utc::now().date_naive());
        } else {
            // Parse user input
            let date = match NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                Ok(d) => d,
                Err(_) => {
                    println!("Invalid date format. Expected YYYY-MM-DD.");
                    return Ok(());
                }
            };

            self.log_manager.set_current_date(date);
        }

        println!("Date changed to: {}", self.log_manager.get_current_date());
        Ok(())
    }

    // I'll continue adding the rest of the necessary methods
    fn update_weight_activity(&mut self) -> Result<(), String> {
        println!("\n===== Update Weight & Activity Level =====");
        // Copy out what we need from the log before mutating
        let current_weight = self.log_manager.get_current_log().weight;
        let current_activity = self.log_manager.get_current_log().activity_level;

        // Display current values
        if let Some(weight) = current_weight {
            println!("Current weight: {:.1} kg", weight);
        } else {
            println!("Weight not set for today");
        }

        if let Some(activity) = current_activity {
            println!("Current activity level: {:?}", activity);
        } else {
            println!("Activity level not set for today");
        }

        // Get new weight
        print!("Enter new weight in kg (press Enter to skip): ");
        io::stdout().flush().unwrap();
        let mut weight_str = String::new();
        io::stdin().read_line(&mut weight_str).expect("Failed to read line");

        // Create a command for undo functionality
        let old_log = self.log_manager.get_current_log().clone();

        // Update weight if provided
        if !weight_str.trim().is_empty() {
            match weight_str.trim().parse::<f32>() {
                Ok(w) if w > 0.0 => {
                    self.log_manager.update_weight(w);
                    println!("Weight updated to {:.1} kg", w);
                }
                _ => println!("Invalid weight value, not updated."),
            }
        }

        // Get new activity level
        println!("Activity levels:");
        println!("1. Sedentary");
        println!("2. Light");
        println!("3. Moderate");
        println!("4. Very Active");
        println!("5. Extra Active");
        print!("Select new activity level (press Enter to skip): ");
        io::stdout().flush().unwrap();

        let mut activity_str = String::new();
        io::stdin().read_line(&mut activity_str).expect("Failed to read line");

        // Update activity if provided
        if !activity_str.trim().is_empty() {
            let activity = match activity_str.trim().parse::<u32>() {
                Ok(1) => ActivityLevel::Sedentary,
                Ok(2) => ActivityLevel::Light,
                Ok(3) => ActivityLevel::Moderate,
                Ok(4) => ActivityLevel::VeryActive,
                Ok(5) => ActivityLevel::ExtraActive,
                _ => {
                    println!("Invalid activity level, not updated.");
                    self.log_manager
                        .get_current_log()
                        .activity_level.clone()
                        .unwrap_or(ActivityLevel::Sedentary)
                }
            };

            self.log_manager.update_activity_level(activity.clone());
            println!("Activity level updated to {:?}", activity);
        }

        // Save the undo command
        let _new_log = self.log_manager.get_current_log().clone();
        self.undo_manager.add_command(
            Box::new(move |log_manager: &mut DailyLogManager| {
                log_manager.update_current_log(&old_log);
                Ok(())
            })
        );

        Ok(())
    }

    fn add_basic_food(&mut self) -> Result<(), String> {
        println!("\n===== Add Basic Food =====");

        print!("Enter food name: ");
        io::stdout().flush().unwrap();
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read line");
        let name = name.trim().to_string();

        if name.is_empty() {
            println!("Food name cannot be empty.");
            return Ok(());
        }

        // Check if food already exists
        if self.db_manager.get_food_by_name(&name).is_some() {
            println!("A food with this name already exists.");
            return Ok(());
        }

        print!("Enter keywords (separated by spaces): ");
        io::stdout().flush().unwrap();
        let mut keywords_str = String::new();
        io::stdin().read_line(&mut keywords_str).expect("Failed to read line");
        let keywords: Vec<String> = keywords_str
            .trim()
            .split_whitespace()
            .map(String::from)
            .collect();

        print!("Enter calories per serving: ");
        io::stdout().flush().unwrap();
        let mut calories_str = String::new();
        io::stdin().read_line(&mut calories_str).expect("Failed to read line");
        let calories = match calories_str.trim().parse::<f32>() {
            Ok(c) if c >= 0.0 => c,
            _ => {
                println!("Invalid calorie value.");
                return Ok(());
            }
        };

        // Create a new basic food
        let food = BasicFood {
            name: name.clone(),
            keywords,
            calories_per_serving: calories,
        };

        // Add to database
        self.db_manager.add_basic_food(food);
        println!("Added new basic food: {}", name);

        Ok(())
    }

    // I'll implement the remaining methods
    fn create_composite_food(&mut self) -> Result<(), String> {
        println!("\n===== Create Composite Food =====");

        print!("Enter composite food name: ");
        io::stdout().flush().unwrap();
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read line");
        let name = name.trim().to_string();

        if name.is_empty() {
            println!("Food name cannot be empty.");
            return Ok(());
        }

        // Check if food already exists
        if self.db_manager.get_food_by_name(&name).is_some() {
            println!("A food with this name already exists.");
            return Ok(());
        }

        print!("Enter keywords (separated by spaces): ");
        io::stdout().flush().unwrap();
        let mut keywords_str = String::new();
        io::stdin().read_line(&mut keywords_str).expect("Failed to read line");
        let keywords: Vec<String> = keywords_str
            .trim()
            .split_whitespace()
            .map(String::from)
            .collect();

        let mut components: Vec<FoodComponent> = Vec::new();

        loop {
            println!("\nAdd components to composite food (current: {})", components.len());
            println!("1. Search food to add");
            println!("2. List all foods");
            println!("3. Finish adding components");

            print!("Enter choice: ");
            io::stdout().flush().unwrap();
            let mut choice = String::new();
            io::stdin().read_line(&mut choice).expect("Failed to read line");

            match choice.trim().parse::<u32>() {
                Ok(1) => {
                    print!("Enter search term: ");
                    io::stdout().flush().unwrap();
                    let mut search_term = String::new();
                    io::stdin().read_line(&mut search_term).expect("Failed to read line");
                    let search_term = search_term.trim();

                    let results = self.db_manager.search_foods_by_name(search_term);
                    if results.is_empty() {
                        println!("No foods found.");
                        continue;
                    }

                    self.display_food_selection(&results)?;
                    let component = self.select_food_component(&results)?;
                    if let Some(c) = component {
                        components.push(c);
                    }
                }
                Ok(2) => {
                    let foods: Vec<&Box<dyn FoodTrait>> = self.db_manager.food_db.iter().collect();
                    if foods.is_empty() {
                        println!("No foods available.");
                        continue;
                    }

                    self.display_food_selection(&foods)?;
                    let component = self.select_food_component(&foods)?;
                    if let Some(c) = component {
                        components.push(c);
                    }
                }
                Ok(3) => {
                    if components.is_empty() {
                        println!("Cannot create empty composite food.");
                        continue;
                    }
                    break;
                }
                _ => println!("Invalid choice."),
            }
        }

        // Create the composite food
        let food = CompositeFood {
            name,
            keywords,
            components,
        };

        // Add to database
        self.db_manager.add_composite_food(food.clone());
        println!(
            "Added new composite food: {} ({:.1} kcal/serving)",
            food.name,
            food.calories_per_serving()
        );

        Ok(())
    }

    // Change these methods to accept either owned or reference types:
    fn display_food_selection(&self, foods: &[&Box<dyn FoodTrait>]) -> Result<(), String> {
        println!("\nAvailable foods:");
        for (i, food) in foods.iter().enumerate() {
            println!(
                "{}. {} ({:.1} kcal/serving)",
                i + 1,
                food.name(),
                food.calories_per_serving()
            );
        }
        Ok(())
    }

    fn select_food_component(
        &self,
        foods: &[&Box<dyn FoodTrait>]
    ) -> Result<Option<FoodComponent>, String> {
        print!("Select food (0 to cancel): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        let index = match choice.trim().parse::<usize>() {
            Ok(0) => {
                return Ok(None);
            }
            Ok(i) if i <= foods.len() => i - 1,
            _ => {
                println!("Invalid selection.");
                return Ok(None);
            }
        };

        print!("Enter number of servings: ");
        io::stdout().flush().unwrap();

        let mut servings = String::new();
        io::stdin().read_line(&mut servings).expect("Failed to read line");

        let servings = match servings.trim().parse::<f32>() {
            Ok(s) if s > 0.0 => s,
            _ => {
                println!("Invalid serving amount.");
                return Ok(None);
            }
        };

        let selected_food = foods[index];
        Ok(
            Some(FoodComponent {
                food_name: selected_food.name().to_string(),
                servings,
            })
        )
    }
    fn search_food_database(&self) -> Result<(), String> {
        println!("\n===== Search Food Database =====");
        println!("1. Search by name");
        println!("2. Search by keywords");
        print!("Enter choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        let foods = match choice.trim().parse::<u32>() {
            Ok(1) => {
                print!("Enter food name: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read line");
                self.db_manager.search_foods_by_name(name.trim())
            }
            Ok(2) => {
                print!("Enter keywords (separated by spaces): ");
                io::stdout().flush().unwrap();
                let mut keywords = String::new();
                io::stdin().read_line(&mut keywords).expect("Failed to read line");

                let keywords: Vec<&str> = keywords.trim().split_whitespace().collect();

                print!("Match: 1. All keywords, 2. Any keyword: ");
                io::stdout().flush().unwrap();
                let mut match_type = String::new();
                io::stdin().read_line(&mut match_type).expect("Failed to read line");

                let match_all = match match_type.trim().parse::<u32>() {
                    Ok(1) => true,
                    _ => false,
                };

                search_foods(&self.db_manager.food_db, &keywords, match_all)
            }
            _ => {
                println!("Invalid choice.");
                return Ok(());
            }
        };

        if foods.is_empty() {
            println!("No foods found.");
        } else {
            println!("\nSearch results:");
            for (i, food) in foods.iter().enumerate() {
                println!(
                    "{}. {} ({:.1} kcal/serving)",
                    i + 1,
                    food.name(),
                    food.calories_per_serving()
                );

                // If it's a composite food, show components
                if let Some(composite) = self.db_manager.get_composite_food(&food.name()) {
                    for comp in &composite.components {
                        println!("   - {} ({} servings)", comp.food_name, comp.servings);
                    }
                }
            }
        }

        Ok(())
    }

    fn update_user_profile(&mut self) -> Result<(), String> {
        println!("\n===== Update User Profile =====");

        let profile = self.db_manager.profile.as_ref().unwrap().clone();

        println!("Current profile:");
        println!("Gender: {:?}", profile.gender);
        println!("Height: {} cm", profile.height_cm);
        println!("Age: {} years", profile.age);
        println!("Calculation method: {:?}", profile.calorie_method);

        println!("\nUpdate profile (leave blank to keep current value)");

        // Update gender
        println!("Gender: 1. Male, 2. Female");
        print!("Choose gender [{}]: ", match profile.gender {
            Gender::Male => "1",
            Gender::Female => "2",
        });
        io::stdout().flush().unwrap();

        let mut gender_choice = String::new();
        io::stdin().read_line(&mut gender_choice).expect("Failed to read line");

        let gender = if gender_choice.trim().is_empty() {
            profile.gender
        } else {
            match gender_choice.trim().parse::<u32>() {
                Ok(1) => Gender::Male,
                Ok(2) => Gender::Female,
                _ => {
                    println!("Invalid gender selection, keeping current value.");
                    profile.gender
                }
            }
        };

        // Update height
        print!("Enter height in cm [{}]: ", profile.height_cm);
        io::stdout().flush().unwrap();

        let mut height_str = String::new();
        io::stdin().read_line(&mut height_str).expect("Failed to read line");

        let height_cm = if height_str.trim().is_empty() {
            profile.height_cm
        } else {
            match height_str.trim().parse::<f32>() {
                Ok(h) if h > 0.0 => h,
                _ => {
                    println!("Invalid height value, keeping current value.");
                    profile.height_cm
                }
            }
        };

        // Update age
        print!("Enter age in years [{}]: ", profile.age);
        io::stdout().flush().unwrap();

        let mut age_str = String::new();
        io::stdin().read_line(&mut age_str).expect("Failed to read line");

        let age = if age_str.trim().is_empty() {
            profile.age
        } else {
            match age_str.trim().parse::<u32>() {
                Ok(a) if a > 0 => a,
                _ => {
                    println!("Invalid age value, keeping current value.");
                    profile.age
                }
            }
        };

        // Update calculation method
        println!("Calorie calculation method:");
        println!("1. Harris-Benedict");
        println!("2. Mifflin-St Jeor");
        print!("Choose method [{}]: ", match profile.calorie_method {
            CalorieCalculationMethod::HarrisBenedict => "1",
            CalorieCalculationMethod::MifflinStJeor => "2",
        });
        io::stdout().flush().unwrap();

        let mut method_str = String::new();
        io::stdin().read_line(&mut method_str).expect("Failed to read line");

        let method = if method_str.trim().is_empty() {
            profile.calorie_method
        } else {
            match method_str.trim().parse::<u32>() {
                Ok(1) => CalorieCalculationMethod::HarrisBenedict,
                Ok(2) => CalorieCalculationMethod::MifflinStJeor,
                _ => {
                    println!("Invalid method selection, keeping current value.");
                    profile.calorie_method
                }
            }
        };

        // Create updated profile
        let updated_profile = UserProfile {
            gender,
            height_cm,
            age,
            calorie_method: method,
            weight_kg: profile.weight_kg, // Include this field
            activity_level: profile.activity_level, // Include this field
        };

        // Update profile in database manager
        self.db_manager.profile = Some(updated_profile);
        println!("Profile updated successfully!");

        Ok(())
    }

    fn save_data(&mut self) -> Result<(), String> {
        println!("Saving data...");

        self.db_manager.logs = self.log_manager.get_all_logs().clone();
        self.db_manager.save_to_files()?;

        println!("Data saved successfully!");
        Ok(())
    }

    fn undo_last_action(&mut self) -> Result<(), String> {
        match self.undo_manager.undo(&mut self.log_manager) {
            Ok(true) => println!("Action undone successfully."),
            Ok(false) => println!("Nothing to undo."),
            Err(e) => println!("Error undoing action: {}", e),
        }

        Ok(())
    }

    // Replace the create_user_profile method with this:

    fn create_user_profile(&mut self) -> Result<(), String> {
        println!("\n===== Create User Profile =====");

        // Get gender
        println!("Select gender:");
        println!("1. Male");
        println!("2. Female");
        print!("Enter choice: ");
        io::stdout().flush().unwrap();

        let mut gender_choice = String::new();
        io::stdin().read_line(&mut gender_choice).expect("Failed to read line");

        let gender = match gender_choice.trim().parse::<u32>() {
            Ok(1) => Gender::Male,
            Ok(2) => Gender::Female,
            _ => {
                return Err("Invalid gender selection.".to_string());
            }
        };

        // Get height
        print!("Enter height in cm: ");
        io::stdout().flush().unwrap();

        let mut height_str = String::new();
        io::stdin().read_line(&mut height_str).expect("Failed to read line");

        let height_cm = match height_str.trim().parse::<f32>() {
            Ok(h) if h > 0.0 => h,
            _ => {
                return Err("Invalid height value.".to_string());
            }
        };

        // Get age
        print!("Enter age in years: ");
        io::stdout().flush().unwrap();

        let mut age_str = String::new();
        io::stdin().read_line(&mut age_str).expect("Failed to read line");

        let age = match age_str.trim().parse::<u32>() {
            Ok(a) if a > 0 => a,
            _ => {
                return Err("Invalid age value.".to_string());
            }
        };

        // Get calculation method
        println!("Select calorie calculation method:");
        println!("1. Harris-Benedict");
        println!("2. Mifflin-St Jeor");
        print!("Enter choice: ");
        io::stdout().flush().unwrap();

        let mut method_str = String::new();
        io::stdin().read_line(&mut method_str).expect("Failed to read line");

        let method = match method_str.trim().parse::<u32>() {
            Ok(1) => CalorieCalculationMethod::HarrisBenedict,
            Ok(2) => CalorieCalculationMethod::MifflinStJeor,
            _ => {
                return Err("Invalid method selection.".to_string());
            }
        };

        // Get initial weight
        print!("Enter current weight in kg: ");
        io::stdout().flush().unwrap();

        let mut weight_str = String::new();
        io::stdin().read_line(&mut weight_str).expect("Failed to read line");

        let weight = match weight_str.trim().parse::<f32>() {
            Ok(w) if w > 0.0 => w,
            _ => {
                return Err("Invalid weight value.".to_string());
            }
        };

        // Get activity level
        println!("Select activity level:");
        println!("1. Sedentary");
        println!("2. Light");
        println!("3. Moderate");
        println!("4. Very Active");
        println!("5. Extra Active");
        print!("Enter choice: ");
        io::stdout().flush().unwrap();

        let mut activity_choice = String::new();
        io::stdin().read_line(&mut activity_choice).expect("Failed to read line");

        let activity = match activity_choice.trim().parse::<u32>() {
            Ok(1) => ActivityLevel::Sedentary,
            Ok(2) => ActivityLevel::Light,
            Ok(3) => ActivityLevel::Moderate,
            Ok(4) => ActivityLevel::VeryActive,
            Ok(5) => ActivityLevel::ExtraActive,
            _ => {
                return Err("Invalid activity level selection.".to_string());
            }
        };

        // Create user profile
        let profile = UserProfile {
            gender,
            height_cm,
            age,
            calorie_method: method,
            weight_kg: weight,
            activity_level: activity,
        };

        // Update profile in database manager
        self.db_manager.profile = Some(profile);
        println!("Profile created successfully!");

        // Update the log with weight and activity
        self.log_manager.update_weight(weight);
        self.log_manager.update_activity_level(activity);

        // Save all data
        self.save_data()?;

        println!("Initial profile setup complete!");
        Ok(())
    }
}

// Implementation of unit tests for CLI
// #[cfg(test)]
mod tests {
    use super::*;
    use crate::models::DailyLog;


    // Replace the create_test_db_manager function with this:

    fn create_test_db_manager() -> DatabaseManager {
        // use std::path::PathBuf;
        use std::env::temp_dir;

        // Create temporary file paths for testing
        let temp_dir = temp_dir();
        let food_db_path = temp_dir.join("test_food_db.json");
        let logs_path = temp_dir.join("test_logs.json");
        let profile_path = temp_dir.join("test_profile.json");

        // Create the database manager
        let mut db_manager = DatabaseManager::new(&food_db_path, &logs_path, &profile_path).expect(
            "Failed to create test database manager"
        );

        // Add basic foods
        db_manager.add_basic_food(BasicFood {
            name: "Apple".to_string(),
            keywords: vec!["fruit".to_string(), "sweet".to_string()],
            calories_per_serving: 95.0,
        });

        db_manager.add_basic_food(BasicFood {
            name: "Chicken Breast".to_string(),
            keywords: vec!["meat".to_string(), "protein".to_string()],
            calories_per_serving: 165.0,
        });

        // Add lettuce for the composite food
        db_manager.add_basic_food(BasicFood {
            name: "Lettuce".to_string(),
            keywords: vec!["vegetable".to_string(), "salad".to_string()],
            calories_per_serving: 15.0,
        });

        // Add a composite food
        db_manager.add_composite_food(CompositeFood {
            name: "Chicken Salad".to_string(),
            keywords: vec!["meal".to_string(), "lunch".to_string()],
            components: vec![
                FoodComponent {
                    food_name: "Chicken Breast".to_string(),
                    servings: 1.0,
                },
                FoodComponent {
                    food_name: "Lettuce".to_string(),
                    servings: 2.0,
                }
            ],
        });

        // Set user profile
        let profile = UserProfile {
            gender: Gender::Male,
            height_cm: 175.0,
            age: 30,
            calorie_method: CalorieCalculationMethod::HarrisBenedict,
            weight_kg: 70.0,
            activity_level: ActivityLevel::Moderate,
        };
        db_manager.profile = Some(profile);

        // Create logs
        let today = Utc::now().date_naive();
        let log_entry = LogEntry::new("Apple".to_string(), 1.0);

        // Add log entry to the current day
        let  daily_log = DailyLog {
            date: today,
            food_entries: vec![log_entry],
            weight: Some(70.0),
            activity_level: Some(ActivityLevel::Moderate),
        };

        // Insert the log into the database
        db_manager.logs.insert(today.to_string(), daily_log);

        db_manager
    }
    #[test]
    fn test_cli_creation() {
        let db_manager = create_test_db_manager();
        let cli = CLI::new(db_manager);

        assert!(cli.db_manager.profile.is_some());
        assert_eq!(cli.db_manager.food_db.len(), 3);
    }

    // Additional tests would be added here to test individual CLI methods
    // Note: Most CLI methods are interactive and difficult to test directly
    // without mocking stdin/stdout, so we focus on testing the underlying logic
}

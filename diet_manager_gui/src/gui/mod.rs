// src/gui/mod.rs
mod home_screen;
mod add_basic_food_screen;
mod add_composite_food_screen;
pub mod download_food_data_screen;
mod view_daily_log_screen;
mod login_screen; // Add this line
mod register_screen; // Add this line
pub mod undo_manager; // Add this line
mod add_food_to_log_screen; // Add this line
mod edit_food_log_screen;   // Add this line
mod update_profile_screen;  // Add this line
pub mod styling; // Add this line

pub use home_screen::*;
pub use add_basic_food_screen::*;
pub use add_composite_food_screen::*;
pub use download_food_data_screen::*;
pub use view_daily_log_screen::*;
pub use login_screen::*; // Add this line
pub use register_screen::*; // Add this line
// pub use undo_manager::*; // Add this line
pub use add_food_to_log_screen::*; // Add this line
pub use edit_food_log_screen::*;   // Add this line
pub use update_profile_screen::*;  // Add this line

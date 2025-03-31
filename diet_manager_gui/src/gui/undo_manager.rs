use crate::models::Database;
use std::io::Write;
use chrono::Local;

pub struct UndoManager {
    history: Vec<(Database, String)>,  // (Database snapshot, action description)
    capacity: usize,
    current_index: usize,
}

impl UndoManager {
    pub fn new(capacity: usize) -> Self {
        Self {
            history: Vec::with_capacity(capacity),
            capacity,
            current_index: 0,
        }
    }

    pub fn initialize(&mut self, initial_db: Database) {
        self.history.clear();
        self.history.push((initial_db, "Initial state".to_string()));
        self.current_index = 0;
    }

    pub fn record_action(&mut self, db_snapshot: Database, description: &str) {
        println!("Recording action: {} at index {}", description, self.current_index);

        // If we're not at the end of the history, truncate
        if !self.history.is_empty() && self.current_index < self.history.len() - 1 {
            println!("Truncating history from {} to {}", self.history.len(), self.current_index + 1);
            self.history.truncate(self.current_index + 1);
        }

        // Add new action
        self.history.push((db_snapshot.clone(), description.to_string()));
        self.current_index = self.history.len() - 1;
        println!("Current index after recording: {}", self.current_index);

        // Remove oldest entries if exceeding capacity
        while self.history.len() > self.capacity {
            self.history.remove(0);
            self.current_index = self.current_index.saturating_sub(1);
        }

        // Write to log file
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let log_entry = format!("[{}] {}\n", timestamp, description);

        std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("diet_manager_actions.log")
            .map(|mut file| file.write_all(log_entry.as_bytes()))
            .unwrap_or_else(|e| {
                eprintln!("Failed to write to log file: {}", e);
                Ok(())
            }).ok();
    }

    pub fn can_undo(&self) -> bool {
        self.current_index > 0
    }

    pub fn undo(&mut self) -> Option<(Database, String)> {
        if self.can_undo() {
            println!("Undoing from index {} of {}", self.current_index, self.history.len());

            // The action being undone is at the current index
            let current_action = self.history[self.current_index].1.clone();

            // Move back one step in history
            self.current_index -= 1;

            // Get the state we're reverting to
            let (db, _) = &self.history[self.current_index];

            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let log_entry = format!("[{}] UNDO: {}\n", timestamp, current_action);

            println!("New current index: {}", self.current_index);

            // Log the undo action
            std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open("diet_manager_actions.log")
                .map(|mut file| file.write_all(log_entry.as_bytes()))
                .unwrap_or_else(|e| {
                    eprintln!("Failed to write undo to log file: {}", e);
                    Ok(())
                }).ok();

            return Some((db.clone(), current_action));
        }
        None
    }

    pub fn last_action_description(&self) -> Option<String> {
        if self.history.len() > 1 && self.current_index > 0 {
            Some(self.history[self.current_index].1.clone())
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.history.clear();
        self.current_index = 0;
    }

    // Debug function to print state of the undo manager
    pub fn debug_print_state(&self) {
        println!("UndoManager state:");
        println!("  - History length: {}", self.history.len());
        println!("  - Current index: {}", self.current_index);
        println!("  - Can undo: {}", self.can_undo());

        for (i, (_, action)) in self.history.iter().enumerate() {
            println!("  - [{}] {}{}", i,
                if i == self.current_index { "-> " } else { "" },
                action);
        }
    }
}
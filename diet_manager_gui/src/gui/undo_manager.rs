use crate::models::Database;
use std::io::Write;

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
        // First check if history has items to avoid subtraction overflow
        if !self.history.is_empty() && self.current_index < self.history.len() - 1 {
            self.history.truncate(self.current_index + 1);
        }

        // Add new action
        self.history.push((db_snapshot.clone(), description.to_string()));
        self.current_index = self.history.len() - 1;

        // Remove oldest entries if exceeding capacity
        if self.history.len() > self.capacity {
            self.history.remove(0);
            self.current_index = self.history.len() - 1;
        }

        // Write to a separate log file
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
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
            println!("Undoing action at index {}", self.current_index);
            self.current_index -= 1;
            let (db, desc) = &self.history[self.current_index];

            // Log to both console and file
            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let log_entry = format!("[{}] UNDO: {}\n", timestamp, desc);

            std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open("diet_manager_actions.log")
                .map(|mut file| file.write_all(log_entry.as_bytes()))
                .unwrap_or_else(|e| {
                    eprintln!("Failed to write undo to log file: {}", e);
                    Ok(())
                }).ok();

            return Some((db.clone(), desc.clone()));
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
}
use crate::models::Database;

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
        self.history.push((db_snapshot, description.to_string()));
        self.current_index = self.history.len() - 1;

        // Remove oldest entries if exceeding capacity
        if self.history.len() > self.capacity {
            self.history.remove(0);
            self.current_index = self.history.len() - 1;
        }
    }

    pub fn can_undo(&self) -> bool {
        self.current_index > 0
    }

    pub fn undo(&mut self) -> Option<(Database, String)> {
        if self.can_undo() {
            self.current_index -= 1;
            let (db, desc) = &self.history[self.current_index];
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
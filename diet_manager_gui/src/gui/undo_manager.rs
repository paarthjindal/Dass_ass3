use crate::models::Database;
use std::collections::VecDeque;

/// Represents a database state along with a description of the action that created it
#[derive(Clone)]
pub struct DatabaseState {
    /// The database state
    pub database: Database,
    /// Description of the action that led to this state
    pub description: String,
}

pub struct UndoManager {
    /// Maximum number of states to keep in history
    max_states: usize,
    /// History of previous states for undo (most recent at the end)
    undo_stack: VecDeque<DatabaseState>,
    /// History of undone states for redo (most recently undone at the end)
    redo_stack: VecDeque<DatabaseState>,
    /// Current database state (not in either stack)
    current_state: Option<DatabaseState>,
}

impl UndoManager {
    /// Create a new UndoManager with the given maximum number of states
    pub fn new(max_states: usize) -> Self {
        Self {
            max_states,
            undo_stack: VecDeque::with_capacity(max_states),
            redo_stack: VecDeque::with_capacity(max_states),
            current_state: None,
        }
    }

    /// Initialize with a starting state
    pub fn initialize(&mut self, database: Database) {
        self.current_state = Some(DatabaseState {
            database,
            description: "Initial state".to_string(),
        });
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// Record a new state after an action
    pub fn record_action(&mut self, database: Database, description: &str) {
        // Store the current state in the undo stack if it exists
        if let Some(current) = self.current_state.take() {
            self.undo_stack.push_back(current);

            // Maintain maximum stack size
            if self.undo_stack.len() > self.max_states {
                self.undo_stack.pop_front();
            }
        }

        // Set the new state as current
        self.current_state = Some(DatabaseState {
            database,
            description: description.to_string(),
        });

        // Clear the redo stack since we've taken a new action
        self.redo_stack.clear();
    }

    /// Undo the last action
    pub fn undo(&mut self) -> Option<Database> {
        // Can't undo if no prior states
        if self.undo_stack.is_empty() {
            return None;
        }

        // Move current state to redo stack
        if let Some(current) = self.current_state.take() {
            self.redo_stack.push_back(current);

            // Maintain maximum stack size
            if self.redo_stack.len() > self.max_states {
                self.redo_stack.pop_front();
            }
        }

        // Get previous state
        let previous = self.undo_stack.pop_back()?;
        let db_clone = previous.database.clone();
        self.current_state = Some(previous);

        Some(db_clone)
    }

    /// Redo a previously undone action
    pub fn redo(&mut self) -> Option<Database> {
        // Can't redo if no undone states
        if self.redo_stack.is_empty() {
            return None;
        }

        // Move current state to undo stack
        if let Some(current) = self.current_state.take() {
            self.undo_stack.push_back(current);

            // Maintain maximum stack size
            if self.undo_stack.len() > self.max_states {
                self.undo_stack.pop_front();
            }
        }

        // Get next state
        let next = self.redo_stack.pop_back()?;
        let db_clone = next.database.clone();
        self.current_state = Some(next);

        Some(db_clone)
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }
}
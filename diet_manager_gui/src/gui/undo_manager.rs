use crate::models::Database;

pub struct UndoManager {
    max_states: usize,
    states: Vec<Database>,
}

impl UndoManager {
    pub fn new(max_states: usize) -> Self {
        Self {
            max_states,
            states: Vec::with_capacity(max_states),
        }
    }

    pub fn push_state(&mut self, state: Database) {
        if self.states.len() >= self.max_states {
            self.states.remove(0);
        }
        self.states.push(state);
    }

    pub fn undo(&mut self) -> Option<Database> {
        self.states.pop()
    }
}
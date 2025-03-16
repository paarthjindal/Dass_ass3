use std::collections::VecDeque;
use crate::models::Command;
use crate::log::daily::DailyLogManager;

pub struct UndoManager {
    commands: VecDeque<Command>,
    max_commands: usize,
}

impl UndoManager {
    pub fn new(max_commands: usize) -> Self {
        UndoManager {
            commands: VecDeque::new(),
            max_commands,
        }
    }

    pub fn add_command(&mut self, command: Command) {
        if self.commands.len() >= self.max_commands {
            self.commands.pop_front();
        }
        self.commands.push_back(command);
    }

    pub fn undo(&mut self, log_manager: &mut DailyLogManager) -> Result<bool, String> {
        if let Some(command) = self.commands.pop_back() {
            command(log_manager)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.commands.is_empty()
    }
}
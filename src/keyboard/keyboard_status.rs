use crate::CellValue;
use std::collections::HashMap;

use super::key::{KeyStatus, KeyValue};

#[derive(PartialEq, Clone)]
pub struct KeyboardStatus {
    keys: HashMap<char, KeyStatus>,
}

impl KeyboardStatus {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
        }
    }

    pub fn get_status(&self, letter: char) -> KeyValue {
        let status = self
            .keys
            .get(&letter.to_ascii_lowercase())
            .cloned()
            .unwrap_or(KeyStatus::Unused);
        KeyValue { status, letter }
    }

    pub fn update_status(&mut self, guess: &[CellValue]) {
        for cell in guess {
            match cell {
                CellValue::Absent(c) => {
                    self.keys.insert(*c, KeyStatus::Absent);
                }
                CellValue::Present(c) => {
                    self.keys.insert(*c, KeyStatus::Present);
                }
                CellValue::Correct(c) => {
                    self.keys.insert(*c, KeyStatus::Correct);
                }
                _ => {}
            }
        }
    }
}

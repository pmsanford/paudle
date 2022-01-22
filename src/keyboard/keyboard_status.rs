use crate::CellValue;
use std::collections::HashMap;

use super::key::{KeyStatus, KeyValue};

#[derive(PartialEq, Clone, Default)]
pub struct KeyboardStatus {
    keys: HashMap<char, KeyStatus>,
}

impl KeyboardStatus {
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
                    // Only set absent if no entry at all
                    self.keys.entry(*c).or_insert(KeyStatus::Absent);
                }
                CellValue::Present(c) => {
                    // If the key status is anything but correct, set Present
                    self.keys
                        .entry(*c)
                        .and_modify(|e| {
                            if *e != KeyStatus::Correct {
                                *e = KeyStatus::Present;
                            }
                        })
                        .or_insert(KeyStatus::Present);
                }
                CellValue::Correct(c) => {
                    // Always set correct
                    self.keys.insert(*c, KeyStatus::Correct);
                }
                _ => {}
            }
        }
    }
}

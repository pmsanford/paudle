use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use web_sys::console;

use crate::GameState;

use super::{board::CellValue, Paudle};

pub const SAVE_KEY: &str = "paudle_save_v1";

#[derive(Serialize, Deserialize)]
pub struct SaveState {
    word: String,
    guesses: Vec<Vec<CellValue>>,
}

impl SaveState {
    pub fn from_live(from: &Paudle) -> Self {
        Self {
            word: from.word.clone(),
            guesses: from.guesses.clone(),
        }
    }
}

pub fn update_saved_state(live: &Paudle) {
    if live.game_state == GameState::InProgress {
        if let Err(e) = LocalStorage::set(SAVE_KEY, SaveState::from_live(live)) {
            console::log_1(&format!("Couldn't save game state: {}", e).into());
        }
    } else {
        LocalStorage::delete(SAVE_KEY);
    }
}

pub fn load_saved_sate() -> Option<SaveState> {
    let save_state: gloo_storage::Result<SaveState> = LocalStorage::get(SAVE_KEY);
    match save_state {
        Ok(save_state) => Some(save_state),
        Err(gloo_storage::errors::StorageError::KeyNotFound(_)) => None,
        Err(e) => {
            console::log_1(&format!("Found game state but couldn't deserialize: {}", e).into());
            LocalStorage::delete(SAVE_KEY);
            None
        }
    }
}

impl From<SaveState> for Paudle {
    fn from(other: SaveState) -> Self {
        let mut new = Self {
            word: other.word,
            ..Paudle::default()
        };

        other.guesses.into_iter().for_each(|g| new.add_guess(g));

        new
    }
}

use std::collections::HashMap;

use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use web_sys::console;

use crate::{GameMode, GameState};

use super::{board::CellValue, Paudle};

pub const SAVE_KEY: &str = "paudle_save_v1";
pub const HISTORY_KEY: &str = "paudle_history_v1";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SaveState {
    word: String,
    guesses: Vec<Vec<CellValue>>,
    game_mode: GameMode,
}

impl SaveState {
    pub fn from_live(from: &Paudle) -> Self {
        Self {
            word: from.word.clone(),
            guesses: from.guesses.clone(),
            game_mode: from.game_mode.clone(),
        }
    }
}

pub fn update_saved_state(live: &Paudle) {
    if live.game_state == GameState::InProgress {
        if let Err(e) = LocalStorage::set(SAVE_KEY, SaveState::from_live(live)) {
            console::log_1(&format!("Couldn't save game state: {}", e).into());
        }
    } else {
        if let GameMode::Daily(ts) = live.game_mode {
            let mut history = load_game_history();
            history.scores.insert(ts, SaveState::from_live(live));
            if let Err(e) = LocalStorage::set(HISTORY_KEY, history) {
                console::log_1(&format!("Couldn't save game history: {}", e).into());
            }
        }
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
            game_mode: other.game_mode,
            ..Paudle::default()
        };

        other.guesses.into_iter().for_each(|g| new.add_guess(g));

        new
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GameHistory {
    pub scores: HashMap<i64, SaveState>,
}

pub fn load_game_history() -> GameHistory {
    let history: gloo_storage::Result<GameHistory> = LocalStorage::get(HISTORY_KEY);
    match history {
        Ok(history) => history,
        Err(gloo_storage::errors::StorageError::KeyNotFound(_)) => GameHistory::default(),
        Err(e) => {
            console::log_1(&format!("Found game history but couldn't deserialize: {}", e).into());
            LocalStorage::delete(HISTORY_KEY);
            GameHistory::default()
        }
    }
}

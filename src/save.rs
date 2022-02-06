use std::collections::HashMap;

use chrono::NaiveDateTime;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use web_sys::console;

use crate::{GameMode, GameState};

use super::{board::CellValue, Paudle};

pub const SAVE_KEY: &str = "paudle_save_v1";
pub const HISTORY_KEY: &str = "paudle_history_v1";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SaveState {
    pub word: String,
    pub guesses: Vec<Vec<CellValue>>,
    pub game_mode: GameMode,
}

impl SaveState {
    pub fn from_live(from: &Paudle) -> Self {
        Self {
            word: from.word.clone(),
            guesses: from.guesses.clone(),
            game_mode: from.game_mode.clone(),
        }
    }

    pub fn was_won(&self) -> bool {
        self.guesses[self.guesses.len() - 1]
            .iter()
            .map(|v| match v {
                CellValue::Typing(c)
                | CellValue::Absent(c)
                | CellValue::Present(c)
                | CellValue::Correct(c) => c,
                CellValue::Empty => &' ',
            })
            .collect::<String>()
            == self.word
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

impl GameHistory {
    pub fn wins(&self) -> usize {
        self.scores.values().filter(|v| v.was_won()).count()
    }

    pub fn won_last(&self) -> bool {
        self.scores
            .keys()
            .max()
            .and_then(|k| self.scores.get(k).map(SaveState::was_won))
            .unwrap_or(false)
    }

    pub fn current_streak(&self) -> usize {
        let last_streak = *self.streaks().get(0).unwrap_or(&0);

        if self.won_last() {
            last_streak
        } else {
            0
        }
    }

    pub fn max_streak(&self) -> usize {
        *self.streaks().iter().max().unwrap_or(&0)
    }

    fn streaks(&self) -> Vec<usize> {
        let mut dates = self
            .scores
            .iter()
            .map(|(k, v)| (chrono::NaiveDateTime::from_timestamp(*k, 0), v.was_won()))
            .collect::<Vec<_>>();
        dates.sort_unstable_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());

        let dates = dates.into_iter().rev().collect::<Vec<_>>();

        let mut count = 0;
        let mut prev = None;
        let mut streaks = Vec::new();

        for (date, won) in &dates {
            if prev.map(|p: &NaiveDateTime| *p - *date > chrono::Duration::days(1)) == Some(true) {
                if count > 0 {
                    streaks.push(count);
                }
                count = 0;
            }
            if *won {
                count += 1;
            } else {
                if count > 0 {
                    streaks.push(count);
                }
                count = 0;
            }
            prev = Some(date);
        }

        if count > 0 {
            streaks.push(count);
        }

        streaks
    }
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

#[cfg(test)]
mod test {
    use chrono::NaiveDate;

    use crate::{board::CellValue, GameMode};

    use super::{GameHistory, SaveState};

    #[test]
    fn test_streak() {
        let day1 = NaiveDate::from_ymd(1987, 11, 11).and_hms(0, 0, 0);
        let winner = |ts| SaveState {
            word: "pauls".into(),
            guesses: vec![vec![
                CellValue::Correct('p'),
                CellValue::Correct('a'),
                CellValue::Correct('u'),
                CellValue::Correct('l'),
                CellValue::Correct('s'),
            ]],
            game_mode: GameMode::Daily(ts),
        };
        let loser = |ts| SaveState {
            word: "pauls".into(),
            guesses: vec![vec![
                CellValue::Correct('s'),
                CellValue::Correct('l'),
                CellValue::Correct('u'),
                CellValue::Correct('a'),
                CellValue::Correct('p'),
            ]],
            game_mode: GameMode::Daily(ts),
        };

        let mut history = GameHistory::default();

        let ts = |days| (day1 + chrono::Duration::days(days)).timestamp();

        assert_eq!(history.current_streak(), 0);

        history.scores.insert(ts(0), winner(ts(0)));

        assert_eq!(history.current_streak(), 1);

        history.scores.insert(ts(1), loser(ts(1)));
        history.scores.insert(ts(2), winner(ts(2)));
        history.scores.insert(ts(3), winner(ts(3)));
        history.scores.insert(ts(4), winner(ts(4)));

        assert_eq!(history.current_streak(), 3);

        history.scores.insert(ts(5), loser(ts(5)));

        assert_eq!(history.current_streak(), 0);

        history.scores.insert(ts(6), loser(ts(6)));

        assert_eq!(history.current_streak(), 0);

        history.scores.insert(ts(7), winner(ts(7)));

        // missed day 8
        history.scores.insert(ts(9), winner(ts(9)));
        assert_eq!(history.current_streak(), 1);
    }
}

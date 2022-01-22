#![allow(clippy::module_name_repetitions)]
mod board;
mod keyboard;
mod scoreboard;

use gloo_events::EventListener;
use gloo_storage::{LocalStorage, Storage};
use rand::{prelude::IteratorRandom, thread_rng};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, mem};
use wasm_bindgen::JsCast;
#[allow(unused_imports)]
use web_sys::console;
use web_sys::window;
use yew::prelude::*;

use board::{Board, CellValue};
use keyboard::{Keyboard, KeyboardStatus, BACKSPACE, ENTER};
use scoreboard::Scoreboard;

const SAVE_KEY: &str = "paudle_save_v1";

const WORD_LIST: &str = include_str!("awords.txt");
struct Paudle {
    word: String,
    guesses: Vec<Vec<CellValue>>,
    keyboard_status: KeyboardStatus,
    current_guess: String,
    word_length: usize,
    max_guesses: usize,
    bad_guess: bool,
    game_state: GameState,
}

impl Default for Paudle {
    fn default() -> Self {
        let word_choices = WORD_LIST.lines();
        let word = word_choices.choose(&mut thread_rng()).unwrap().to_string();
        Self {
            word,
            guesses: Vec::new(),
            keyboard_status: KeyboardStatus::default(),
            current_guess: String::new(),
            word_length: 5,
            max_guesses: 6,
            bad_guess: false,
            game_state: GameState::InProgress,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SaveState {
    word: String,
    guesses: Vec<Vec<CellValue>>,
}

impl SaveState {
    fn from_live(from: &Paudle) -> Self {
        Self {
            word: from.word.clone(),
            guesses: from.guesses.clone(),
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

pub enum PaudleMsg {
    TypeLetter(char),
    Backspace,
    Submit,
}

#[derive(PartialEq, Clone)]
pub enum GameState {
    InProgress,
    Won,
    Lost,
}

impl Paudle {
    fn eval_and_add_guess(&mut self, guess: &str) {
        let new_guess = evaluate_guess(&self.word, &guess.to_lowercase());
        self.add_guess(new_guess);
    }

    fn add_guess(&mut self, new_guess: Vec<CellValue>) {
        self.keyboard_status.update_status(&new_guess);
        let correct = new_guess.iter().all(|g| matches!(g, CellValue::Correct(_)));
        self.guesses.push(new_guess);
        if correct {
            self.game_state = GameState::Won;
        } else if self.guesses.len() == self.max_guesses {
            self.game_state = GameState::Lost;
        }
    }
}

impl Component for Paudle {
    type Message = PaudleMsg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let save_state: gloo_storage::Result<SaveState> = LocalStorage::get(SAVE_KEY);
        match save_state {
            Ok(save_state) => save_state.into(),
            Err(gloo_storage::errors::StorageError::KeyNotFound(_)) => Self::default(),
            Err(e) => {
                console::log_1(&format!("Found game state but couldn't deserialize: {}", e).into());
                LocalStorage::delete(SAVE_KEY);
                Self::default()
            }
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        if self.game_state != GameState::InProgress {
            return false;
        }
        match msg {
            PaudleMsg::TypeLetter(c) if self.current_guess.len() < self.word_length => {
                self.current_guess.push(c.to_ascii_lowercase());
                true
            }
            PaudleMsg::TypeLetter(_) => false,
            PaudleMsg::Backspace => {
                self.bad_guess = false;
                self.current_guess.pop();
                true
            }
            PaudleMsg::Submit => {
                if self.current_guess.len() == self.word_length {
                    if !WORD_LIST.contains(&self.current_guess) {
                        self.bad_guess = true;
                        return true;
                    }
                    let current_guess = mem::take(&mut self.current_guess);
                    self.eval_and_add_guess(&current_guess);
                    if self.game_state == GameState::InProgress {
                        if let Err(e) = LocalStorage::set(SAVE_KEY, SaveState::from_live(self)) {
                            console::log_1(&format!("Couldn't save game state: {}", e).into());
                        }
                    } else {
                        LocalStorage::delete(SAVE_KEY);
                    }
                    true
                } else {
                    false
                }
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }

        let on_keypress = ctx.link().batch_callback(handle_keypress);

        let window = window().expect("No window? Where am I?");

        EventListener::new(&window, "keydown", move |e: &Event| {
            if let Ok(e) = e.clone().dyn_into::<KeyboardEvent>() {
                on_keypress.emit(e);
            }
        })
        .forget();
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cb = ctx.link().callback(|msg: PaudleMsg| msg);

        // tabIndex=0 for keyboard events: https://stackoverflow.com/questions/43503964/onkeydown-event-not-working-on-divs-in-react/44434971#44434971
        html! {
            <div class="page">
                <Board
                    current_guess={self.current_guess.clone()}
                    guesses={self.guesses.clone()}
                    row_count={self.max_guesses}
                    word_length={self.word_length}
                    bad_guess={self.bad_guess}
                />
                <Keyboard key_press={cb} keys={self.keyboard_status.clone()} />
                <Scoreboard word={self.word.clone()} guesses={self.guesses.clone()} max_guesses={self.max_guesses} game_state={self.game_state.clone()} />
            </div>
        }
    }
}

fn evaluate_guess(word: &str, guess: &str) -> Vec<CellValue> {
    let mut vals = Vec::with_capacity(word.len());
    let mut counts = word
        .chars()
        .fold(HashMap::new(), |mut acc: HashMap<char, usize>, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

    // find correct characters
    for (w, g) in word.chars().zip(guess.chars()) {
        let cell = if w == g {
            if let Some(count) = counts.get_mut(&g) {
                *count = count.saturating_sub(1);
            }
            Some(CellValue::Correct(g))
        } else {
            None
        };
        vals.push(cell);
    }

    // categorize the rest of the characters
    for (idx, g) in guess.chars().enumerate() {
        let cell = match (vals[idx], counts.get(&g)) {
            (v @ Some(_), _) => v,
            (None, Some(f)) if *f > 0 => {
                if let Some(count) = counts.get_mut(&g) {
                    *count = count.saturating_sub(1);
                }
                Some(CellValue::Present(g))
            }
            (_, _) => Some(CellValue::Absent(g)),
        };
        vals[idx] = cell;
    }

    vals.into_iter().map(Option::unwrap).collect()
}

#[allow(clippy::needless_pass_by_value)]
fn handle_keypress(e: KeyboardEvent) -> Option<PaudleMsg> {
    if e.key() == BACKSPACE {
        return Some(PaudleMsg::Backspace);
    }
    if e.key() == ENTER {
        return Some(PaudleMsg::Submit);
    }
    if e.key().len() > 1 {
        return None;
    }
    if e.ctrl_key() || e.alt_key() || e.meta_key() || e.shift_key() {
        return None;
    }
    if let Some(c) = e.key().chars().next() {
        if c.is_alphabetic() {
            Some(PaudleMsg::TypeLetter(c))
        } else {
            None
        }
    } else {
        None
    }
}

fn main() {
    yew::start_app::<Paudle>();
}

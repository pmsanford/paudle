#![allow(clippy::module_name_repetitions)]
mod board;
mod keyboard;
mod scoreboard;

use rand::{prelude::IteratorRandom, thread_rng};
use std::collections::HashMap;
#[allow(unused_imports)]
use web_sys::console;
use yew::prelude::*;

use board::{Board, CellValue};
use keyboard::{Keyboard, KeyboardStatus};
use scoreboard::Scoreboard;

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

impl Component for Paudle {
    type Message = PaudleMsg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let word_choices = WORD_LIST.lines();
        let word = word_choices.choose(&mut thread_rng()).unwrap().to_string();
        Self {
            word,
            guesses: vec![],
            current_guess: String::new(),
            keyboard_status: KeyboardStatus::new(),
            word_length: 5,
            max_guesses: 6,
            bad_guess: false,
            game_state: GameState::InProgress,
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
                    let new_guess = evaluate_guess(&self.word, &self.current_guess.to_lowercase());
                    self.keyboard_status.update_status(&new_guess);
                    let correct = new_guess.iter().all(|g| matches!(g, CellValue::Correct(_)));
                    self.guesses.push(new_guess);
                    self.current_guess = String::new();
                    if correct {
                        self.game_state = GameState::Won;
                    } else if self.guesses.len() == self.max_guesses {
                        self.game_state = GameState::Lost;
                    }
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let on_keypress = link.batch_callback(handle_keypress);

        let cb = ctx.link().callback(|msg: PaudleMsg| msg);

        // tabIndex=0 for keyboard events: https://stackoverflow.com/questions/43503964/onkeydown-event-not-working-on-divs-in-react/44434971#44434971
        html! {
            <div id="outer_container" tabIndex=0 onkeyup={on_keypress} class="page">
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
        let cell = match (g, w == g) {
            (g, true) => {
                if let Some(count) = counts.get_mut(&g) {
                    *count = count.saturating_sub(1);
                }
                Some(CellValue::Correct(g))
            }
            _ => None,
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
    if &e.key() == "Backspace" {
        return Some(PaudleMsg::Backspace);
    }
    if &e.key() == "Enter" {
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

#![allow(clippy::module_name_repetitions)]
mod board;
mod keyboard;
mod save;
mod scoreboard;

use gloo_events::EventListener;
use patternfly_yew::{Backdrop, BackdropDispatcher, Bullseye, Modal, ModalVariant};
use patternfly_yew::{BackdropViewer, Toast, ToastDispatcher, ToastViewer, Type};
use rand::SeedableRng;
use rand::{prelude::IteratorRandom, thread_rng};
use save::update_saved_state;
use save::{load_game_history, load_saved_sate};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::{collections::HashMap, mem};
use wasm_bindgen::JsCast;
use web_sys::window;
use yew::prelude::*;

use board::{Board, CellValue};
use keyboard::{Keyboard, KeyboardStatus, BACKSPACE, ENTER, ESCAPE};
use scoreboard::{Scoreboard, ScoreboardFooter};

const WORD_LIST: &str = include_str!("awords.txt");
pub struct Paudle {
    word: String,
    guesses: Vec<Vec<CellValue>>,
    keyboard_status: KeyboardStatus,
    current_guess: String,
    word_length: usize,
    max_guesses: usize,
    game_state: GameState,
    game_mode: GameMode,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GameMode {
    Daily(i64),
    Random,
}

fn get_todays_key() -> i64 {
    chrono::Local::now().date().and_hms(0, 0, 0).timestamp()
}

impl Default for Paudle {
    fn default() -> Self {
        let word_choices = WORD_LIST.lines();
        let ts = get_todays_key();
        #[allow(clippy::cast_sign_loss)]
        let mut rng = rand::prelude::StdRng::seed_from_u64(ts as u64);
        let word = word_choices.choose(&mut rng).unwrap().to_string();
        Self::with_word(word, GameMode::Daily(ts))
    }
}

pub enum PaudleMsg {
    TypeLetter(char),
    Backspace,
    Submit,
    StartRandom,
    Escape,
}

#[derive(PartialEq, Clone)]
pub enum GameState {
    InProgress,
    Won,
    Lost,
}

impl Paudle {
    fn with_word(word: String, game_mode: GameMode) -> Self {
        Self {
            word,
            guesses: Vec::new(),
            keyboard_status: KeyboardStatus::default(),
            current_guess: String::new(),
            word_length: 5,
            max_guesses: 6,
            game_state: GameState::InProgress,
            game_mode,
        }
    }

    fn random() -> Self {
        let word_choices = WORD_LIST.lines();
        let mut rng = thread_rng();
        let word = word_choices.choose(&mut rng).unwrap().to_string();
        Self::with_word(word, GameMode::Random)
    }

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

    fn show_scoreboard(&mut self, ctx: &Context<Self>) {
        let clear = ctx.link().callback(|msg: PaudleMsg| msg);
        let title = if self.game_state == GameState::Won {
            "Winner!".to_string()
        } else {
            format!(r#"Game Over. Word was "{}""#, self.word)
        };
        let bd = Backdrop {
            content: html! {
                <Bullseye>
                    <Modal
                        title={title}
                        variant={ModalVariant::Small}
                        footer={Some(html!{<ScoreboardFooter
                                                guesses={self.guesses.clone()}
                                                won={self.game_state == GameState::Won}
                                                max_guesses={self.max_guesses}
                                                random={self.game_mode == GameMode::Random}
                                                clear={clear}
                                            />})}
                    >
                        <Scoreboard />
                    </Modal>
                </Bullseye>
            },
        };
        BackdropDispatcher::default().open(bd);
    }
}

impl Component for Paudle {
    type Message = PaudleMsg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let saved_state = load_saved_sate();
        if let Some(saved_state) = saved_state {
            saved_state.into()
        } else {
            let history = load_game_history();
            history
                .scores
                .get(&get_todays_key())
                .cloned()
                .map_or_else(Paudle::default, Into::into)
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match (self.game_state == GameState::InProgress, msg) {
            (true, PaudleMsg::TypeLetter(c)) if self.current_guess.len() < self.word_length => {
                self.current_guess.push(c.to_ascii_lowercase());
                true
            }
            (true, PaudleMsg::Backspace) => {
                self.current_guess.pop();
                true
            }
            (true, PaudleMsg::Submit) => {
                if self.current_guess.len() == self.word_length {
                    if !WORD_LIST.contains(&self.current_guess) {
                        ToastDispatcher::new().toast(Toast {
                            title: "Word not in word list".into(),
                            r#type: Type::Danger,
                            timeout: Some(Duration::from_secs(2)),
                            ..Toast::default()
                        });
                        return true;
                    }
                    let current_guess = mem::take(&mut self.current_guess);
                    self.eval_and_add_guess(&current_guess);
                    update_saved_state(self);
                    if self.game_state != GameState::InProgress {
                        self.show_scoreboard(ctx);
                    }
                    true
                } else {
                    false
                }
            }
            (false, PaudleMsg::StartRandom) => {
                let mut new_game = Paudle::random();
                mem::swap(self, &mut new_game);
                true
            }
            (_, PaudleMsg::Escape) => {
                BackdropDispatcher::default().close();
                true
            }
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cb = ctx.link().callback(|msg: PaudleMsg| msg);

        html! {
            <div class="page">
                <Board
                    current_guess={self.current_guess.clone()}
                    guesses={self.guesses.clone()}
                    row_count={self.max_guesses}
                    word_length={self.word_length}
                />
                <Keyboard key_press={cb} keys={self.keyboard_status.clone()} />
                <BackdropViewer />
                <ToastViewer />
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }

        if self.game_state != GameState::InProgress {
            self.show_scoreboard(ctx);
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
    if e.key() == ESCAPE {
        return Some(PaudleMsg::Escape);
    }
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

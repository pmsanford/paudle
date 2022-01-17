use keyboard::{Keyboard, KeyboardStatus};
use rand::{prelude::IteratorRandom, thread_rng};
use std::collections::HashMap;
#[allow(unused_imports)]
use web_sys::console;

mod keyboard;

use yew::{html::ImplicitClone, prelude::*};

const WORD_LIST: &str = include_str!("awords.txt");

#[derive(PartialEq, Clone, Copy)]
pub enum CellValue {
    Empty,
    Typing(char),
    Absent(char),
    Present(char),
    Correct(char),
}

impl ImplicitClone for CellValue {}

#[derive(Properties, PartialEq, Clone)]
pub struct CellProps {
    value: CellValue,
}

#[function_component(Cell)]
pub fn cell(props: &CellProps) -> Html {
    match props.value {
        CellValue::Empty => {
            html! {
                <div data-status="empty" class="tile" />
            }
        }
        CellValue::Typing(v) => {
            html! {
                <div data-status="empty" class="tile">{v}</div>
            }
        }
        CellValue::Absent(v) => {
            html! {
                <div data-status="absent" class="tile">{v}</div>
            }
        }
        CellValue::Present(v) => {
            html! {
                <div data-status="present" class="tile">{v}</div>
            }
        }
        CellValue::Correct(v) => {
            html! {
                <div data-status="correct" class="tile">{v}</div>
            }
        }
    }
}

#[derive(Properties, PartialEq)]
struct RowProps {
    values: Vec<CellValue>,
    wrong: bool,
}

struct PaudleRow;

impl Component for PaudleRow {
    type Message = ();

    type Properties = RowProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = vec!["row"];
        if ctx.props().wrong {
            classes.push("wrong");
        }
        html! {
            <div class={classes!(classes)}>
                { ctx.props().values.clone().iter().map(|c| html! { <Cell value={c} /> }).collect::<Html>() }
            </div>
        }
    }
}
fn create_row_props(word: &str, guess: &str) -> Vec<CellValue> {
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
struct Paudle {
    word: String,
    guesses: Vec<Vec<CellValue>>,
    keyboard_status: KeyboardStatus,
    current_guess: String,
    word_length: usize,
    max_guesses: usize,
    bad_guess: bool,
}

pub enum PaudleMsg {
    TypeLetter(char),
    Backspace,
    Submit,
}

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
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
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
                if self.current_guess.len() == self.word_length
                    && self.guesses.len() < self.max_guesses
                {
                    if !WORD_LIST.contains(&self.current_guess) {
                        self.bad_guess = true;
                        return true;
                    }
                    let new_guess =
                        create_row_props(&self.word, &self.current_guess.to_lowercase());
                    self.keyboard_status.update_status(&new_guess);
                    let correct = new_guess.iter().all(|g| matches!(g, CellValue::Correct(_)));
                    self.guesses.push(new_guess);
                    self.current_guess = String::new();
                    if self.guesses.len() == self.max_guesses || correct {
                        console::log_1(&format!("Word: {}", self.word).into());
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
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
struct BoardProps {
    current_guess: String,
    guesses: Vec<Vec<CellValue>>,
    row_count: usize,
    word_length: usize,
    bad_guess: bool,
}

#[function_component(Board)]
fn view(props: &BoardProps) -> Html {
    let mut filled_rows = props.guesses.clone();
    let mut rows = vec![vec![CellValue::Empty; props.word_length]; props.row_count];
    if filled_rows.len() < rows.len() {
        let mut guess_row = vec![CellValue::Typing(' '); props.word_length];
        for (idx, c) in props.current_guess.chars().enumerate() {
            guess_row[idx] = CellValue::Typing(c);
        }
        filled_rows.push(guess_row);
    }
    for (i, val) in filled_rows.into_iter().enumerate() {
        rows[i] = val;
    }
    html! {
            <div class="wrapper">
                <div class="game">
                    {
                        rows.into_iter()
                            .enumerate()
                            .map(|(idx, r)| {
                                let wrong = idx == props.guesses.len() && props.bad_guess;
                                html! { <PaudleRow wrong={wrong} values={r} /> }
                            }).collect::<Html>()
                    }
                </div>
            </div>
    }
}

fn main() {
    yew::start_app::<Paudle>();
}

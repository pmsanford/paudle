use rand::{prelude::IteratorRandom, thread_rng};
use std::collections::HashMap;
#[allow(unused_imports)]
use web_sys::console;

use yew::{
    html::{ImplicitClone, IntoPropValue},
    prelude::*,
};

const WORD_LIST: &'static str = include_str!("awords.txt");

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

#[derive(Properties, PartialEq)]
pub struct KeyProps {
    def: KeyDef,
}

impl IntoPropValue<KeyDef> for KeyStatus {
    fn into_prop_value(self) -> KeyDef {
        KeyDef::Letter(self)
    }
}

#[function_component(Key)]
pub fn key(props: &KeyProps) -> Html {
    html! {
      <div data-status={props.status_string()} class={props.class()}>
        {props.disp()}
      </div>
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Status {
    Unused,
    Absent,
    Present,
    Correct,
}

impl KeyProps {
    fn status_string(&self) -> String {
        match &self.def {
            KeyDef::Letter(letter) => match letter.status {
                Status::Unused => "unused",
                Status::Absent => "absent",
                Status::Present => "present",
                Status::Correct => "correct",
            },
            KeyDef::Enter => "unused",
            KeyDef::Backspace => "unused",
        }
        .to_string()
    }

    fn class(&self) -> Classes {
        let mut classes = classes!("key");
        if matches!(self.def, KeyDef::Enter | KeyDef::Backspace) {
            classes.push("special-key");
        }
        classes
    }

    fn disp(&self) -> String {
        match &self.def {
            KeyDef::Letter(l) => l.letter.to_string(),
            KeyDef::Enter => "ENTER".to_string(),
            KeyDef::Backspace => "DEL".to_string(),
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct KeyStatus {
    status: Status,
    letter: char,
}

#[derive(PartialEq, Clone)]
pub enum KeyDef {
    Letter(KeyStatus),
    Enter,
    Backspace,
}

impl ImplicitClone for KeyDef {}

struct Keyboard;

#[derive(Properties, PartialEq)]
struct KeyboardProperties {
    keys: KeyboardStatus,
}

impl Component for Keyboard {
    type Message = ();

    type Properties = KeyboardProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div class="wrapper">
          <div class="keyboard">
            <div class="keyboard-row">
              <Key def={ctx.props().keys.get_status('Q')} />
              <Key def={ctx.props().keys.get_status('W')} />
              <Key def={ctx.props().keys.get_status('E')} />
              <Key def={ctx.props().keys.get_status('R')} />
              <Key def={ctx.props().keys.get_status('T')} />
              <Key def={ctx.props().keys.get_status('Y')} />
              <Key def={ctx.props().keys.get_status('U')} />
              <Key def={ctx.props().keys.get_status('I')} />
              <Key def={ctx.props().keys.get_status('O')} />
              <Key def={ctx.props().keys.get_status('P')} />
            </div>
            <div class="keyboard-row">
              <Key def={ctx.props().keys.get_status('A')} />
              <Key def={ctx.props().keys.get_status('S')} />
              <Key def={ctx.props().keys.get_status('D')} />
              <Key def={ctx.props().keys.get_status('F')} />
              <Key def={ctx.props().keys.get_status('G')} />
              <Key def={ctx.props().keys.get_status('H')} />
              <Key def={ctx.props().keys.get_status('J')} />
              <Key def={ctx.props().keys.get_status('K')} />
              <Key def={ctx.props().keys.get_status('L')} />
            </div>
            <div class="keyboard-row">
              <Key def={KeyDef::Enter} />
              <Key def={ctx.props().keys.get_status('Z')} />
              <Key def={ctx.props().keys.get_status('X')} />
              <Key def={ctx.props().keys.get_status('C')} />
              <Key def={ctx.props().keys.get_status('V')} />
              <Key def={ctx.props().keys.get_status('B')} />
              <Key def={ctx.props().keys.get_status('N')} />
              <Key def={ctx.props().keys.get_status('M')} />
              <Key def={KeyDef::Backspace} />
            </div>
          </div>
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

#[derive(PartialEq, Clone)]
pub struct KeyboardStatus {
    keys: HashMap<char, Status>,
}

impl KeyboardStatus {
    fn new() -> Self {
        Self {
            keys: HashMap::new(),
        }
    }

    fn get_status(&self, letter: char) -> KeyStatus {
        let status = self
            .keys
            .get(&letter.to_ascii_lowercase())
            .cloned()
            .unwrap_or_else(|| Status::Unused);
        KeyStatus { letter, status }
    }

    fn update_status(&mut self, guess: &Vec<CellValue>) {
        for cell in guess {
            match cell {
                CellValue::Absent(c) => {
                    self.keys.insert(*c, Status::Absent);
                }
                CellValue::Present(c) => {
                    self.keys.insert(*c, Status::Present);
                }
                CellValue::Correct(c) => {
                    self.keys.insert(*c, Status::Correct);
                }
                _ => {}
            }
        }
    }
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

enum PaudleMsg {
    TypeLetter(char),
    Backspace,
    Submit,
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
                self.current_guess.push(c);
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
        let guesses = self.guesses.clone();
        let mut rows = vec![vec![CellValue::Empty; self.word_length]; self.max_guesses];
        let mut filled_rows = guesses.iter().cloned().collect::<Vec<_>>();
        if filled_rows.len() < self.max_guesses {
            let mut guess_row = vec![CellValue::Typing(' '); self.word_length];
            for (idx, c) in self.current_guess.chars().enumerate() {
                guess_row[idx] = CellValue::Typing(c);
            }
            filled_rows.push(guess_row);
        }
        for (i, val) in filled_rows.into_iter().enumerate() {
            rows[i] = val;
        }

        let link = ctx.link();

        let on_keypress = link.batch_callback(|e: KeyboardEvent| {
            if &e.key() == "Backspace" {
                return Some(PaudleMsg::Backspace);
            }
            if &e.key() == "Enter" {
                return Some(PaudleMsg::Submit);
            }
            if e.key().len() > 1 {
                return None;
            }
            if e.ctrl_key() || e.alt_key() || e.meta_key() {
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
        });

        // tabIndex=0 for keyboard events: https://stackoverflow.com/questions/43503964/onkeydown-event-not-working-on-divs-in-react/44434971#44434971
        html! {
            <div tabIndex=0 onkeyup={on_keypress} class="page">
                <div class="wrapper">
                    <div class="game">
                        {
                            rows.into_iter()
                                .enumerate()
                                .map(|(idx, r)| {
                                    let wrong = idx == self.guesses.len() && self.bad_guess;
                                    html! { <PaudleRow wrong={wrong} values={r} /> }
                                }).collect::<Html>()
                        }
                    </div>
                </div>
                <Keyboard keys={self.keyboard_status.clone()} />
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Paudle>();
}

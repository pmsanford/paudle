use std::collections::HashMap;

use yew::{html::ImplicitClone, prelude::*};

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
}

struct PaudleRow;

impl Component for PaudleRow {
    type Message = ();

    type Properties = RowProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="row">
                { ctx.props().values.clone().iter().map(|c| html! { <Cell value={c} /> }).collect::<Html>() }
            </div>
        }
    }
}

struct Paudle {
    word: String,
    guesses: Vec<String>,
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

impl Component for Paudle {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let guesses = vec![
            "poops".to_string(),
            "pepis".to_string(),
            "bepos".to_string(),
            "ppsps".to_string(),
        ];
        Self {
            word: "poops".to_string(),
            guesses,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let guesses = self.guesses.clone();
        let filled_rows = guesses
            .iter()
            .map(|guess| create_row_props(&self.word, guess));
        let mut rows = vec![vec![CellValue::Empty; 5]; 6];
        for (i, val) in filled_rows.enumerate() {
            rows[i] = val;
        }
        html! {
            <div class="wrapper">
                <div class="game">
                    {rows.into_iter().map(|r| html! { <PaudleRow values={r} /> }).collect::<Html>()}
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Paudle>();
}

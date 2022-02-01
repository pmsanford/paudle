use web_sys::window;
use yew::prelude::*;

use crate::{board::CellValue, GameState};

#[derive(Properties, PartialEq)]
pub struct ScoreboardProps {
    pub word: String,
    pub guesses: Vec<Vec<CellValue>>,
    pub max_guesses: usize,
    pub game_state: GameState,
}

#[derive(Properties, PartialEq)]
pub struct ScoreRowProps {
    pub guess: Vec<CellValue>,
}

#[function_component(ScoreRow)]
pub fn score_row(props: &ScoreRowProps) -> Html {
    html! { <>{ props.guess.iter().copied().map(CellValue::score_char).collect::<String>() }<br /></> }
}

#[function_component(Scoreboard)]
pub fn scoreboard(props: &ScoreboardProps) -> Html {
    let guesses = if props.game_state == GameState::Won {
        props.guesses.len().to_string()
    } else {
        "X".to_string()
    };
    html! {
        <div class="scoreboard">{"Word: "}{&props.word}<br />
            {"Paudle "}{guesses}{"/"}{props.max_guesses}<br /><br />
            { props.guesses.iter().map(|g| html! { <ScoreRow guess={g.clone()} /> }).collect::<Html>() }
            </div>
    }
}

fn generate_unicode_block(guesses: &[Vec<CellValue>]) -> String {
    guesses
        .iter()
        .map(|g| {
            g.iter()
                .copied()
                .map(CellValue::score_char)
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn generate_score_copy(won: bool, max_guesses: usize, guesses: &[Vec<CellValue>]) -> String {
    format!(
        "Paudle {}/{}\n{}",
        if won {
            guesses.len().to_string()
        } else {
            "X".to_string()
        },
        max_guesses,
        generate_unicode_block(guesses)
    )
}

#[derive(Properties, PartialEq)]
pub struct ScoreboardFooterProps {
    pub guesses: Vec<Vec<CellValue>>,
    pub won: bool,
    pub max_guesses: usize,
}

#[function_component(ScoreboardFooter)]
pub fn scoreboard_footer(props: &ScoreboardFooterProps) -> Html {
    let guesses = props.guesses.clone();
    let won = props.won;
    let max_guesses = props.max_guesses;
    let label = use_state(|| "Share score".to_string());
    let cblabel = label.clone();
    let cb = Callback::from(move |_: MouseEvent| {
        let clipboard = window().unwrap().navigator().clipboard().unwrap();
        let boxes = generate_score_copy(won, max_guesses, &guesses);
        #[allow(clippy::let_underscore_drop)]
        let _ = clipboard.write_text(&boxes);
        cblabel.set("Copied!".to_string());
    });
    html! {
        <div class="share-score" onclick={cb}>{&*label}</div>
    }
}

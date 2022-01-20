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
    match props.game_state {
        GameState::InProgress => html! { <></> },
        GameState::Won => {
            html! {
                <div class="scoreboard winner">{"Winner: "}<br /><br />{"Paudle "}{props.guesses.len()}{"/"}{props.max_guesses}<br /><br />
                    { props.guesses.iter().map(|g| html! { <ScoreRow guess={g.clone()} /> }).collect::<Html>() }
                    </div>
            }
        }
        GameState::Lost => {
            html! {
                <div class="scoreboard">{"LOSER: The word was "}{props.word.clone()}</div>
            }
        }
    }
}

use yew::prelude::*;

use crate::{board::CellValue, GameState};

#[derive(Properties, PartialEq)]
pub struct ScoreboardProps {
    pub word: String,
    pub guesses: Vec<Vec<CellValue>>,
    pub max_guesses: usize,
    pub game_state: GameState,
}

#[function_component(Scoreboard)]
pub fn scoreboard(props: &ScoreboardProps) -> Html {
    match props.game_state {
        GameState::InProgress => html! { <></> },
        GameState::Won => {
            html! {
                <div class="scoreboard">{"WINNER: "}{props.guesses.len()}{"/"}{props.max_guesses}</div>
            }
        }
        GameState::Lost => {
            html! {
                <div class="scoreboard">{"LOSER: The word was "}{props.word.clone()}</div>
            }
        }
    }
}

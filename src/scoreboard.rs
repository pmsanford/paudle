#![allow(
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation
)]
use std::collections::HashMap;

use patternfly_yew::BackdropDispatcher;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use yew::prelude::*;

use crate::{board::CellValue, save::load_game_history, GameState, PaudleMsg};

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

#[derive(Properties, PartialEq)]
pub struct DistributionBarProps {
    proportion: usize,
    num: usize,
    count: usize,
}

#[function_component(DistributionBar)]
pub fn distribution_bar(props: &DistributionBarProps) -> Html {
    html! { <div class="distribution-row"><div class="distribution-row-num">{props.num}{": "}</div><div class="distribution-row-bar"><div class="distribution-bar" style={format!("width: calc(18px + {} * 0.9%)", props.proportion)}>{props.count}</div></div></div> }
}

#[derive(Properties, PartialEq)]
pub struct ScoreboardStatProps {
    stat: String,
    caption: String,
}

#[function_component(ScoreboardStat)]
pub fn scoreboard_stat(props: &ScoreboardStatProps) -> Html {
    html! {
        <div class="scoreboard-stat">
            <div>{&props.stat}</div>
            <div>{&props.caption}</div>
        </div>
    }
}

#[function_component(Scoreboard)]
pub fn scoreboard(_props: &()) -> Html {
    let history = load_game_history();
    let total_games = history.scores.len();
    let mut distribution: HashMap<usize, usize> = (1..=6).map(|num| (num, 0)).collect();
    let winning_games = history.scores.values().filter(|val| val.was_won());
    winning_games.for_each(|val| {
        distribution
            .entry(val.guesses.len())
            .and_modify(|v| *v += 1);
    });
    let win_count = history.wins();
    let max_wins = distribution.values().max().copied().unwrap();
    let ratio = 100. / (max_wins as f32 / win_count as f32);
    let proportion = |count: usize| ((count as f32 / win_count as f32) * ratio) as usize;
    let mut bars = distribution.iter().map(|(num, count)| (num, html! { <DistributionBar num={*num} count={*count} proportion={proportion(*count)} /> })).collect::<Vec<_>>();
    bars.sort_unstable_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
    html! {
        <div class="scoreboard">
            <div class="scoreboard-header">{"Statistics"}</div>
            <ScoreboardStat stat={total_games.to_string()} caption={"Played"} />
            <ScoreboardStat stat={(((win_count as f32 / total_games as f32) * 100.) as usize).to_string()} caption={"Win %"} />
            <ScoreboardStat stat={history.current_streak().to_string()} caption={"Current Streak"} />
            <ScoreboardStat stat={history.max_streak().to_string()} caption={"Max Streak"} />
            <div class="scoreboard-header">{"Guess Distribution"}</div>
            <div class="scoreboard-distribution">
                {bars.into_iter().map(|(_, b)| b).collect::<Vec<_>>()}
            </div>
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

fn generate_score_copy(
    won: bool,
    max_guesses: usize,
    guesses: &[Vec<CellValue>],
    modifiers: &str,
) -> String {
    format!(
        "Paudle {}/{}{}\n\n{}",
        if won {
            guesses.len().to_string()
        } else {
            "X".to_string()
        },
        max_guesses,
        modifiers,
        generate_unicode_block(guesses)
    )
}

#[derive(Properties, PartialEq)]
pub struct ScoreboardFooterProps {
    pub guesses: Vec<Vec<CellValue>>,
    pub won: bool,
    pub max_guesses: usize,
    pub clear: Callback<PaudleMsg>,
    pub random: bool,
}

#[function_component(ScoreboardFooter)]
pub fn scoreboard_footer(props: &ScoreboardFooterProps) -> Html {
    let guesses = props.guesses.clone();
    let won = props.won;
    let max_guesses = props.max_guesses;
    let random = props.random;
    let label = use_state(|| "Share score".to_string());
    let cblabel = label.clone();
    let cb = Callback::from(move |_: MouseEvent| {
        let modifiers = if random {
            "r".to_string()
        } else {
            String::new()
        };
        let boxes = generate_score_copy(won, max_guesses, &guesses, &modifiers);
        wasm_bindgen_futures::spawn_local(async move {
            copy_to_clipboard(boxes).await.unwrap();
        });
        cblabel.set("Copied!".to_string());
    });
    let clear = props.clear.clone();
    let ccb = Callback::from(move |_: MouseEvent| {
        clear.emit(PaudleMsg::StartRandom);
        BackdropDispatcher::default().close();
    });
    html! {
        <div class="share-score"><span onclick={cb}>{&*label}</span><span class="play-button" onclick={ccb}>{"Play random"}</span></div>
    }
}

#[wasm_bindgen(inline_js=r#"
export function copy_to_clipboard(value) {
    try {
        return window.navigator.clipboard.writeText(value);
    } catch(e) {
        console.log(e);
        return Promise.reject(e)
    }

}
"#)]
#[rustfmt::skip] // required to keep the "async" keyword
extern "C" { 
    #[wasm_bindgen(catch)]
    async fn copy_to_clipboard(value: String) -> Result<(), JsValue>;
}

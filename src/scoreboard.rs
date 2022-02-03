use patternfly_yew::BackdropDispatcher;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use yew::prelude::*;

use crate::{board::CellValue, GameState, PaudleMsg};

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
    pub clear: Callback<PaudleMsg>,
}

#[function_component(ScoreboardFooter)]
pub fn scoreboard_footer(props: &ScoreboardFooterProps) -> Html {
    let guesses = props.guesses.clone();
    let won = props.won;
    let max_guesses = props.max_guesses;
    let label = use_state(|| "Share score".to_string());
    let cblabel = label.clone();
    let cb = Callback::from(move |_: MouseEvent| {
        let boxes = generate_score_copy(won, max_guesses, &guesses);
        wasm_bindgen_futures::spawn_local(async move {
            copy_to_clipboard(boxes).await.unwrap();
        });
        cblabel.set("Copied!".to_string());
    });
    let clear = props.clear.clone();
    let ccb = Callback::from(move |_: MouseEvent| {
        clear.emit(PaudleMsg::Clear);
        BackdropDispatcher::default().close();
    });
    html! {
        <div class="share-score"><span onclick={cb}>{&*label}</span><span class="play-button" onclick={ccb}>{"Play again"}</span></div>
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

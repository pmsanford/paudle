use super::row::Row;
use yew::prelude::*;

use super::cell::CellValue;

#[derive(Properties, PartialEq)]
pub struct BoardProps {
    pub current_guess: String,
    pub guesses: Vec<Vec<CellValue>>,
    pub row_count: usize,
    pub word_length: usize,
}

#[function_component(Board)]
pub fn view(props: &BoardProps) -> Html {
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
                            .map(|r| {
                                html! { <Row values={r} /> }
                            }).collect::<Html>()
                    }
                </div>
            </div>
    }
}

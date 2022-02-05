use serde::{Deserialize, Serialize};
use yew::{html::ImplicitClone, prelude::*};

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum CellValue {
    Empty,
    Typing(char),
    Absent(char),
    Present(char),
    Correct(char),
}

impl CellValue {
    pub fn score_char(self) -> char {
        match self {
            Self::Empty | Self::Typing(_) | Self::Absent(_) => '⬜',
            Self::Present(_) => '🟨',
            Self::Correct(_) => '🟩',
        }
    }
}

impl ImplicitClone for CellValue {}

#[derive(Properties, PartialEq, Clone)]
pub struct CellProps {
    pub value: CellValue,
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

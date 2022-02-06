use yew::prelude::*;

use super::cell::{Cell, CellValue};

#[derive(Properties, PartialEq)]
pub struct RowProps {
    pub values: Vec<CellValue>,
}

pub struct Row;

impl Component for Row {
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

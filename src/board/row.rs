use yew::prelude::*;

use super::cell::{Cell, CellValue};

#[allow(clippy::module_name_repetitions)]
#[derive(Properties, PartialEq)]
pub struct RowProps {
    pub values: Vec<CellValue>,
    pub wrong: bool,
}

pub struct Row;

impl Component for Row {
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

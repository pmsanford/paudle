use crate::PaudleMsg;

use super::key::{Key, KeyType};
use super::keyboard_status::KeyboardStatus;
use yew::prelude::*;

pub struct Keyboard;

#[derive(Properties, PartialEq)]
pub struct KeyboardProperties {
    pub keys: KeyboardStatus,
    pub key_press: Callback<PaudleMsg>,
}

impl Component for Keyboard {
    type Message = ();

    type Properties = KeyboardProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let kp = &ctx.props().key_press;

        let key = |c: char| {
            html! { <Key key_press={kp.clone()} def={ctx.props().keys.get_status(c)} /> }
        };

        let row_one = ['Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P']
            .into_iter()
            .map(key)
            .collect::<Vec<_>>();
        let row_two = ['A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L']
            .into_iter()
            .map(key)
            .collect::<Vec<_>>();
        let row_three = ['Z', 'X', 'C', 'V', 'B', 'N', 'M']
            .into_iter()
            .map(key)
            .collect::<Vec<_>>();

        html! {
        <div class="wrapper">
          <div class="keyboard">
            <div class="keyboard-row">
              {row_one}
            </div>
            <div class="keyboard-row">
              {row_two}
            </div>
            <div class="keyboard-row">
              <Key key_press={kp.clone()} def={KeyType::Enter} />
              {row_three}
              <Key key_press={kp.clone()} def={KeyType::Backspace} />
            </div>
          </div>
        </div>
            }
    }
}

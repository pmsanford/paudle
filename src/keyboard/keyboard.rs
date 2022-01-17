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
        html! {
        <div class="wrapper">
          <div class="keyboard">
            <div class="keyboard-row">
              <Key key_press={ctx.props().key_press.clone()} def={ctx.props().keys.get_status('Q')} />
              <Key key_press={ctx.props().key_press.clone()} def={ctx.props().keys.get_status('W')} />
              <Key key_press={ctx.props().key_press.clone()} def={ctx.props().keys.get_status('E')} />
              <Key key_press={ctx.props().key_press.clone()} def={ctx.props().keys.get_status('R')} />
              <Key key_press={ctx.props().key_press.clone()} def={ctx.props().keys.get_status('T')} />
              <Key key_press={ctx.props().key_press.clone()} def={ctx.props().keys.get_status('Y')} />
              <Key key_press={ctx.props().key_press.clone()} def={ctx.props().keys.get_status('U')} />
              <Key key_press={ctx.props().key_press.clone()} def={ctx.props().keys.get_status('I')} />
              <Key key_press={ctx.props().key_press.clone()} def={ctx.props().keys.get_status('O')} />
              <Key key_press={ctx.props().key_press.clone()} def={ctx.props().keys.get_status('P')} />
            </div>
            <div class="keyboard-row">
              <Key key_press={ctx.props().key_press.clone()} def={ctx.props().keys.get_status('A')} />
              <Key key_press={ctx.props().key_press.clone()} def={ctx.props().keys.get_status('S')} />
              <Key key_press={ctx.props().key_press.clone()} def={ctx.props().keys.get_status('D')} />
              <Key key_press={ctx.props().key_press.clone()} def={ctx.props().keys.get_status('F')} />
              <Key key_press={ctx.props().key_press.clone()} def={ctx.props().keys.get_status('G')} />
              <Key key_press={ctx.props().key_press.clone()} def={ctx.props().keys.get_status('H')} />
              <Key key_press={ctx.props().key_press.clone()} def={ctx.props().keys.get_status('J')} />
              <Key key_press={ctx.props().key_press.clone()} def={ctx.props().keys.get_status('K')} />
              <Key key_press={ctx.props().key_press.clone()} def={ctx.props().keys.get_status('L')} />
            </div>
            <div class="keyboard-row">
              <Key key_press={ctx.props().key_press.clone()} def={KeyType::Enter} />
              <Key key_press={ctx.props().key_press.clone()} def={ctx.props().keys.get_status('Z')} />
              <Key key_press={ctx.props().key_press.clone()} def={ctx.props().keys.get_status('X')} />
              <Key key_press={ctx.props().key_press.clone()} def={ctx.props().keys.get_status('C')} />
              <Key key_press={ctx.props().key_press.clone()} def={ctx.props().keys.get_status('V')} />
              <Key key_press={ctx.props().key_press.clone()} def={ctx.props().keys.get_status('B')} />
              <Key key_press={ctx.props().key_press.clone()} def={ctx.props().keys.get_status('N')} />
              <Key key_press={ctx.props().key_press.clone()} def={ctx.props().keys.get_status('M')} />
              <Key key_press={ctx.props().key_press.clone()} def={KeyType::Backspace} />
            </div>
          </div>
        </div>
            }
    }
}

use super::key::{Key, KeyType};
use super::keyboard_status::KeyboardStatus;
use yew::prelude::*;

pub struct Keyboard;

#[derive(Properties, PartialEq)]
pub struct KeyboardProperties {
    pub keys: KeyboardStatus,
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
              <Key def={ctx.props().keys.get_status('Q')} />
              <Key def={ctx.props().keys.get_status('W')} />
              <Key def={ctx.props().keys.get_status('E')} />
              <Key def={ctx.props().keys.get_status('R')} />
              <Key def={ctx.props().keys.get_status('T')} />
              <Key def={ctx.props().keys.get_status('Y')} />
              <Key def={ctx.props().keys.get_status('U')} />
              <Key def={ctx.props().keys.get_status('I')} />
              <Key def={ctx.props().keys.get_status('O')} />
              <Key def={ctx.props().keys.get_status('P')} />
            </div>
            <div class="keyboard-row">
              <Key def={ctx.props().keys.get_status('A')} />
              <Key def={ctx.props().keys.get_status('S')} />
              <Key def={ctx.props().keys.get_status('D')} />
              <Key def={ctx.props().keys.get_status('F')} />
              <Key def={ctx.props().keys.get_status('G')} />
              <Key def={ctx.props().keys.get_status('H')} />
              <Key def={ctx.props().keys.get_status('J')} />
              <Key def={ctx.props().keys.get_status('K')} />
              <Key def={ctx.props().keys.get_status('L')} />
            </div>
            <div class="keyboard-row">
              <Key def={KeyType::Enter} />
              <Key def={ctx.props().keys.get_status('Z')} />
              <Key def={ctx.props().keys.get_status('X')} />
              <Key def={ctx.props().keys.get_status('C')} />
              <Key def={ctx.props().keys.get_status('V')} />
              <Key def={ctx.props().keys.get_status('B')} />
              <Key def={ctx.props().keys.get_status('N')} />
              <Key def={ctx.props().keys.get_status('M')} />
              <Key def={KeyType::Backspace} />
            </div>
          </div>
        </div>
            }
    }
}

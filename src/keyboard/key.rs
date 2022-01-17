use std::rc::Rc;

use yew::{
    html::{ImplicitClone, IntoPropValue},
    prelude::*,
};

use crate::PaudleMsg;

#[derive(PartialEq, Clone, Debug)]
pub enum KeyStatus {
    Unused,
    Absent,
    Present,
    Correct,
}

#[derive(PartialEq, Clone)]
pub struct KeyValue {
    pub status: KeyStatus,
    pub letter: char,
}

#[derive(PartialEq, Clone)]
pub enum KeyType {
    Letter(KeyValue),
    Enter,
    Backspace,
}

impl ImplicitClone for KeyType {}

impl IntoPropValue<KeyType> for KeyValue {
    fn into_prop_value(self) -> KeyType {
        KeyType::Letter(self)
    }
}

#[derive(Properties, PartialEq)]
pub struct KeyProps {
    pub def: KeyType,
    pub key_press: Callback<PaudleMsg>,
}

impl KeyProps {
    fn status_string(&self) -> String {
        match &self.def {
            KeyType::Letter(letter) => match letter.status {
                KeyStatus::Unused => "unused",
                KeyStatus::Absent => "absent",
                KeyStatus::Present => "present",
                KeyStatus::Correct => "correct",
            },
            KeyType::Enter | KeyType::Backspace => "unused",
        }
        .to_string()
    }

    fn class(&self) -> Classes {
        let mut classes = classes!("key");
        if matches!(self.def, KeyType::Enter | KeyType::Backspace) {
            classes.push("special-key");
        }
        classes
    }

    fn disp(&self) -> String {
        match &self.def {
            KeyType::Letter(l) => l.letter.to_string(),
            KeyType::Enter => "ENTER".to_string(),
            KeyType::Backspace => "DEL".to_string(),
        }
    }
}

#[function_component(Key)]
pub fn key(props: &KeyProps) -> Html {
    let def = Rc::new(props.def.clone());
    let key_press = props.key_press.clone();
    let onclick = Callback::from(move |_: MouseEvent| match &*def {
        KeyType::Letter(l) => key_press.emit(PaudleMsg::TypeLetter(l.letter)),
        KeyType::Enter => key_press.emit(PaudleMsg::Submit),
        KeyType::Backspace => key_press.emit(PaudleMsg::Backspace),
    });
    html! {
      <div onclick={onclick} data-status={props.status_string()} class={props.class()}>
        {props.disp()}
      </div>
    }
}

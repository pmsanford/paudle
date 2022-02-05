use std::rc::Rc;

pub const ENTER: &str = "Enter";
pub const BACKSPACE: &str = "Backspace";
pub const ESCAPE: &str = "Escape";

use yew::{
    html::{ImplicitClone, IntoPropValue},
    prelude::*,
};

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
    let key_id = match &*def {
        KeyType::Letter(l) => l.letter.to_string(),
        KeyType::Enter => "Enter".to_string(),
        KeyType::Backspace => "Backspace".to_string(),
    };
    html! {
      <div data-key-id={key_id} data-status={props.status_string()} class={props.class()}>
        {props.disp()}
      </div>
    }
}

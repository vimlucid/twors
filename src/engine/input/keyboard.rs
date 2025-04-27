use super::{
    event_listener::{self, EventHandler},
    key_state_map::KeyStateMap,
};
use crate::error::Result;
use log::warn;
use std::{cell::RefCell, rc::Rc};
use web_sys::{KeyboardEvent, Window};

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum Button {
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,

    AltLeft,
    AltRight,
    Backspace,
    CapsLock,
    ControlLeft,
    ControlRight,
    Delete,
    End,
    Enter,
    Escape,
    Home,
    Insert,
    MetaLeft,
    MetaRight,
    PageDown,
    PageUp,
    ShiftLeft,
    ShiftRight,
    Space,
    Tab,

    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
    KeyG,
    KeyH,
    KeyI,
    KeyJ,
    KeyK,
    KeyL,
    KeyM,
    KeyN,
    KeyO,
    KeyP,
    KeyQ,
    KeyR,
    KeyS,
    KeyT,
    KeyU,
    KeyV,
    KeyW,
    KeyX,
    KeyY,
    KeyZ,

    Backquote,
    Backslash,
    BracketLeft,
    BracketRight,
    Comma,
    Equal,
    Minus,
    Period,
    Quote,
    Semicolon,
    Slash,

    NumLock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadAdd,
    NumpadComma,
    NumpadDecimal,
    NumpadDivide,
    NumpadEnter,
    NumpadEqual,
    NumpadMultiply,
    NumpadSubtract,
}

impl Button {
    /// Accepted key codes are a partial subset from
    /// https://developer.mozilla.org/en-US/docs/Web/API/UI_Events/Keyboard_event_code_values
    pub fn new(key_code: &str) -> Result<Button> {
        match key_code {
            "ArrowDown" => Ok(Button::ArrowDown),
            "ArrowLeft" => Ok(Button::ArrowLeft),
            "ArrowRight" => Ok(Button::ArrowRight),
            "ArrowUp" => Ok(Button::ArrowUp),

            "AltLeft" => Ok(Button::AltLeft),
            "AltRight" => Ok(Button::AltRight),
            "Backspace" => Ok(Button::Backspace),
            "CapsLock" => Ok(Button::CapsLock),
            "ControlLeft" => Ok(Button::ControlLeft),
            "ControlRight" => Ok(Button::ControlRight),
            "Delete" => Ok(Button::Delete),
            "End" => Ok(Button::End),
            "Enter" => Ok(Button::Enter),
            "Escape" => Ok(Button::Escape),
            "Home" => Ok(Button::Home),
            "Insert" => Ok(Button::Insert),
            "MetaLeft" => Ok(Button::MetaLeft),
            "MetaRight" => Ok(Button::MetaRight),
            "PageDown" => Ok(Button::PageDown),
            "PageUp" => Ok(Button::PageUp),
            "ShiftLeft" => Ok(Button::ShiftLeft),
            "ShiftRight" => Ok(Button::ShiftRight),
            "Space" => Ok(Button::Space),
            "Tab" => Ok(Button::Tab),

            "Digit0" => Ok(Button::Digit0),
            "Digit1" => Ok(Button::Digit1),
            "Digit2" => Ok(Button::Digit2),
            "Digit3" => Ok(Button::Digit3),
            "Digit4" => Ok(Button::Digit4),
            "Digit5" => Ok(Button::Digit5),
            "Digit6" => Ok(Button::Digit6),
            "Digit7" => Ok(Button::Digit7),
            "Digit8" => Ok(Button::Digit8),
            "Digit9" => Ok(Button::Digit9),

            "F1" => Ok(Button::F1),
            "F2" => Ok(Button::F2),
            "F3" => Ok(Button::F3),
            "F4" => Ok(Button::F4),
            "F5" => Ok(Button::F5),
            "F6" => Ok(Button::F6),
            "F7" => Ok(Button::F7),
            "F8" => Ok(Button::F8),
            "F9" => Ok(Button::F9),
            "F10" => Ok(Button::F10),
            "F11" => Ok(Button::F11),
            "F12" => Ok(Button::F12),

            "KeyA" => Ok(Button::KeyA),
            "KeyB" => Ok(Button::KeyB),
            "KeyC" => Ok(Button::KeyC),
            "KeyD" => Ok(Button::KeyD),
            "KeyE" => Ok(Button::KeyE),
            "KeyF" => Ok(Button::KeyF),
            "KeyG" => Ok(Button::KeyG),
            "KeyH" => Ok(Button::KeyH),
            "KeyI" => Ok(Button::KeyI),
            "KeyJ" => Ok(Button::KeyJ),
            "KeyK" => Ok(Button::KeyK),
            "KeyL" => Ok(Button::KeyL),
            "KeyM" => Ok(Button::KeyM),
            "KeyN" => Ok(Button::KeyN),
            "KeyO" => Ok(Button::KeyO),
            "KeyP" => Ok(Button::KeyP),
            "KeyQ" => Ok(Button::KeyQ),
            "KeyR" => Ok(Button::KeyR),
            "KeyS" => Ok(Button::KeyS),
            "KeyT" => Ok(Button::KeyT),
            "KeyU" => Ok(Button::KeyU),
            "KeyV" => Ok(Button::KeyV),
            "KeyW" => Ok(Button::KeyW),
            "KeyX" => Ok(Button::KeyX),
            "KeyY" => Ok(Button::KeyY),
            "KeyZ" => Ok(Button::KeyZ),

            "Backquote" => Ok(Button::Backquote),
            "Backslash" => Ok(Button::Backslash),
            "BracketLeft" => Ok(Button::BracketLeft),
            "BracketRight" => Ok(Button::BracketRight),
            "Comma" => Ok(Button::Comma),
            "Equal" => Ok(Button::Equal),
            "Minus" => Ok(Button::Minus),
            "Period" => Ok(Button::Period),
            "Quote" => Ok(Button::Quote),
            "Semicolon" => Ok(Button::Semicolon),
            "Slash" => Ok(Button::Slash),

            "NumLock" => Ok(Button::NumLock),
            "Numpad0" => Ok(Button::Numpad0),
            "Numpad1" => Ok(Button::Numpad1),
            "Numpad2" => Ok(Button::Numpad2),
            "Numpad3" => Ok(Button::Numpad3),
            "Numpad4" => Ok(Button::Numpad4),
            "Numpad5" => Ok(Button::Numpad5),
            "Numpad6" => Ok(Button::Numpad6),
            "Numpad7" => Ok(Button::Numpad7),
            "Numpad8" => Ok(Button::Numpad8),
            "Numpad9" => Ok(Button::Numpad9),
            "NumpadAdd" => Ok(Button::NumpadAdd),
            "NumpadComma" => Ok(Button::NumpadComma),
            "NumpadDecimal" => Ok(Button::NumpadDecimal),
            "NumpadDivide" => Ok(Button::NumpadDivide),
            "NumpadEnter" => Ok(Button::NumpadEnter),
            "NumpadEqual" => Ok(Button::NumpadEqual),
            "NumpadMultiply" => Ok(Button::NumpadMultiply),
            "NumpadSubtract" => Ok(Button::NumpadSubtract),

            key_code => Err(format!("Unsupported keyboard key code: '{}'", key_code).into()),
        }
    }
}

#[derive(Default)]
pub struct Keyboard {
    state_map: Rc<RefCell<KeyStateMap<Button>>>,
}

impl Keyboard {
    pub fn init(&self, window: &Window) -> Result<()> {
        self.attach_key_up_handler(window)?;
        self.attach_key_down_handler(window)?;

        Ok(())
    }

    pub fn is_pressed(&self, key: Button) -> bool {
        self.state_map.borrow().is_pressed(&key)
    }

    pub fn is_down(&self, key: Button) -> bool {
        self.state_map.borrow().is_down(&key)
    }

    pub fn is_released(&self, key: Button) -> bool {
        self.state_map.borrow().is_released(&key)
    }

    pub fn is_up(&self, key: Button) -> bool {
        self.state_map.borrow().is_up(&key)
    }

    pub fn transition_states(&mut self) {
        self.state_map.borrow_mut().transition_states()
    }

    fn attach_key_down_handler(&self, window: &Window) -> Result<()> {
        let state_map = self.state_map.clone();
        event_listener::attach(
            window,
            "keydown",
            EventHandler::new(move |event: KeyboardEvent| {
                let button = match Button::new(&event.code()) {
                    Ok(button) => button,
                    Err(err) => {
                        warn!("{}", err);
                        return;
                    }
                };

                state_map.borrow_mut().handle_key_down(&button);
            }),
        )
    }

    fn attach_key_up_handler(&self, window: &Window) -> Result<()> {
        let state_map = self.state_map.clone();
        event_listener::attach(
            window,
            "keyup",
            EventHandler::new(move |event: KeyboardEvent| {
                let button = match Button::new(&event.code()) {
                    Ok(button) => button,
                    Err(err) => {
                        warn!("{}", err);
                        return;
                    }
                };

                state_map.borrow_mut().handle_key_up(&button);
            }),
        )
    }
}

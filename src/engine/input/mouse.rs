use super::{
    event_listener::{self, EventHandler},
    key_state_map::KeyStateMap,
};
use crate::{Vertex2, error::Result};
use log::warn;
use std::{cell::RefCell, rc::Rc};
use web_sys::{MouseEvent, Window};

/// Used in the `mouse` field in the [Input](super) module API
#[derive(Eq, PartialEq, Hash, Clone, Copy)]
#[repr(u8)]
pub enum Button {
    LMB = 0,
    MMB = 1,
    RMB = 2,
}

impl Button {
    pub fn new(key_code: i16) -> Result<Button> {
        match key_code {
            0 => Ok(Button::LMB),
            1 => Ok(Button::MMB),
            2 => Ok(Button::RMB),
            key_code => Err(format!("Unsupported mouse key code: '{}'", key_code).into()),
        }
    }
}

#[derive(Default)]
pub struct Mouse {
    state_map: Rc<RefCell<KeyStateMap<Button>>>,
    position: Rc<RefCell<Vertex2<i32>>>,
}

impl Mouse {
    pub fn init(&self, window: &Window) -> Result<()> {
        self.attach_mouse_up_handler(window)?;
        self.attach_mouse_down_handler(window)?;
        self.attach_mouse_move_handler(window)?;

        Ok(())
    }

    pub fn position(&self) -> Vertex2<f32> {
        let position = self.position.borrow();
        Vertex2::new(position.x as f32, position.y as f32)
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

    fn attach_mouse_down_handler(&self, window: &Window) -> Result<()> {
        let state_map = self.state_map.clone();
        event_listener::attach(
            window,
            "mousedown",
            EventHandler::new(move |event: MouseEvent| {
                let button = match Button::new(event.button()) {
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

    fn attach_mouse_up_handler(&self, window: &Window) -> Result<()> {
        let state_map = self.state_map.clone();
        event_listener::attach(
            window,
            "mouseup",
            EventHandler::new(move |event: MouseEvent| {
                let button = match Button::new(event.button()) {
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

    fn attach_mouse_move_handler(&self, window: &Window) -> Result<()> {
        let position = self.position.clone();
        event_listener::attach(
            window,
            "mousemove",
            EventHandler::new(move |event: MouseEvent| {
                let mut position = position.borrow_mut();
                position.x = event.client_x();
                position.y = event.client_y();
            }),
        )
    }
}

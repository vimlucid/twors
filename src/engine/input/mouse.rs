use super::key_state_map::KeyStateMap;
use crate::{Vertex2, error::Result};
use log::error;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::{MouseEvent, Window};

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
#[repr(u8)]
pub enum Button {
    Main = 0,
    Secondary = 1,
}

impl Button {
    pub fn new(key_code: i16) -> Result<Button> {
        match key_code {
            0 => Ok(Button::Main),
            1 => Ok(Button::Secondary),
            key_code => Err(format!("Unsupported mouse button key code: '{}'", key_code).into()),
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
        self.attach_event_listener(
            window,
            "mousedown",
            Closure::new(move |event: MouseEvent| {
                let button = match Button::new(event.button()) {
                    Ok(button) => button,
                    Err(err) => {
                        error!("{}", err);
                        return;
                    }
                };

                state_map.borrow_mut().handle_key_down(&button);
            }),
        )
    }

    fn attach_mouse_up_handler(&self, window: &Window) -> Result<()> {
        let state_map = self.state_map.clone();
        self.attach_event_listener(
            window,
            "mouseup",
            Closure::new(move |event: MouseEvent| {
                let button = match Button::new(event.button()) {
                    Ok(button) => button,
                    Err(err) => {
                        error!("{}", err);
                        return;
                    }
                };

                state_map.borrow_mut().handle_key_up(&button);
            }),
        )
    }

    fn attach_mouse_move_handler(&self, window: &Window) -> Result<()> {
        let position = self.position.clone();
        self.attach_event_listener(
            window,
            "mousemove",
            Closure::new(move |event: MouseEvent| {
                let mut position = position.borrow_mut();
                position.x = event.client_x();
                position.y = event.client_y();
            }),
        )
    }

    fn attach_event_listener(
        &self,
        window: &Window,
        event_name: &str,
        handler: Closure<dyn Fn(MouseEvent)>,
    ) -> Result<()> {
        window
            .add_event_listener_with_callback(event_name, handler.as_ref().unchecked_ref())
            .map_err(|_| format!("Failed to attach {} event listener", event_name))?;

        handler.forget();

        Ok(())
    }
}

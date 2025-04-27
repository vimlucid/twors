mod event_listener;
mod key_state_map;
mod keyboard;
mod mouse;

pub use keyboard::Button as Key;
pub use mouse::Button as Mouse;

use crate::error::Result;
use keyboard::Keyboard;
use web_sys::Window;

#[derive(Default)]
pub struct Input {
    pub mouse: mouse::Mouse,
    pub keyboard: Keyboard,
}

impl Input {
    pub fn init(&self, window: &Window) -> Result<()> {
        self.mouse.init(window)?;
        self.keyboard.init(window)
    }

    /// Transitions ("pressed" -> "down") and ("released" -> "inactive")
    ///
    /// # Notes
    /// - Make sure to call this method **AFTER** executing the main loop logic or "is_pressed" and
    ///   "is_released" will always return false.
    pub fn transition_states(&mut self) {
        self.mouse.transition_states();
        self.keyboard.transition_states();
    }
}

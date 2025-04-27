mod key_state_map;
mod mouse;

pub use mouse::Button as MouseButton;

use crate::error::Result;
use mouse::Mouse;
use web_sys::Window;

#[derive(Default)]
pub struct Input {
    pub mouse: Mouse,
}

impl Input {
    pub fn init(&self, window: &Window) -> Result<()> {
        self.mouse.init(window)
    }

    /// Transitions ("pressed" -> "down") and ("released" -> "inactive")
    ///
    /// # Notes
    /// - Make sure to call this method **AFTER** executing the main loop logic or "is_pressed" and
    ///   "is_released" will always return false.
    pub fn transition_states(&mut self) {
        self.mouse.transition_states();
    }
}

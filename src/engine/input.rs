//! A module for handling inputs - the `mouse` and `keyboard` fields are used respectively for
//! detecting mouse and keyboard inputs.
//!
//! Both modules follow the same convention:
//! - use `is_pressed` and `is_released` methods for a one time key activation or deactivation -
//!   these will return `true` only for a single frame and `false` for all remaining frames.
//! - use `is_down` and `is_up` methods for detecting when a key is in the "down" or "up" states -
//!   these will keep returning `true` until the key is respectively released or pressed.
//!
//! Use the [Key](Key) enum for the `keyboard` methods and the [Mouse](Mouse) enum for the `mouse`
//! methods

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
    #[doc(hidden)]
    pub fn init(&self, window: &Window) -> Result<()> {
        self.mouse.init(window)?;
        self.keyboard.init(window)
    }

    #[doc(hidden)]
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

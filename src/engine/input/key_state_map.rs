use std::collections::HashMap;
use std::hash::Hash;

/// "Pressed" and "Released" should be active for a single frame
/// "Down" and "Up" are returned instead of "Pressed" and "Released" after that single frame
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyState {
    Up,
    Down,
    Pressed,
    Released,
}

pub struct KeyStateMap<T>
where
    T: Clone + Eq + Hash + PartialEq,
{
    keys: HashMap<T, KeyState>,
}

// #[derive(Default)] doesn't work if T is not Default, but T doesn't need to be Default, so we
// implement Defaulgit@github_vimlucid:vimlucid/twors.git manually instead
impl<T> Default for KeyStateMap<T>
where
    T: Clone + Eq + Hash + PartialEq,
{
    fn default() -> Self {
        Self {
            keys: HashMap::default(),
        }
    }
}

// TODO: Sort trait bounds automatically?
impl<T> KeyStateMap<T>
where
    T: Clone + Eq + Hash + PartialEq,
{
    pub fn is_pressed(&self, key: &T) -> bool {
        let state = self.state(key);
        state == KeyState::Pressed
    }

    pub fn is_down(&self, key: &T) -> bool {
        let state = self.state(key);
        (state == KeyState::Pressed) || (state == KeyState::Down)
    }

    pub fn is_released(&self, key: &T) -> bool {
        let state = self.state(key);
        state == KeyState::Released
    }

    pub fn is_up(&self, key: &T) -> bool {
        let state = self.state(key);
        (state == KeyState::Released) || (state == KeyState::Up)
    }

    pub fn transition_states(&mut self) {
        for (_, curr_state) in self.keys.iter_mut() {
            let new_state = match curr_state {
                KeyState::Pressed => KeyState::Down,
                KeyState::Released => KeyState::Up,
                _ => *curr_state,
            };

            *curr_state = new_state;
        }
    }

    pub fn handle_key_down(&mut self, key: &T) {
        self.keys.insert(key.clone(), KeyState::Pressed);
    }

    pub fn handle_key_up(&mut self, key: &T) {
        self.keys.insert(key.clone(), KeyState::Released);
    }

    fn state(&self, key: &T) -> KeyState {
        let state = self.keys.get(key);
        match state {
            Some(current_state) => *current_state,
            None => KeyState::Up,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Eq, Hash, PartialEq)]
    enum MouseButton {
        Main,
        Secondary,
    }

    #[test]
    fn initial() {
        let input = KeyStateMap::default();
        assert_eq!(input.state(&MouseButton::Main), KeyState::Up);
    }

    #[test]
    fn transition_noop() {
        let mut input = KeyStateMap::default();
        input.transition_states();
        assert_eq!(input.state(&MouseButton::Main), KeyState::Up);
    }

    #[test]
    fn transition_pressed_down() {
        let mut input = KeyStateMap::default();

        input.handle_key_down(&MouseButton::Main);
        assert_eq!(input.state(&MouseButton::Main), KeyState::Pressed);

        input.transition_states();
        assert_eq!(input.state(&MouseButton::Main), KeyState::Down);
    }

    #[test]
    fn transition_released_inactive() {
        let mut input = KeyStateMap::default();
        input.handle_key_down(&MouseButton::Main);
        input.transition_states();
        assert_eq!(input.state(&MouseButton::Main), KeyState::Down);

        input.handle_key_up(&MouseButton::Main);
        assert_eq!(input.state(&MouseButton::Main), KeyState::Released); // pressed vs released

        input.transition_states();
        assert_eq!(input.state(&MouseButton::Main), KeyState::Up);
    }

    #[test]
    fn independent() {
        let mut input = KeyStateMap::default();

        input.handle_key_down(&MouseButton::Main);
        assert_eq!(input.state(&MouseButton::Main), KeyState::Pressed);
        assert_eq!(input.state(&MouseButton::Secondary), KeyState::Up);

        input.transition_states();
        assert_eq!(input.state(&MouseButton::Main), KeyState::Down);
        assert_eq!(input.state(&MouseButton::Secondary), KeyState::Up);

        input.handle_key_up(&MouseButton::Main);
        input.handle_key_down(&MouseButton::Secondary);
        assert_eq!(input.state(&MouseButton::Main), KeyState::Released);
        assert_eq!(input.state(&MouseButton::Secondary), KeyState::Pressed);

        input.transition_states();
        assert_eq!(input.state(&MouseButton::Main), KeyState::Up);
        assert_eq!(input.state(&MouseButton::Secondary), KeyState::Down);
    }
}

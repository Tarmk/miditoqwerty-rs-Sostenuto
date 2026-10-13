#![allow(unused_variables)]
#![allow(dead_code)]

use midi_event::Note;

use crate::keycodes::{KeyEvents, Key};

mod generic;
mod pv;
mod piano_rooms;

pub trait InputMethod {
    fn get_name(&self) -> String;
    fn press_note(&mut self, note: Note, velocity: u8) -> KeyEvents;
    fn release_note(&mut self, note: Note) -> KeyEvents;
    fn reset(&mut self, data: &str);
    fn process_sustain(&mut self, value: u8) -> KeyEvents;
    fn process_sostenuto(&mut self, value: u8) -> KeyEvents;
}


pub mod unified {
    pub use super::generic::Inner as generic_inner;
    pub use super::pv::Inner as pv_inner;
    pub use super::piano_rooms::Inner as piano_rooms_inner;
}

#[cfg(test)]
mod tests {
    use super::unified::{generic_inner, piano_rooms_inner, pv_inner};
    use super::InputMethod;
    use crate::keycodes::{Key, KeyEvent};

    fn is_press(events: &[KeyEvent], name: &str) -> bool {
        matches!(events, [KeyEvent::Press(k)] if k.code == Key::new(name).code)
    }

    fn is_release(events: &[KeyEvent], name: &str) -> bool {
        matches!(events, [KeyEvent::Release(k)] if k.code == Key::new(name).code)
    }

    /// CC 66 >= 64 holds `]` once, < 64 releases it once, repeats are ignored.
    fn check_sostenuto(method: &mut dyn InputMethod) {
        assert!(is_press(&method.process_sostenuto(127), "rightbrace"));
        assert!(method.process_sostenuto(100).is_empty(), "already held");
        assert!(is_release(&method.process_sostenuto(0), "rightbrace"));
        assert!(method.process_sostenuto(10).is_empty(), "already released");
        // The threshold is 64, the same as the sustain pedal.
        assert!(method.process_sostenuto(63).is_empty());
        assert!(is_press(&method.process_sostenuto(64), "rightbrace"));
    }

    /// The sustain (space) and sostenuto (`]`) pedals do not affect each other.
    fn check_pedals_independent(method: &mut dyn InputMethod) {
        assert!(is_press(&method.process_sustain(127), "space"));
        assert!(is_press(&method.process_sostenuto(127), "rightbrace"));
        assert!(is_release(&method.process_sustain(0), "space"));
        assert!(is_release(&method.process_sostenuto(0), "rightbrace"));
    }

    #[test]
    fn generic_sostenuto() {
        check_sostenuto(&mut generic_inner::new());
        check_pedals_independent(&mut generic_inner::new());
    }

    #[test]
    fn pv_sostenuto() {
        check_sostenuto(&mut pv_inner::new());
        check_pedals_independent(&mut pv_inner::new());
    }

    #[test]
    fn reset_forgets_held_sostenuto() {
        let mut method = generic_inner::new();
        assert!(is_press(&method.process_sostenuto(127), "rightbrace"));
        method.reset("");
        assert!(is_press(&method.process_sostenuto(127), "rightbrace"));
    }

    #[test]
    fn piano_rooms_ignores_sostenuto() {
        let mut method = piano_rooms_inner;
        assert!(method.process_sostenuto(127).is_empty());
        assert!(method.process_sostenuto(0).is_empty());
    }
}

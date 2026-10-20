use crate::config::KeyConfig;
use crate::structs::input::KeyPress;

// the toggle chord (the key, with Ctrl when configured) and the hold that follows a press: Windows repeats
// key-down while a key is held, and every repeat and the release belong to the press that started the hold
pub struct ToggleKey {
    code: u32,
    needs_ctrl: bool,
    // Some while the key is held: true when the press that started the hold toggled
    hold: Option<bool>,
}

impl ToggleKey {
    pub fn new(config: &KeyConfig) -> Self {
        Self { code: config.toggle, needs_ctrl: config.ctrl, hold: None }
    }

    // ctrl is whether Ctrl is down at the event; allowed is whether a toggle may happen now
    pub fn press(&mut self, code: u32, down: bool, ctrl: bool, allowed: bool) -> KeyPress {
        if code != self.code {
            return KeyPress::Ignore;
        }
        if !down {
            return Self::held(self.hold.take());
        }
        if self.hold.is_some() {
            return Self::held(self.hold);
        }
        if self.needs_ctrl && !ctrl {
            self.hold = Some(false);
            return KeyPress::Ignore;
        }
        self.hold = Some(allowed);
        if allowed { KeyPress::Toggle } else { KeyPress::Blocked }
    }

    // forgets the hold, for a key-up the game never delivered (focus lost, a screen opened)
    pub fn reset(&mut self) {
        self.hold = None;
    }

    fn held(hold: Option<bool>) -> KeyPress {
        if hold == Some(true) { KeyPress::Swallow } else { KeyPress::Ignore }
    }
}

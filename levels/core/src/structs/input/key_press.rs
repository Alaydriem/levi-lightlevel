// what a key event means for the overlay
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyPress {
    // not the toggle chord, or a repeat or release of a press that did not toggle: the game gets it
    Ignore,
    // the toggle chord, pressed where toggling is allowed
    Toggle,
    // a repeat or the release of a press that toggled: the game must not see it
    Swallow,
    // the toggle chord, pressed where toggling is not allowed (no world, a screen open)
    Blocked,
}

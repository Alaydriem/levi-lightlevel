// what a mod manager event means for the overlay
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManagerSignal {
    // the toggle keybind was pressed
    TogglePressed,
    // the WorldDraw lease was denied, revoked or suspended
    LeaseLost,
    // the player disabled the mod in the manager
    Disabled,
}

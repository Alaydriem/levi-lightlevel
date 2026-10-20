use levels_core::config::KeyConfig;
use levels_core::services::ToggleKey;
use levels_core::structs::input::KeyPress;

const L: u32 = 0x4C;

fn ctrl_l() -> ToggleKey {
    ToggleKey::new(&KeyConfig::default())
}

#[test]
fn ctrl_l_toggles() {
    let mut key = ctrl_l();

    assert_eq!(key.press(L, true, true, true), KeyPress::Toggle);
}

#[test]
fn l_without_ctrl_passes_through() {
    let mut key = ctrl_l();

    assert_eq!(key.press(L, true, false, true), KeyPress::Ignore);
    assert_eq!(key.press(L, false, false, true), KeyPress::Ignore);
}

#[test]
fn other_keys_pass_through() {
    let mut key = ctrl_l();

    assert_eq!(key.press(0x41, true, true, true), KeyPress::Ignore);
}

#[test]
fn repeats_and_release_of_a_toggle_are_swallowed() {
    let mut key = ctrl_l();

    let presses = [
        key.press(L, true, true, true),
        key.press(L, true, true, true),
        key.press(L, true, true, true),
        key.press(L, false, true, true),
    ];

    assert_eq!(presses, [KeyPress::Toggle, KeyPress::Swallow, KeyPress::Swallow, KeyPress::Swallow]);
}

#[test]
fn ctrl_released_mid_hold_keeps_the_hold_swallowed() {
    let mut key = ctrl_l();
    key.press(L, true, true, true);

    assert_eq!(key.press(L, true, false, true), KeyPress::Swallow);
    assert_eq!(key.press(L, false, false, true), KeyPress::Swallow);
}

#[test]
fn not_allowed_press_is_blocked_and_its_repeats_pass_through() {
    let mut key = ctrl_l();

    assert_eq!(key.press(L, true, true, false), KeyPress::Blocked);
    assert_eq!(key.press(L, true, true, true), KeyPress::Ignore);
    assert_eq!(key.press(L, false, true, true), KeyPress::Ignore);
    assert_eq!(key.press(L, true, true, true), KeyPress::Toggle);
}

#[test]
fn reset_frees_a_key_whose_release_was_lost() {
    let mut key = ctrl_l();
    key.press(L, true, true, true);

    key.reset();

    assert_eq!(key.press(L, true, true, true), KeyPress::Toggle);
}

#[test]
fn ctrl_can_be_turned_off_in_config() {
    let mut key = ToggleKey::new(&KeyConfig { toggle: L, ctrl: false });

    assert_eq!(key.press(L, true, false, true), KeyPress::Toggle);
}

use amm_client::test_support::FakeTable;
use amm_core::structs::descriptor::Capability;
use amm_core::structs::event::{KeybindPhase, ModEvent};
use amm_core::structs::lease::AcquireOutcome;
use amm_core::structs::registry::ModIndex;
use levels_ffi::types::LvFrameStatus;

use crate::support::LvSession;

const OTHER_MOD: ModIndex = ModIndex(2);

fn acquired() -> Vec<Capability> {
    FakeTable::with(|state| state.acquired.clone())
}

fn released() -> Vec<Capability> {
    FakeTable::with(|state| state.released.clone())
}

fn columns_at(session: &LvSession, frame: u32) -> u32 {
    session.frame(&LvSession::in_world(frame as f64 / 60.0)).1.count
}

#[test]
fn registers_without_a_frame_callback() {
    let _session = LvSession::new();

    assert!(FakeTable::with(|state| state.registered_without_frame()));
}

#[test]
fn toggle_keybind_turns_on_and_acquires_world_draw() {
    let session = LvSession::new();

    session.turn_on();

    assert_eq!(acquired(), [Capability::WorldDraw]);
    assert!(columns_at(&session, 1) > 0);
}

#[test]
fn toggle_without_world_takes_no_lease() {
    let session = LvSession::new();
    FakeTable::push_event(LvSession::TOGGLE);

    session.frame(&LvSession::out_of_world(0.0));

    assert!(acquired().is_empty());
    assert_eq!(columns_at(&session, 1), 0);
}

#[test]
fn toggle_with_menu_open_takes_no_lease() {
    let session = LvSession::new();
    session.frame(&LvFrameStatus { menu_open: 1, ..LvSession::in_world(0.0) });
    FakeTable::push_event(LvSession::TOGGLE);

    session.frame(&LvFrameStatus { menu_open: 1, ..LvSession::in_world(1.0 / 60.0) });

    assert!(acquired().is_empty());
}

#[test]
fn denied_world_draw_keeps_overlay_off() {
    FakeTable::set_acquire_outcome(Capability::WorldDraw, AcquireOutcome::Denied { holder: OTHER_MOD });
    let session = LvSession::new();

    session.turn_on();

    assert_eq!(columns_at(&session, 1), 0);
    assert_eq!(columns_at(&session, 2), 0);
    assert_eq!(acquired(), [Capability::WorldDraw]);
}

#[test]
fn revoked_world_draw_turns_overlay_off() {
    let session = LvSession::new();
    session.turn_on();
    FakeTable::push_event(ModEvent::Revoked { capability: Capability::WorldDraw, by: OTHER_MOD });

    assert_eq!(columns_at(&session, 1), 0);
    assert!(released().is_empty());
}

#[test]
fn suspended_world_draw_turns_off_and_gives_the_lease_back() {
    let session = LvSession::new();
    session.turn_on();
    FakeTable::push_event(ModEvent::Suspended { capability: Capability::WorldDraw, by: OTHER_MOD });

    assert_eq!(columns_at(&session, 1), 0);
    assert_eq!(released(), [Capability::WorldDraw]);
}

#[test]
fn disabled_turns_overlay_off() {
    let session = LvSession::new();
    session.turn_on();
    FakeTable::push_event(ModEvent::Disabled);

    assert_eq!(columns_at(&session, 1), 0);
    assert!(released().is_empty());
}

#[test]
fn lost_lease_can_be_taken_again_with_the_toggle() {
    let session = LvSession::new();
    session.turn_on();
    FakeTable::push_event(ModEvent::Revoked { capability: Capability::WorldDraw, by: OTHER_MOD });
    columns_at(&session, 1);

    FakeTable::push_event(LvSession::TOGGLE);
    session.frame(&LvFrameStatus { has_player: 0, ..LvSession::in_world(2.0 / 60.0) });

    assert_eq!(acquired(), [Capability::WorldDraw, Capability::WorldDraw]);
    assert!(columns_at(&session, 3) > 0);
}

#[test]
fn leaving_world_releases_world_draw() {
    let session = LvSession::new();
    session.turn_on();

    session.frame(&LvSession::out_of_world(1.0 / 60.0));

    assert_eq!(released(), [Capability::WorldDraw]);
}

#[test]
fn repeat_and_release_phases_do_not_toggle() {
    let session = LvSession::new();
    session.turn_on();
    FakeTable::push_event(ModEvent::Keybind { action: 0, phase: KeybindPhase::Repeat });
    FakeTable::push_event(ModEvent::Keybind { action: 0, phase: KeybindPhase::Released });

    assert!(columns_at(&session, 1) > 0);
    assert_eq!(acquired(), [Capability::WorldDraw]);
    assert!(released().is_empty());
}

#[test]
fn settings_changed_rereads_the_document() {
    let session = LvSession::new();
    session.turn_on();
    session.frame(&LvSession::in_world(1.0 / 60.0));
    FakeTable::set_config("label {\n  yellow = [0.0, 0.0, 1.0, 1.0]\n}\n");
    FakeTable::push_event(ModEvent::SettingsChanged { paths: 1 });
    session.frame(&LvSession::in_world(2.0 / 60.0));
    session.submit(10, -20, &[LvSession::under_feet(7, 0)]);

    let (_, view) = session.draw();

    assert_eq!(view.count, 1);
    // SAFETY: the view is valid for count elements until the next lv_draw_list on this handle
    let label = unsafe { *view.labels };
    assert_eq!((label.r, label.g, label.b, label.a), (0.0, 0.0, 1.0, 1.0));
}

#[test]
fn unreadable_settings_keep_the_last_config() {
    let session = LvSession::with_config("label {\n  yellow = [0.0, 0.0, 1.0, 1.0]\n}\n");
    session.turn_on();
    session.frame(&LvSession::in_world(1.0 / 60.0));
    FakeTable::set_config("label {");
    FakeTable::push_event(ModEvent::SettingsChanged { paths: 1 });
    session.frame(&LvSession::in_world(2.0 / 60.0));
    session.submit(10, -20, &[LvSession::under_feet(7, 0)]);

    let (_, view) = session.draw();

    // SAFETY: the view is valid for count elements until the next lv_draw_list on this handle
    let label = unsafe { *view.labels };
    assert_eq!((label.r, label.g, label.b), (0.0, 0.0, 1.0));
}

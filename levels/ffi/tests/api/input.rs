use levels_ffi::api::input::lv_key;
use levels_ffi::types::LvStatus;

use crate::support::LvSession;
use crate::support::session::TOGGLE;

#[test]
fn null_handle_is_invalid() {
    let mut consume = 0u8;

    assert_eq!(lv_key(std::ptr::null(), TOGGLE, 1, 1, &mut consume), LvStatus::InvalidArgument);
}

#[test]
fn null_consume_is_invalid() {
    let session = LvSession::new();

    assert_eq!(lv_key(session.handle, TOGGLE, 1, 1, std::ptr::null_mut()), LvStatus::InvalidArgument);
}

#[test]
fn toggle_in_world_is_consumed() {
    let session = LvSession::new();

    assert_eq!(session.turn_on(), 1);
}

#[test]
fn toggle_before_any_frame_passes_through() {
    let session = LvSession::new();
    let mut consume = 1u8;

    assert_eq!(lv_key(session.handle, TOGGLE, 1, 1, &mut consume), LvStatus::Ok);
    assert_eq!(consume, 0);
}

#[test]
fn toggle_without_ctrl_passes_through() {
    let session = LvSession::new();
    session.frame(&LvSession::in_world(0.0));
    let mut consume = 1u8;

    assert_eq!(lv_key(session.handle, TOGGLE, 1, 0, &mut consume), LvStatus::Ok);
    assert_eq!(consume, 0);
}

use levels_ffi::api::frame::lv_frame;
use levels_ffi::types::{LvScanView, LvStatus};

use crate::support::LvSession;

#[test]
fn null_arguments_are_invalid() {
    let session = LvSession::new();
    let status = LvSession::in_world(0.0);
    let mut out = LvScanView::default();

    assert_eq!(lv_frame(std::ptr::null(), &status, &mut out), LvStatus::InvalidArgument);
    assert_eq!(lv_frame(session.handle, std::ptr::null(), &mut out), LvStatus::InvalidArgument);
    assert_eq!(lv_frame(session.handle, &status, std::ptr::null_mut()), LvStatus::InvalidArgument);
}

#[test]
fn off_frame_requests_nothing() {
    let session = LvSession::new();

    let (code, view) = session.frame(&LvSession::in_world(0.0));

    assert_eq!(code, LvStatus::Ok);
    assert_eq!(view.count, 0);
}

#[test]
fn toggle_then_frame_returns_columns() {
    let session = LvSession::new();
    session.turn_on();

    let (code, view) = session.frame(&LvSession::in_world(1.0 / 60.0));

    assert_eq!(code, LvStatus::Ok);
    assert!(view.count > 0);
    assert!(!view.columns.is_null());
    // SAFETY: the view is valid for count elements until the next lv_frame on this handle
    let first = unsafe { *view.columns };
    assert_eq!((first.x, first.z, first.y_min, first.y_max), (10, -20, 64 - 24, 64 + 24));
}

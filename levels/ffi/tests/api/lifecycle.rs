use levels_ffi::api::lifecycle::{lv_create, lv_destroy};
use levels_ffi::types::LvStr;

use crate::support::LvSession;

#[test]
fn null_config_path_gives_a_null_handle() {
    assert!(lv_create(std::ptr::null(), None).is_null());
}

#[test]
fn non_utf8_config_path_gives_a_null_handle() {
    let bad = [0xFFu8];
    let path = LvStr { ptr: bad.as_ptr(), len: 1 };

    assert!(lv_create(&path, None).is_null());
}

#[test]
fn create_with_missing_config_works() {
    let session = LvSession::new();

    assert!(!session.handle.is_null());
}

#[test]
fn destroy_null_is_safe() {
    lv_destroy(std::ptr::null_mut());
}

use amm_abi::types::AmmLogLevel;
use amm_client::test_support::{FakeTable, LoggedCall};
use levels_ffi::api::lifecycle::{lv_create, lv_destroy, lv_log};
use levels_ffi::types::LvStr;

use crate::support::LvSession;

#[test]
fn null_config_dir_gives_a_null_handle() {
    assert!(lv_create(std::ptr::null()).is_null());
}

#[test]
fn non_utf8_config_dir_gives_a_null_handle() {
    let bad = [0xFFu8];
    let dir = LvStr { ptr: bad.as_ptr(), len: 1 };

    assert!(lv_create(&dir).is_null());
}

#[test]
fn create_without_manager_returns_null() {
    let text = "C:\\mods\\alaydriem-levels\\config";
    let dir = LvStr { ptr: text.as_ptr(), len: text.len() };

    assert!(lv_create(&dir).is_null());
}

#[test]
fn create_with_an_empty_config_works() {
    let session = LvSession::new();

    assert!(!session.handle.is_null());
}

#[test]
fn destroy_null_is_safe() {
    lv_destroy(std::ptr::null_mut());
}

fn log(level: u8, message: &str, fields: &str) {
    let file = "LevelsMod.cpp";
    lv_log(level, message.as_ptr(), message.len(), fields.as_ptr(), fields.len(), file.as_ptr(), file.len(), 7);
}

fn logged(message: &str) -> Vec<LoggedCall> {
    FakeTable::with(|state| state.logs.iter().filter(|call| call.message == message).cloned().collect())
}

#[test]
fn shim_log_line_reaches_the_manager_with_its_level_and_fields() {
    let _session = LvSession::new();

    log(AmmLogLevel::Warn as u8, "shim line", "{\"count\":3}");

    let calls = logged("shim line");
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].level, AmmLogLevel::Warn as u8);
    assert_eq!(calls[0].target, "alaydriem_levels::shim");
    assert!(calls[0].fields_json.contains("\"count\":3"));
    assert_eq!((calls[0].file.as_str(), calls[0].line), ("LevelsMod.cpp", 7));
}

#[test]
fn shim_log_line_with_bad_fields_is_still_sent() {
    let _session = LvSession::new();

    log(AmmLogLevel::Info as u8, "bad fields", "{not json");

    assert_eq!(logged("bad fields").len(), 1);
}

#[test]
fn shim_log_line_with_an_unknown_level_is_dropped() {
    let _session = LvSession::new();

    log(9, "unknown level", "{}");

    assert!(logged("unknown level").is_empty());
}

#[test]
fn shim_log_line_with_a_null_text_is_dropped() {
    let _session = LvSession::new();

    lv_log(AmmLogLevel::Error as u8, std::ptr::null(), 4, std::ptr::null(), 0, std::ptr::null(), 0, 1);

    assert!(FakeTable::with(|state| state.logs.iter().all(|call| call.target != "alaydriem_levels::shim")));
}

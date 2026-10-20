use levels_ffi::services::LvBoundary;
use levels_ffi::types::LvStatus;

#[test]
fn panic_is_stopped_at_boundary() {
    assert_eq!(LvBoundary::guard(|| panic!("x")), LvStatus::Panicked);
    assert_eq!(LvBoundary::guard(|| LvStatus::Ok), LvStatus::Ok);
}

#[test]
fn panic_in_value_guard_gives_the_fallback() {
    assert_eq!(LvBoundary::guard_value(7, || panic!("x")), 7);
}

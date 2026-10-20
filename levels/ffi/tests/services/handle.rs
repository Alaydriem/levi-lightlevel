use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::atomic::{AtomicUsize, Ordering};

use levels_core::config::LevelsConfig;
use levels_core::services::LevelsController;
use levels_ffi::services::{LogBridge, LvHandle};

static ERRORS: AtomicUsize = AtomicUsize::new(0);

extern "C" fn count_errors(level: u8, _message: *const u8, _len: usize) {
    if level == 1 {
        ERRORS.fetch_add(1, Ordering::SeqCst);
    }
}

#[test]
fn poisoned_lock_fails_every_call_and_logs_once() {
    LogBridge::install(Some(count_errors));
    let handle = LvHandle::new(LevelsController::new(LevelsConfig::default()));
    let _ = catch_unwind(AssertUnwindSafe(|| handle.with_state(|_| panic!("poison"))));

    let calls = [handle.with_state(|_| ()), handle.with_state(|_| ()), handle.with_state(|_| ())];

    assert_eq!(calls, [None, None, None]);
    assert_eq!(ERRORS.load(Ordering::SeqCst), 1);
}

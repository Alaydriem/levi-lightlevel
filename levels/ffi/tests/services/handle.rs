use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::Path;

use amm_abi::types::AmmLogLevel;
use amm_client::test_support::FakeTable;
use levels_ffi::services::LvHandle;

#[test]
fn poisoned_lock_fails_every_call_and_logs_once() {
    let handle = LvHandle::connect(FakeTable::table(), Path::new(FakeTable::CONFIG_DIR)).unwrap();
    let _ = catch_unwind(AssertUnwindSafe(|| handle.with_state(|_| panic!("poison"))));

    let calls = [handle.with_state(|_| ()), handle.with_state(|_| ()), handle.with_state(|_| ())];

    assert_eq!(calls, [None, None, None]);
    let poison_lines = FakeTable::with(|state| {
        state
            .logs
            .iter()
            .filter(|call| call.level == AmmLogLevel::Error as u8 && call.message.contains("poisoned"))
            .count()
    });
    assert_eq!(poison_lines, 1);
}

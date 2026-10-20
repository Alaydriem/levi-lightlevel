use crate::services::{LvBoundary, LvHandle};
use crate::types::LvStatus;

/// Passes one key event. `code` is a Windows virtual-key code, `down` is 1 for a press or repeat and 0 for a
/// release, and `ctrl` is 1 when Ctrl is held at the event. On Ok, `consume` is 1 when the game must not
/// see the key.
#[unsafe(no_mangle)]
pub extern "C" fn lv_key(handle: *const LvHandle, code: u32, down: u8, ctrl: u8, consume: *mut u8) -> LvStatus {
    LvBoundary::guard(|| {
        // SAFETY: the handle came from lv_create; consume is writable for the call
        let (Some(handle), Some(consume)) =
            (unsafe { LvBoundary::handle(handle) }, unsafe { LvBoundary::output(consume) })
        else {
            return LvStatus::InvalidArgument;
        };
        match handle.with_state(|state| state.key(code, down != 0, ctrl != 0)) {
            Some(consumed) => {
                *consume = u8::from(consumed);
                LvStatus::Ok
            }
            None => LvStatus::Failed,
        }
    })
}

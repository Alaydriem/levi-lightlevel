use levels_core::structs::frame::FrameStatus;

use crate::services::{LvBoundary, LvHandle};
use crate::types::{LvFrameStatus, LvScanView, LvStatus};

/// Takes this frame's game state and fills `out` with the columns to sample. On Ok, `out.columns` is
/// valid for `out.count` elements, borrows a buffer the handle owns, and stays valid until the next
/// lv_frame or lv_destroy call on the same handle.
#[unsafe(no_mangle)]
pub extern "C" fn lv_frame(handle: *const LvHandle, status: *const LvFrameStatus, out: *mut LvScanView) -> LvStatus {
    LvBoundary::guard(|| {
        // SAFETY: the handle came from lv_create; status is readable and out is writable for the call
        let (Some(handle), Some(status), Some(out)) =
            (unsafe { LvBoundary::handle(handle) }, unsafe { LvBoundary::input(status) }, unsafe {
                LvBoundary::output(out)
            })
        else {
            return LvStatus::InvalidArgument;
        };
        let status = FrameStatus::from(status);
        match handle.with_state(|state| state.frame(status)) {
            Some(view) => {
                *out = view;
                LvStatus::Ok
            }
            None => LvStatus::Failed,
        }
    })
}

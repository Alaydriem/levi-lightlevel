use levels_core::structs::world::ColumnPos;

use crate::services::{LvBoundary, LvHandle};
use crate::types::{LV_MAX_COLUMN_SURFACES, LvCamera, LvDrawView, LvStatus, LvSurfaceSample};

/// Stores the surfaces found in column (x, z), replacing what the column held. `count` 0 clears it.
/// `samples` may be null only when `count` is 0; `count` above LV_MAX_COLUMN_SURFACES is invalid.
#[unsafe(no_mangle)]
pub extern "C" fn lv_submit_column(
    handle: *const LvHandle,
    x: i32,
    z: i32,
    samples: *const LvSurfaceSample,
    count: u32,
) -> LvStatus {
    LvBoundary::guard(|| {
        if count > LV_MAX_COLUMN_SURFACES {
            return LvStatus::InvalidArgument;
        }
        // SAFETY: the handle came from lv_create; samples points to count readable elements for the call
        let (Some(handle), Some(samples)) =
            (unsafe { LvBoundary::handle(handle) }, unsafe { LvBoundary::slice(samples, count as usize) })
        else {
            return LvStatus::InvalidArgument;
        };
        match handle.with_state(|state| state.submit_column(ColumnPos { x, z }, samples)) {
            Some(()) => LvStatus::Ok,
            None => LvStatus::Failed,
        }
    })
}

/// Fills `out` with the labels to draw this frame. `game_tags` is the number of name tags the game itself
/// draws this frame; past the headroom of 256 they lower the label count, nearest labels kept. `camera` is
/// the camera the frame is drawn from, read for the call; labels behind it are left out. Null, or a camera
/// with a zero or non-finite forward direction, leaves every label in. On Ok,
/// `out.labels` is valid for `out.count` elements, borrows a buffer the handle owns, and stays valid until
/// the next lv_draw_list or lv_destroy call on the same handle. lv_frame and lv_submit_column do not touch it.
#[unsafe(no_mangle)]
pub extern "C" fn lv_draw_list(
    handle: *const LvHandle,
    game_tags: u32,
    camera: *const LvCamera,
    out: *mut LvDrawView,
) -> LvStatus {
    LvBoundary::guard(|| {
        // SAFETY: the handle came from lv_create and out is writable for the call
        let (Some(handle), Some(out)) = (unsafe { LvBoundary::handle(handle) }, unsafe { LvBoundary::output(out) })
        else {
            return LvStatus::InvalidArgument;
        };
        // SAFETY: camera is null or points to an LvCamera the shim owns for the call
        let camera = unsafe { LvBoundary::input(camera) }.and_then(LvCamera::to_core);
        match handle.with_state(|state| state.draw_list(game_tags as usize, camera)) {
            Some(view) => {
                *out = view;
                LvStatus::Ok
            }
            None => LvStatus::Failed,
        }
    })
}

/// Reports, for telemetry, the shim's time in microseconds for this frame's sample step and name-tag append,
/// and the bytes of memory the shim holds for the overlay. Call it once per frame.
#[unsafe(no_mangle)]
pub extern "C" fn lv_report_timing(handle: *const LvHandle, sample_us: f32, draw_us: f32, shim_bytes: u64) -> LvStatus {
    LvBoundary::guard(|| {
        // SAFETY: the handle came from lv_create
        let Some(handle) = (unsafe { LvBoundary::handle(handle) }) else { return LvStatus::InvalidArgument };
        match handle.with_state(|state| state.timing(sample_us, draw_us, shim_bytes)) {
            Some(()) => LvStatus::Ok,
            None => LvStatus::Failed,
        }
    })
}

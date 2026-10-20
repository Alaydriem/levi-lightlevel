use std::path::PathBuf;

use levels_core::config::ConfigLoader;
use levels_core::services::LevelsController;

use crate::services::{LogBridge, LvBoundary, LvHandle};
use crate::types::{LvLogFn, LvStatus, LvStr};

/// Starts the overlay core from the UTF-8 path of config.json and installs `log` as the log sink. Returns
/// null when the path is null or not UTF-8; a missing or broken config file gives the defaults.
#[unsafe(no_mangle)]
pub extern "C" fn lv_create(config_file: *const LvStr, log: LvLogFn) -> *mut LvHandle {
    LogBridge::install(log);
    LvBoundary::guard_value(std::ptr::null_mut(), || {
        // SAFETY: the shim passes a pointer to an LvStr it owns for the call, or null
        let Some(path) = (unsafe { LvBoundary::input(config_file) }) else { return std::ptr::null_mut() };
        // SAFETY: the LvStr points to bytes the shim owns for the call
        let Some(path) = (unsafe { LvBoundary::text(path) }) else { return std::ptr::null_mut() };
        let config = ConfigLoader::load(&PathBuf::from(path));
        Box::into_raw(Box::new(LvHandle::new(LevelsController::new(config))))
    })
}

/// Frees the handle. The handle and every view it returned must not be used again. Null is ignored.
#[unsafe(no_mangle)]
pub extern "C" fn lv_destroy(handle: *mut LvHandle) {
    let _ = LvBoundary::guard(|| {
        if handle.is_null() {
            return LvStatus::InvalidArgument;
        }
        // SAFETY: a non-null handle came from lv_create and is destroyed exactly once
        drop(unsafe { Box::from_raw(handle) });
        LvStatus::Ok
    });
    LogBridge::uninstall();
}

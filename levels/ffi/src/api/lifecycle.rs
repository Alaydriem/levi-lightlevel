use std::path::Path;

use amm_client::link::ApiTable;

use crate::services::{LvBoundary, LvHandle, LvLogLine};
use crate::types::{LvStatus, LvStr};

/// Registers the overlay with the mod manager and starts its core from the manager's config.hcl. `config_dir`
/// is the mod's own LeviLamina config folder as an absolute UTF-8 path. Returns null when the path is null or
/// not UTF-8, when the mod manager is not loaded, or when it refuses the mod. Call it from the mod's enable,
/// never under the OS loader lock: the manager starts a log writer thread for each registration.
#[unsafe(no_mangle)]
pub extern "C" fn lv_create(config_dir: *const LvStr) -> *mut LvHandle {
    LvBoundary::guard_value(std::ptr::null_mut(), || {
        // SAFETY: the shim passes a pointer to an LvStr it owns for the call, or null
        let Some(dir) = (unsafe { LvBoundary::input(config_dir) }) else { return std::ptr::null_mut() };
        // SAFETY: the LvStr points to bytes the shim owns for the call
        let Some(dir) = (unsafe { LvBoundary::text(dir) }) else { return std::ptr::null_mut() };
        let Ok(table) = ApiTable::resolve() else { return std::ptr::null_mut() };
        match LvHandle::connect(table, Path::new(&dir)) {
            Ok(handle) => Box::into_raw(Box::new(handle)),
            Err(_) => std::ptr::null_mut(),
        }
    })
}

/// Unregisters from the mod manager and frees the handle. The handle and every view it returned must not be
/// used again. Null is ignored.
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
}

/// Sends one shim log line to the mod manager's log. `level` is an `AmmLogLevel` value and `fields_json` a JSON
/// object; each text is UTF-8 and lives for the call. Nothing is sent before lv_create succeeds or after
/// lv_destroy.
#[unsafe(no_mangle)]
pub extern "C" fn lv_log(
    level: u8,
    message: *const u8,
    message_len: usize,
    fields_json: *const u8,
    fields_len: usize,
    file: *const u8,
    file_len: usize,
    line: u32,
) {
    let _ = LvBoundary::guard(|| {
        let texts = [
            LvStr { ptr: message, len: message_len },
            LvStr { ptr: fields_json, len: fields_len },
            LvStr { ptr: file, len: file_len },
        ];
        // SAFETY: the shim passes the bytes of live string views for the call
        let [Some(message), Some(fields), Some(file)] = texts.map(|text| unsafe { LvBoundary::text(&text) }) else {
            return LvStatus::InvalidArgument;
        };
        LvLogLine::emit(level, &message, &fields, &file, line);
        LvStatus::Ok
    });
}

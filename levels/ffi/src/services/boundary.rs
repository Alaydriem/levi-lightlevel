use std::panic::{AssertUnwindSafe, catch_unwind};

use crate::services::LvHandle;
use crate::types::{LvStatus, LvStr};

// the rules every entry point follows: no panic crosses into C++, every pointer is checked
pub struct LvBoundary;

impl LvBoundary {
    // runs the body; a panic becomes LvStatus::Panicked and a log line
    pub fn guard(body: impl FnOnce() -> LvStatus) -> LvStatus {
        match catch_unwind(AssertUnwindSafe(body)) {
            Ok(status) => status,
            Err(payload) => {
                let message = payload
                    .downcast_ref::<&str>()
                    .map(|s| (*s).to_string())
                    .or_else(|| payload.downcast_ref::<String>().cloned())
                    .unwrap_or_else(|| "unknown panic".to_string());
                curia::error!("panic stopped at the C boundary", { payload: message });
                LvStatus::Panicked
            }
        }
    }

    // like guard, for entry points that return a value; `fallback` is returned on panic
    pub fn guard_value<T>(fallback: T, body: impl FnOnce() -> T) -> T {
        match catch_unwind(AssertUnwindSafe(body)) {
            Ok(value) => value,
            Err(_) => {
                curia::error!("panic stopped at the C boundary");
                fallback
            }
        }
    }

    // SAFETY: the caller passes either null or a pointer returned by lv_create and not yet destroyed
    pub unsafe fn handle<'a>(handle: *const LvHandle) -> Option<&'a LvHandle> {
        // SAFETY: see the function contract
        unsafe { handle.as_ref() }
    }

    // SAFETY: the caller passes either null or a pointer to a live, aligned T for the call
    pub unsafe fn input<'a, T>(value: *const T) -> Option<&'a T> {
        // SAFETY: see the function contract
        unsafe { value.as_ref() }
    }

    // SAFETY: the caller passes either null or a pointer to a live, aligned, writable T for the call
    pub unsafe fn output<'a, T>(value: *mut T) -> Option<&'a mut T> {
        // SAFETY: see the function contract
        unsafe { value.as_mut() }
    }

    // None when ptr is null and len is not 0; an empty slice when len is 0
    // SAFETY: the caller passes either null or a pointer to len live, aligned T for the call
    pub unsafe fn slice<'a, T>(ptr: *const T, len: usize) -> Option<&'a [T]> {
        if len == 0 {
            return Some(&[]);
        }
        if ptr.is_null() {
            return None;
        }
        // SAFETY: see the function contract
        Some(unsafe { std::slice::from_raw_parts(ptr, len) })
    }

    // SAFETY: ptr is null with len 0, or points to len readable bytes for the call
    pub unsafe fn text(value: &LvStr) -> Option<String> {
        // SAFETY: see the function contract
        let bytes = unsafe { Self::slice(value.ptr, value.len) }?;
        std::str::from_utf8(bytes).ok().map(str::to_string)
    }
}

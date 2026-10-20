use crate::types::LvColumnRequest;

// the columns to sample this frame; columns borrows a buffer the handle owns
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LvScanView {
    pub columns: *const LvColumnRequest,
    pub count: u32,
}

impl Default for LvScanView {
    fn default() -> Self {
        Self { columns: std::ptr::null(), count: 0 }
    }
}

use crate::types::LvLabel;

// the labels to draw this frame; labels borrows a buffer the handle owns. rotation is the quaternion
// (x, y, z, w) and scale every label shares
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LvDrawView {
    pub labels: *const LvLabel,
    pub count: u32,
    pub rotation: [f32; 4],
    pub scale: f32,
}

impl Default for LvDrawView {
    fn default() -> Self {
        Self { labels: std::ptr::null(), count: 0, rotation: [0.0, 0.0, 0.0, 1.0], scale: 1.0 }
    }
}

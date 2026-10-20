use levels_core::structs::draw::CameraView;

// the camera a frame is drawn from, in world space: its position and the direction it faces, any length
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct LvCamera {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub forward_x: f32,
    pub forward_y: f32,
    pub forward_z: f32,
}

impl LvCamera {
    // None when the forward direction is zero or not finite
    pub fn to_core(&self) -> Option<CameraView> {
        CameraView::from_forward([self.x, self.y, self.z], [self.forward_x, self.forward_y, self.forward_z])
    }
}

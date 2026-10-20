use levels_ffi::api::draw::{lv_draw_list, lv_submit_column};
use levels_ffi::api::frame::lv_frame;
use levels_ffi::api::input::lv_key;
use levels_ffi::api::lifecycle::{lv_create, lv_destroy};
use levels_ffi::services::LvHandle;
use levels_ffi::types::{LvCamera, LvDrawView, LvFrameStatus, LvScanView, LvStatus, LvStr, LvSurfaceSample};

pub const TOGGLE: u32 = 0x4C;

// a live handle created from a config path that does not exist, destroyed on drop
pub struct LvSession {
    pub handle: *mut LvHandle,
}

impl LvSession {
    pub fn new() -> Self {
        let dir = tempfile::tempdir().unwrap();
        Self::with_config(&dir.path().join("absent.json"))
    }

    pub fn with_config(file: &std::path::Path) -> Self {
        let text = file.to_str().unwrap();
        let path = LvStr { ptr: text.as_ptr(), len: text.len() };
        Self { handle: lv_create(&path, None) }
    }

    // in world, no menu, overworld, feet at (10, 64, -20)
    pub fn in_world(time_seconds: f64) -> LvFrameStatus {
        LvFrameStatus {
            time_seconds,
            in_world: 1,
            has_player: 1,
            menu_open: 0,
            hud_hidden: 0,
            dimension: 0,
            feet_x: 10,
            feet_y: 64,
            feet_z: -20,
            sky_darken: 0,
            yaw: 0.0,
        }
    }

    pub fn frame(&self, status: &LvFrameStatus) -> (LvStatus, LvScanView) {
        let mut out = LvScanView::default();
        let code = lv_frame(self.handle, status, &mut out);
        (code, out)
    }

    // an in-world frame, then one toggle press and release; returns the consume flag of the press
    pub fn turn_on(&self) -> u8 {
        self.frame(&Self::in_world(0.0));
        let mut consume = 0u8;
        assert_eq!(lv_key(self.handle, TOGGLE, 1, 1, &mut consume), LvStatus::Ok);
        let mut released = 0u8;
        assert_eq!(lv_key(self.handle, TOGGLE, 0, 1, &mut released), LvStatus::Ok);
        consume
    }

    pub fn submit(&self, x: i32, z: i32, samples: &[LvSurfaceSample]) -> LvStatus {
        lv_submit_column(self.handle, x, z, samples.as_ptr(), samples.len() as u32)
    }

    pub fn draw(&self) -> (LvStatus, LvDrawView) {
        self.draw_beside(0)
    }

    pub fn draw_beside(&self, game_tags: u32) -> (LvStatus, LvDrawView) {
        self.draw_from(game_tags, std::ptr::null())
    }

    pub fn draw_from(&self, game_tags: u32, camera: *const LvCamera) -> (LvStatus, LvDrawView) {
        let mut out = LvDrawView::default();
        let code = lv_draw_list(self.handle, game_tags, camera, &mut out);
        (code, out)
    }

    pub fn under_feet(block_light: u8, sky_light: u8) -> LvSurfaceSample {
        LvSurfaceSample { x: 10, y: 63, z: -20, block_light, sky_light }
    }
}

impl Drop for LvSession {
    fn drop(&mut self) {
        lv_destroy(self.handle);
    }
}

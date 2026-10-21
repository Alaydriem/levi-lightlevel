use std::path::Path;

use amm_client::test_support::FakeTable;
use amm_core::structs::event::{KeybindPhase, ModEvent};
use levels_ffi::api::draw::{lv_draw_list, lv_submit_column};
use levels_ffi::api::frame::lv_frame;
use levels_ffi::api::lifecycle::lv_destroy;
use levels_ffi::services::LvHandle;
use levels_ffi::types::{LvCamera, LvDrawView, LvFrameStatus, LvScanView, LvStatus, LvSurfaceSample};

// a live handle registered on this thread's fake manager, destroyed on drop
pub struct LvSession {
    pub handle: *mut LvHandle,
}

impl LvSession {
    pub const TOGGLE: ModEvent = ModEvent::Keybind { action: 0, phase: KeybindPhase::Pressed };

    pub fn new() -> Self {
        Self::with_config("")
    }

    // `config` is the config.hcl text the fake manager hands out
    pub fn with_config(config: &str) -> Self {
        FakeTable::set_config(config);
        let handle = LvHandle::connect(FakeTable::table(), Path::new(FakeTable::CONFIG_DIR)).unwrap();
        Self { handle: Box::into_raw(Box::new(handle)) }
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

    pub fn out_of_world(time_seconds: f64) -> LvFrameStatus {
        LvFrameStatus { in_world: 0, has_player: 0, ..Self::in_world(time_seconds) }
    }

    pub fn frame(&self, status: &LvFrameStatus) -> (LvStatus, LvScanView) {
        let mut out = LvScanView::default();
        let code = lv_frame(self.handle, status, &mut out);
        (code, out)
    }

    // an in-world frame, then the toggle keybind delivered on a frame without a player, so the toggle lands
    // and the scan planner is untouched until the next frame
    pub fn turn_on(&self) {
        self.frame(&Self::in_world(0.0));
        FakeTable::push_event(Self::TOGGLE);
        self.frame(&LvFrameStatus { has_player: 0, ..Self::in_world(0.0) });
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

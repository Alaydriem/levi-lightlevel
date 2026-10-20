use levels_core::structs::frame::FrameStatus;
use levels_core::structs::world::BlockPos;

// the game state the shim reads once per frame; every u8 flag is 0 or 1
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct LvFrameStatus {
    pub time_seconds: f64,
    pub in_world: u8,
    // 0 for a tick in a world with no local player; the fields below are then unset
    pub has_player: u8,
    // any screen is open: chat, a sign, a book, the pause menu
    pub menu_open: u8,
    // the HUD is hidden: IOptions::getHideHud()
    pub hud_hidden: u8,
    pub dimension: i32,
    // the block the player's feet are in
    pub feet_x: i32,
    pub feet_y: i32,
    pub feet_z: i32,
    // Dimension::mSkyDarken, 0 at noon to 11 at midnight
    pub sky_darken: u8,
    // LocalPlayer::getRotation().y in degrees
    pub yaw: f32,
}

impl From<&LvFrameStatus> for FrameStatus {
    fn from(value: &LvFrameStatus) -> Self {
        Self {
            time_seconds: value.time_seconds,
            in_world: value.in_world != 0,
            has_player: value.has_player != 0,
            menu_open: value.menu_open != 0,
            hud_hidden: value.hud_hidden != 0,
            dimension: value.dimension,
            feet: BlockPos { x: value.feet_x, y: value.feet_y, z: value.feet_z },
            sky_darken: value.sky_darken,
            yaw: value.yaw,
        }
    }
}

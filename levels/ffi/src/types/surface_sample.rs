use levels_core::structs::light::SurfaceSample;
use levels_core::structs::world::BlockPos;

// the most surfaces one column can return; equal to levels_core::config::MAX_COLUMN_SURFACES
pub const LV_MAX_COLUMN_SURFACES: u32 = 33;

// (x, y, z) is the solid block; both light values are read in the block above it, each 0 to 15
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LvSurfaceSample {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub block_light: u8,
    pub sky_light: u8,
}

impl From<&LvSurfaceSample> for SurfaceSample {
    fn from(value: &LvSurfaceSample) -> Self {
        Self {
            pos: BlockPos { x: value.x, y: value.y, z: value.z },
            block_light: value.block_light,
            sky_light: value.sky_light,
        }
    }
}

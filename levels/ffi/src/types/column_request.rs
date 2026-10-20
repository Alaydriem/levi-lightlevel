use levels_core::structs::frame::ColumnRequest;

// one column to sample: every block from y_min to y_max inclusive at (x, z)
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LvColumnRequest {
    pub x: i32,
    pub z: i32,
    pub y_min: i32,
    pub y_max: i32,
}

impl From<&ColumnRequest> for LvColumnRequest {
    fn from(value: &ColumnRequest) -> Self {
        Self { x: value.column.x, z: value.column.z, y_min: value.y_min, y_max: value.y_max }
    }
}

use levels_core::structs::draw::Label;

// one number to draw: absolute world position of the text, the value 0 to 15 and its RGBA color in 0..=1
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct LvLabel {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub value: u8,
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl From<&Label> for LvLabel {
    fn from(value: &Label) -> Self {
        let [r, g, b, a] = value.color;
        Self { x: value.x, y: value.y, z: value.z, value: value.value, r, g, b, a }
    }
}

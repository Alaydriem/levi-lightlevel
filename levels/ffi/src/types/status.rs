#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LvStatus {
    Ok = 0,
    InvalidArgument = 1,
    Panicked = 2,
    Failed = 3,
}

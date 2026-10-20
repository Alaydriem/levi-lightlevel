use serde::{Deserialize, Serialize};

pub const MAX_RADIUS: u32 = 32;
// a column of 2r + 1 blocks holds at most r + 1 surfaces
pub const MAX_COLUMN_SURFACES: usize = MAX_RADIUS as usize + 1;

const DEFAULT_PASS_SECONDS: f32 = 0.25;
const MAX_PASS_SECONDS: f32 = 5.0;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ScanConfig {
    pub radius: u32,
    // time for one full pass over every column in the radius
    pub pass_seconds: f32,
    pub max_labels: u32,
    pub max_columns_per_frame: u32,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self { radius: 24, pass_seconds: DEFAULT_PASS_SECONDS, max_labels: 8192, max_columns_per_frame: 512 }
    }
}

impl ScanConfig {
    pub fn sanitized(self) -> Self {
        let pass_seconds = if self.pass_seconds > 0.0 && self.pass_seconds <= MAX_PASS_SECONDS {
            self.pass_seconds
        } else {
            DEFAULT_PASS_SECONDS
        };
        Self {
            radius: self.radius.clamp(1, MAX_RADIUS),
            pass_seconds,
            max_labels: self.max_labels.max(1),
            max_columns_per_frame: self.max_columns_per_frame.max(1),
        }
    }
}

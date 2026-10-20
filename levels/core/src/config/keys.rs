use serde::{Deserialize, Serialize};

// Windows virtual-key codes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct KeyConfig {
    pub toggle: u32,
}

impl Default for KeyConfig {
    fn default() -> Self {
        Self { toggle: 0x4C }
    }
}

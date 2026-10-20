use serde::{Deserialize, Serialize};

// toggle is a Windows virtual-key code; with ctrl set, the toggle needs Ctrl held
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct KeyConfig {
    pub toggle: u32,
    pub ctrl: bool,
}

impl Default for KeyConfig {
    fn default() -> Self {
        Self { toggle: 0x4C, ctrl: true }
    }
}

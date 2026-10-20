use serde::{Deserialize, Serialize};

use crate::config::{KeyConfig, LabelConfig, LightConfig, ScanConfig};

const DEFAULT_MEMORY_BUDGET_MB: u32 = 32;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct LevelsConfig {
    pub keys: KeyConfig,
    pub scan: ScanConfig,
    pub light: LightConfig,
    pub label: LabelConfig,
    pub telemetry: bool,
    // resident memory the overlay may use; telemetry reports use against it
    pub memory_budget_mb: u32,
}

impl Default for LevelsConfig {
    fn default() -> Self {
        Self {
            keys: KeyConfig::default(),
            scan: ScanConfig::default(),
            light: LightConfig::default(),
            label: LabelConfig::default(),
            telemetry: false,
            memory_budget_mb: DEFAULT_MEMORY_BUDGET_MB,
        }
    }
}

impl LevelsConfig {
    // returns a copy with every value moved into its usable range
    pub fn sanitized(self) -> Self {
        Self {
            keys: self.keys,
            scan: self.scan.sanitized(),
            light: self.light.sanitized(),
            label: self.label.sanitized(),
            telemetry: self.telemetry,
            memory_budget_mb: self.memory_budget_mb.max(1),
        }
    }
}

use serde::{Deserialize, Serialize};

use crate::config::{KeyConfig, LabelConfig, LightConfig, ScanConfig};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct LevelsConfig {
    pub keys: KeyConfig,
    pub scan: ScanConfig,
    pub light: LightConfig,
    pub label: LabelConfig,
    pub telemetry: bool,
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
        }
    }
}

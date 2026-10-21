use crate::config::{GAME_TAG_HEADROOM, LevelsConfig};
use crate::services::{DrawListBuilder, LabelGrid, LabelRotation, LightClassifier, ScanPlanner, Telemetry};
use crate::structs::draw::{CameraView, Label};
use crate::structs::frame::{ColumnRequest, FrameStatus};
use crate::structs::light::SurfaceSample;
use crate::structs::manager::ManagerSignal;
use crate::structs::telemetry::TelemetryReport;
use crate::structs::world::ColumnPos;

// longest frame step fed to the scan planner, so a hitch or a paused game does not flood one frame
const MAX_FRAME_SECONDS: f64 = 0.1;
const BYTES_PER_MB: u64 = 1024 * 1024;

// the on/off state, the world lifecycle and the per-frame flow from scan requests to the draw list
pub struct LevelsController {
    max_columns_per_frame: usize,
    is_on: bool,
    last_status: Option<FrameStatus>,
    planner: ScanPlanner,
    grid: LabelGrid,
    builder: DrawListBuilder,
    telemetry: Telemetry,
    // the buffers core allocates once in new()
    core_bytes: u64,
    last_report: Option<TelemetryReport>,
    is_tag_limit_reported: bool,
}

impl LevelsController {
    pub fn new(config: LevelsConfig) -> Self {
        let config = config.sanitized();
        let planner = ScanPlanner::new(&config.scan);
        let grid = LabelGrid::new(config.scan.radius);
        let builder = DrawListBuilder::new(&config.scan, config.label, LightClassifier::new(config.light));
        let core_bytes = planner.resident_bytes() + grid.resident_bytes() + builder.resident_bytes();
        let mut telemetry = Telemetry::new(config.telemetry, config.memory_budget_mb as u64 * BYTES_PER_MB);
        telemetry.memory(core_bytes);
        Self {
            max_columns_per_frame: config.scan.max_columns_per_frame as usize,
            is_on: false,
            last_status: None,
            planner,
            grid,
            builder,
            telemetry,
            core_bytes,
            last_report: None,
            is_tag_limit_reported: false,
        }
    }

    pub fn signal(&mut self, signal: ManagerSignal) {
        match signal {
            ManagerSignal::TogglePressed => self.toggle(),
            ManagerSignal::LeaseLost | ManagerSignal::Disabled => {
                if self.is_on {
                    curia::info!("overlay stopped by the mod manager", { reason: format!("{signal:?}") });
                    self.set_on(false);
                }
            }
        }
    }

    // applies the settings that take effect at once; scan and memory budget size buffers in new() and keep
    // their values until the next start
    pub fn apply_config(&mut self, config: LevelsConfig) {
        let config = config.sanitized();
        self.builder.restyle(config.label, LightClassifier::new(config.light));
        self.telemetry.set_enabled(config.telemetry);
    }

    pub fn frame(&mut self, status: FrameStatus) -> &[ColumnRequest] {
        let requests: &[ColumnRequest] = if status.in_world && !status.has_player {
            &[]
        } else {
            let dt = self
                .last_status
                .map_or(0.0, |last| (status.time_seconds - last.time_seconds).clamp(0.0, MAX_FRAME_SECONDS));
            self.track_world(&status);
            self.last_status = Some(status);
            // a hidden HUD pauses the scan; the stored labels stay for when it comes back
            if self.is_on && !status.hud_hidden { self.planner.plan(status.feet, dt as f32) } else { &[] }
        };
        self.telemetry.frame(status.time_seconds, requests.len());
        if let Some(report) = self.telemetry.take_report(status.time_seconds) {
            Self::log_report(&report);
            self.last_report = Some(report);
        }
        requests
    }

    pub fn submit_column(&mut self, column: ColumnPos, samples: &[SurfaceSample]) {
        if self.is_on {
            self.grid.store(column, samples);
        }
    }

    // game_tags is the number of name tags the game itself draws this frame; camera is the view the frame is
    // drawn from, None when the shim has not read one. A hidden HUD draws nothing, since the game would not
    // show the labels
    pub fn draw_list(&mut self, game_tags: usize, camera: Option<CameraView>) -> &[Label] {
        let status = match self.last_status {
            Some(status) if self.is_on && !status.hud_hidden => status,
            _ => {
                self.telemetry.labels(0, 0);
                return &[];
            }
        };
        if game_tags > GAME_TAG_HEADROOM && !self.is_tag_limit_reported {
            self.is_tag_limit_reported = true;
            curia::warn!("many game name tags on screen, fewer labels are drawn, the nearest are kept", {
                game_tags: game_tags
            });
        }
        if camera.is_none() {
            self.telemetry.camera_missing();
        }
        let count = self.builder.build(&self.grid, status.feet, status.sky_darken, game_tags, camera).len();
        self.telemetry.labels(count, self.builder.culled());
        self.builder.labels()
    }

    pub fn label_rotation(&self) -> [f32; 4] {
        LabelRotation::for_yaw(self.last_status.map_or(0.0, |status| status.yaw))
    }

    pub fn label_scale(&self) -> f32 {
        self.builder.scale()
    }

    // outside_bytes is the memory the FFI layer and the shim hold for the overlay
    pub fn timing(&mut self, sample_us: f32, draw_us: f32, outside_bytes: u64) {
        self.telemetry.timing(sample_us, draw_us);
        self.telemetry.memory(self.core_bytes + outside_bytes);
    }

    pub fn last_report(&self) -> Option<TelemetryReport> {
        self.last_report
    }

    pub fn is_on(&self) -> bool {
        self.is_on
    }

    pub fn max_labels(&self) -> usize {
        self.builder.max_labels()
    }

    pub fn max_columns_per_frame(&self) -> usize {
        self.max_columns_per_frame
    }

    fn track_world(&mut self, status: &FrameStatus) {
        let Some(last) = self.last_status else {
            return;
        };
        if !status.in_world {
            if self.is_on {
                curia::info!("left the world, overlay off");
                self.set_on(false);
            }
            return;
        }
        if last.in_world && last.dimension != status.dimension {
            curia::info!("dimension changed, rescanning", { dimension: status.dimension });
            self.drop_labels();
        }
    }

    fn toggle(&mut self) {
        let in_world = self.last_status.is_some_and(|status| status.in_world);
        if !in_world {
            curia::info!("toggle ignored: no world loaded");
            return;
        }
        if self.last_status.is_some_and(|status| status.menu_open) {
            return;
        }
        self.set_on(!self.is_on);
    }

    fn set_on(&mut self, is_on: bool) {
        self.is_on = is_on;
        self.is_tag_limit_reported = false;
        self.drop_labels();
        curia::info!("overlay toggled", { on: is_on });
    }

    fn drop_labels(&mut self) {
        self.grid.clear();
        self.planner.reset();
    }

    fn log_report(report: &TelemetryReport) {
        let memory_mb = report.memory_bytes as f64 / BYTES_PER_MB as f64;
        let budget_mb = report.budget_bytes as f64 / BYTES_PER_MB as f64;
        curia::info!("telemetry", {
            fps: report.fps,
            max_frame_ms: report.max_frame_ms,
            labels: report.labels,
            culled: report.culled,
            draws_without_camera: report.draws_without_camera,
            columns_per_frame: report.columns_per_frame,
            sample_us: report.sample_us_avg,
            draw_us: report.draw_us_avg,
            memory_mb: memory_mb,
            budget_mb: budget_mb,
        });
        if report.is_over_budget() {
            curia::warn!("memory over budget", { memory_mb: memory_mb, budget_mb: budget_mb });
        }
    }
}

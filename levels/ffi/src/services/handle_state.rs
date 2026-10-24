use levels_core::config::MAX_COLUMN_SURFACES;
use levels_core::services::LevelsController;
use levels_core::structs::draw::CameraView;
use levels_core::structs::frame::FrameStatus;
use levels_core::structs::light::SurfaceSample;
use levels_core::structs::world::ColumnPos;

use crate::services::ManagerSession;
use crate::types::{LvColumnRequest, LvDrawView, LvLabel, LvScanView, LvSurfaceSample};

// everything behind the handle's lock: the controller, the manager session and the C-layout copies of the
// controller's output. The scan
// and label copies are separate buffers, so a held draw view survives lv_frame and lv_submit_column.
// Every buffer is sized once by new, so its address never changes and no call allocates
pub struct HandleState {
    controller: LevelsController,
    session: ManagerSession,
    columns: Vec<LvColumnRequest>,
    labels: Vec<LvLabel>,
    samples: Vec<SurfaceSample>,
}

impl HandleState {
    pub fn new(controller: LevelsController, session: ManagerSession) -> Self {
        let columns = Vec::with_capacity(controller.max_columns_per_frame());
        let labels = Vec::with_capacity(controller.max_labels());
        Self { controller, session, columns, labels, samples: Vec::with_capacity(MAX_COLUMN_SURFACES) }
    }

    // the manager's events go first, so a toggle or a lost lease shapes this frame; the lease follows the
    // overlay before the frame and after it, since leaving the world turns the overlay off inside it
    pub fn frame(&mut self, status: FrameStatus) -> LvScanView {
        self.session.drain(&mut self.controller);
        self.session.sync_lease(&mut self.controller);
        let requests = self.controller.frame(status);
        self.columns.clear();
        self.columns.extend(requests.iter().take(self.columns.capacity()).map(LvColumnRequest::from));
        self.session.sync_lease(&mut self.controller);
        LvScanView { columns: self.columns.as_ptr(), count: self.columns.len() as u32 }
    }

    pub fn submit_column(&mut self, column: ColumnPos, samples: &[LvSurfaceSample]) {
        self.samples.clear();
        self.samples.extend(samples.iter().take(self.samples.capacity()).map(SurfaceSample::from));
        self.controller.submit_column(column, &self.samples);
    }

    pub fn draw_list(&mut self, game_tags: usize, camera: Option<CameraView>) -> LvDrawView {
        let rotation = self.controller.label_rotation();
        let scale = self.controller.label_scale();
        let labels = self.controller.draw_list(game_tags, camera);
        self.labels.clear();
        self.labels.extend(labels.iter().take(self.labels.capacity()).map(LvLabel::from));
        LvDrawView { labels: self.labels.as_ptr(), count: self.labels.len() as u32, rotation, scale }
    }

    // shim_bytes is the memory the shim holds for the overlay; this state's own buffers are added to it
    pub fn timing(&mut self, sample_us: f32, draw_us: f32, shim_bytes: u64) {
        let own = self.columns.capacity() * std::mem::size_of::<LvColumnRequest>()
            + self.labels.capacity() * std::mem::size_of::<LvLabel>()
            + self.samples.capacity() * std::mem::size_of::<SurfaceSample>();
        self.controller.timing(sample_us, draw_us, shim_bytes.saturating_add(own as u64));
    }
}

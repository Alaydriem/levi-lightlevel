use std::path::Path;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

use amm_client::link::ApiTable;
use amm_core::errors::LinkError;
use levels_core::services::LevelsController;

use crate::services::{HandleState, ManagerSession};

// the opaque handle C++ holds
pub struct LvHandle {
    state: Mutex<HandleState>,
    is_poison_reported: AtomicBool,
}

impl LvHandle {
    // registers with the manager and starts core from the manager's config; an unreadable config gives the
    // defaults. `config_dir` is the mod's own LeviLamina config folder, an absolute path
    pub fn connect(table: ApiTable, config_dir: &Path) -> Result<Self, LinkError> {
        let session = ManagerSession::connect(table, config_dir)?;
        let controller = LevelsController::new(session.config().unwrap_or_default());
        let state = Mutex::new(HandleState::new(controller, session));
        Ok(Self { state, is_poison_reported: AtomicBool::new(false) })
    }

    // None when a panic poisoned the lock; the first such call logs it, later ones stay quiet
    pub fn with_state<R>(&self, body: impl FnOnce(&mut HandleState) -> R) -> Option<R> {
        match self.state.lock() {
            Ok(mut state) => Some(body(&mut state)),
            Err(_) => {
                if !self.is_poison_reported.swap(true, Ordering::Relaxed) {
                    curia::error!("levels controller lock is poisoned, the overlay stops until the game restarts");
                }
                None
            }
        }
    }
}

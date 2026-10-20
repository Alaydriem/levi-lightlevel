use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

use levels_core::services::LevelsController;

use crate::services::HandleState;

// the opaque handle C++ holds
pub struct LvHandle {
    state: Mutex<HandleState>,
    is_poison_reported: AtomicBool,
}

impl LvHandle {
    pub fn new(controller: LevelsController) -> Self {
        Self { state: Mutex::new(HandleState::new(controller)), is_poison_reported: AtomicBool::new(false) }
    }

    // None when a panic poisoned the lock; the first such call logs it, later ones stay quiet
    pub fn with_state<R>(&self, body: impl FnOnce(&mut HandleState) -> R) -> Option<R> {
        match self.state.lock() {
            Ok(mut state) => Some(body(&mut state)),
            Err(_) => {
                if !self.is_poison_reported.swap(true, Ordering::Relaxed) {
                    log::error!("levels controller lock is poisoned; the overlay stops until the game restarts");
                }
                None
            }
        }
    }
}

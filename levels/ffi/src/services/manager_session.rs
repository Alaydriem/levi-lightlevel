use std::path::Path;

use amm_client::link::{ApiTable, EventDrain, ManagerLink};
use amm_core::errors::LinkError;
use amm_core::structs::descriptor::Capability;
use amm_core::structs::event::{KeybindPhase, ModEvent};
use amm_core::structs::lease::AcquireOutcome;
use levels_core::config::LevelsConfig;
use levels_core::services::LevelsController;
use levels_core::structs::manager::ManagerSignal;

// the registration with the mod manager: turns its events into signals for core and keeps the WorldDraw
// lease held exactly while the overlay is on
pub struct ManagerSession {
    link: ManagerLink,
    // events of one poll, at most one drain buffer, sized once in connect
    pending: Vec<ModEvent>,
    is_world_draw_held: bool,
}

impl ManagerSession {
    const DESCRIPTOR: &'static str = include_str!("../../descriptor.hcl");
    // the index of the toggle keybind in the descriptor
    const TOGGLE_ACTION: u16 = 0;

    // `config_dir` is the mod's own LeviLamina config folder, an absolute path
    pub fn connect(table: ApiTable, config_dir: &Path) -> Result<Self, LinkError> {
        let link = ManagerLink::without_frame(table, Self::DESCRIPTOR, config_dir)?;
        Ok(Self { link, pending: Vec::with_capacity(EventDrain::CAPACITY), is_world_draw_held: false })
    }

    // the manager's checked config.hcl; None, with a warning, when it cannot be read
    pub fn config(&self) -> Option<LevelsConfig> {
        match self.link.config::<LevelsConfig>() {
            Ok(config) => Some(config),
            Err(error) => {
                curia::warn!("config not read", { error: error.to_string() });
                None
            }
        }
    }

    pub fn drain(&mut self, controller: &mut LevelsController) {
        self.pending.clear();
        self.pending.extend(self.link.events());
        for position in 0..self.pending.len() {
            let event = self.pending[position];
            self.handle(event, controller);
        }
    }

    pub fn sync_lease(&mut self, controller: &mut LevelsController) {
        let wants = controller.is_on();
        if wants == self.is_world_draw_held {
            return;
        }
        if !wants {
            self.link.release(Capability::WorldDraw);
            self.is_world_draw_held = false;
            return;
        }
        match self.link.acquire(Capability::WorldDraw) {
            Ok(AcquireOutcome::Granted | AcquireOutcome::Preempted) => self.is_world_draw_held = true,
            Ok(AcquireOutcome::Denied { .. }) => {
                curia::info!("world draw lease denied");
                controller.signal(ManagerSignal::LeaseLost);
            }
            Err(error) => {
                curia::warn!("world draw lease request failed", { error: error.to_string() });
                controller.signal(ManagerSignal::LeaseLost);
            }
        }
    }

    // a suspended lease is still held, so sync_lease gives it back once the overlay is off; a revoked lease and
    // the leases of a disabled mod are already gone
    fn handle(&mut self, event: ModEvent, controller: &mut LevelsController) {
        match event {
            ModEvent::Keybind { action: Self::TOGGLE_ACTION, phase: KeybindPhase::Pressed } => {
                controller.signal(ManagerSignal::TogglePressed);
            }
            ModEvent::Suspended { capability: Capability::WorldDraw, .. } => {
                controller.signal(ManagerSignal::LeaseLost);
            }
            ModEvent::Revoked { capability: Capability::WorldDraw, .. } => {
                self.is_world_draw_held = false;
                controller.signal(ManagerSignal::LeaseLost);
            }
            ModEvent::Disabled => {
                self.is_world_draw_held = false;
                controller.signal(ManagerSignal::Disabled);
            }
            ModEvent::SettingsChanged { .. } => {
                if let Some(config) = self.config() {
                    controller.apply_config(config);
                }
            }
            _ => {}
        }
    }
}

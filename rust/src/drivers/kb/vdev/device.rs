use enigo::Enigo;
use parking_lot::FairMutex;

use super::{emitter::vdev_emitter, key_codex::VdevKeys};

use crate::drivers::kb::{input::KbDriverInput, state::KbDriverState};

pub struct VdevDevice {
    pub enigo: FairMutex<enigo::Enigo>,
    pub key_codex: VdevKeys,
}

impl VdevDevice {
    pub fn init() -> VdevDevice {
        Self {
            enigo: FairMutex::new(Enigo::new()),
            key_codex: VdevKeys::init(),
        }
    }
    pub fn emitter(&self, kb_driver_state: KbDriverState, kb_driver_input: KbDriverInput) {
        vdev_emitter(self, kb_driver_state, kb_driver_input)
    }
}

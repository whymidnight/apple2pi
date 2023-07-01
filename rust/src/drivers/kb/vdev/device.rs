use std::sync::Arc;

use enigo::Enigo;

use super::{emitter::vdev_emitter, key_codex::VdevKeys};

use crate::drivers::kb::{input::KbDriverInput, state::KbDriverState};

pub struct VdevDevice {
    pub enigo: enigo::Enigo,
    pub key_codex: VdevKeys,
}

impl VdevDevice {
    pub fn init() -> VdevDevice {
        let mut enigo = Enigo::new();
        enigo.set_delay(0);
        Self {
            enigo,
            key_codex: VdevKeys::init(),
        }
    }
    pub fn emitter(&mut self, kb_driver_state: KbDriverState, kb_driver_input: KbDriverInput) {
        vdev_emitter(self, kb_driver_state, kb_driver_input)
    }
}

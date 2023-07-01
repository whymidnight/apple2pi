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
        Self {
            enigo: Enigo::new(),
            key_codex: VdevKeys::init(),
        }
    }
    pub fn emitter(&mut self, kb_driver_state: KbDriverState, kb_driver_input: KbDriverInput) {
        {
            // key up previous received input
            let previous_inputs = kb_driver_state.clone().chained_key_inputs;
            let previous_inputs_len = previous_inputs.len();
            if previous_inputs_len > 1 {
                let previous_input_idx = previous_inputs_len - 2;
                let previous_input = previous_inputs.get(previous_input_idx).unwrap().clone();
                vdev_emitter(self, kb_driver_state.clone(), previous_input.invert());
            }
        }

        vdev_emitter(self, kb_driver_state, kb_driver_input)
    }
}

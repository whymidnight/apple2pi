use enigo::Enigo;
use parking_lot::FairMutex;

use super::emitter::vdev_emitter;

use crate::drivers::kb::{input::KbDriverInput, state::KbDriverState};

pub struct VdevDevice {
    // REVIEW: should we wrap this is a mutex?
    // any/all invokations of `enigo` are
    // safeguarded already via mutexed KbDriverState
    // input processing ops. so would be redundant?
    pub enigo: FairMutex<enigo::Enigo>,
}

impl VdevDevice {
    pub fn init() -> VdevDevice {
        Self {
            enigo: FairMutex::new(Enigo::new()),
        }
    }
    pub fn emitter(&self, kb_driver_state: KbDriverState, kb_driver_input: KbDriverInput) {
        let enigo = &mut *(self.enigo.lock());
        vdev_emitter(enigo, kb_driver_state, kb_driver_input)
    }
}

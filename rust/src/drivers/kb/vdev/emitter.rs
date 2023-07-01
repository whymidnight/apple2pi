use enigo::{Enigo, Key, KeyboardControllable};

use crate::drivers::kb::{input::KbDriverInput, state::KbDriverState};

pub fn vdev_emitter(
    enigo: &mut Enigo,
    _kb_driver_state: KbDriverState,
    _kb_driver_input: KbDriverInput,
) {
    enigo.key_click(Key::Layout('a'))
}

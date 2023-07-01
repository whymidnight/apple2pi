use enigo::KeyboardControllable;

use crate::drivers::kb::{input::KbDriverInput, state::KbDriverState};

use super::device::VdevDevice;

pub fn vdev_emitter(
    vdev_device: &mut VdevDevice,
    _kb_driver_state: KbDriverState,
    kb_driver_input: KbDriverInput,
) {
    let enigo = &mut vdev_device.enigo;
    match kb_driver_input {
        KbDriverInput::KeyDown((_mod, _key, key_character)) => {
            let vdev_key_got = vdev_device
                .key_codex
                .clone()
                .get_vdev_key(key_character.clone());
            if vdev_key_got.is_none() {
                println!("\nUNABLE TO FIND KEY CHARACTER CODEX ::: {key_character}\n");
                return;
            }
            let vdev_key = vdev_key_got.unwrap();
            match vdev_key {
                super::key_codex::VdevKey::None(key) => enigo.key_down(key),
                super::key_codex::VdevKey::Remap(key) => enigo.key_down(key),
                super::key_codex::VdevKey::Macro(_key) => {
                    let _ = "unsupported";
                }
            }
        }
        KbDriverInput::KeyUp((_mod, _key, key_character)) => {
            let vdev_key_got = vdev_device
                .key_codex
                .clone()
                .get_vdev_key(key_character.clone());
            if vdev_key_got.is_none() {
                println!("\nUNABLE TO FIND KEY CHARACTER CODEX ::: {key_character}\n");
                return;
            }
            let vdev_key = vdev_key_got.unwrap();
            match vdev_key {
                super::key_codex::VdevKey::None(key) => enigo.key_up(key),
                super::key_codex::VdevKey::Remap(key) => enigo.key_up(key),
                super::key_codex::VdevKey::Macro(_key) => {
                    let _ = "unsupported";
                }
            }
        }
    }
}

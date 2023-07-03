use std::collections::HashMap;

use enigo::KeyboardControllable;

use crate::drivers::kb::{
    input::KbDriverInput, state::KbDriverState, vdev::key_codex::VdevKeyMacroSequenceEntrant,
};

use super::device::VdevDevice;

pub fn vdev_emitter(
    vdev_device: &mut VdevDevice,
    _kb_driver_state: KbDriverState,
    kb_driver_input: KbDriverInput,
) {
    let enigo = &mut vdev_device.enigo;
    match kb_driver_input {
        KbDriverInput::KeyDown((modifier, _key, key_character)) => {
            let vdev_key_got = vdev_device
                .key_codex
                .clone()
                .get_vdev_key(modifier, key_character.clone());
            if vdev_key_got.is_none() {
                println!("\nUNABLE TO FIND KEY CHARACTER CODEX ::: {key_character}\n");
                return;
            }
            let vdev_key = vdev_key_got.unwrap();
            match vdev_key {
                super::key_codex::VdevKey::None(key) => enigo.key_down(key),
                super::key_codex::VdevKey::Remap(key) => enigo.key_down(key),
                super::key_codex::VdevKey::Macro(macro_seq) => {
                    let mut trace: HashMap<String, VdevKeyMacroSequenceEntrant> = HashMap::new();
                    for k in macro_seq.keys() {
                        let entrant = macro_seq.get(k).unwrap();

                        match trace.clone().get(k) {
                            Some(e) => {
                                if e.until.is_some() {
                                    enigo.key_up(e.clone().into_vdev_key());
                                }
                                if entrant.until.is_some() || entrant.until_after.is_some() {
                                    if let Some(until) = entrant.until.clone() {
                                        trace.insert(until, entrant.clone());
                                    }
                                    if let Some(until_after) = entrant.until_after.clone() {
                                        trace.insert(until_after, entrant.clone());
                                    }
                                    enigo.key_down(entrant.clone().into_vdev_key());
                                } else {
                                    enigo.key_click(entrant.clone().into_vdev_key());
                                }
                                if e.until_after.is_some() {
                                    enigo.key_up(e.clone().into_vdev_key());
                                }
                            }
                            None => {
                                if entrant.until.is_some() || entrant.until_after.is_some() {
                                    if let Some(until) = entrant.until.clone() {
                                        trace.insert(until, entrant.clone());
                                    }
                                    if let Some(until_after) = entrant.until_after.clone() {
                                        trace.insert(until_after, entrant.clone());
                                    }
                                    enigo.key_down(entrant.clone().into_vdev_key());
                                } else {
                                    enigo.key_click(entrant.clone().into_vdev_key());
                                }
                            }
                        }
                    }
                }
            }
        }
        KbDriverInput::KeyUp((modifier, _key, key_character)) => {
            let vdev_key_got = vdev_device
                .key_codex
                .clone()
                .get_vdev_key(modifier, key_character.clone());
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
    println!("!!! emitted !!!");
}
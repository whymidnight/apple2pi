use alloc::vec;
use alloc::vec::Vec;

use usbd_human_interface_device::page::Keyboard;

use crate::drivers::shared::kb::Key;

#[cfg(feature = "no-std")]
use super::input::KbDriverInput;

#[derive(Clone)]
pub struct KeyState {
    active_keys: Vec<(u8, Vec<Keyboard>)>,
}

impl KeyState {
    pub fn init() -> Self {
        Self {
            active_keys: Vec::new(),
        }
    }

    pub fn handle_key_event(&mut self, scan_code: u8, key_event: (Key, KbDriverInput)) {
        let (key, input) = key_event;
        match input {
            KbDriverInput::KeyUpEvent(_) => {
                // check if offered `key_event` matches the last key down
                // then reset `active_keys` if true
                let last_key = self.active_keys.last();
                if let Some(last_key_code) = last_key {
                    if scan_code == last_key_code.0 {
                        self.active_keys = Vec::new()
                    }
                }
            }
            KbDriverInput::KeyDownEvent(_) => {
                self.active_keys = [
                    self.active_keys.clone(),
                    vec![(key.key.parse::<u8>().unwrap(), key.usb_hid)],
                ]
                .concat();
            }
            _ => {}
        }
    }

    pub fn get_active_keys(self) -> Vec<Vec<Keyboard>> {
        self.active_keys
            .iter()
            .map(|k| {
                for key in k.1.clone() {
                    let key_code: u8 = key.into();
                    defmt::info!("{}", key_code);
                }
                k.1.clone()
            })
            .collect()
    }
}

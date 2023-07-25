use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

use usbd_human_interface_device::page::Keyboard;

use crate::drivers::shared::kb::Key;

#[cfg(feature = "no-std")]
use super::input::KbDriverInput;

#[derive(Clone)]
pub struct KeyState {
    pub active_keys: Vec<(u8, Vec<Keyboard>)>,
}

impl KeyState {
    pub fn init() -> Self {
        Self {
            active_keys: Vec::new(),
        }
    }

    pub fn handle_key_event(&mut self, _scan_code: u8, key_event: (Key, KbDriverInput)) {
        let (key, input) = key_event;
        match input {
            KbDriverInput::KeyUpEvent(_) => {
                self.active_keys = vec![(0, vec![Keyboard::NoEventIndicated])];
            }
            KbDriverInput::KeyDownEvent(_) => {
                self.active_keys = vec![(key.key.parse::<u8>().unwrap(), key.usb_hid)];
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

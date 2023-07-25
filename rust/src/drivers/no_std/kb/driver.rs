use alloc::vec::Vec;
use usbd_hid::descriptor::KeyboardReport;
use usbd_human_interface_device::page::Keyboard;

use crate::drivers::shared::kb::*;

use super::{kbmap::KeyMap, state::KeyState};

#[derive(Clone)]
pub struct KbDriver {
    pub key_map: KeyMap,
    pub key_state: KeyState,
}

impl KeyboardDriver for KbDriver {
    fn init() -> KbDriver {
        KbDriver {
            key_map: KeyMap::init(),
            key_state: KeyState::init(),
        }
    }

    fn process_key_event(&mut self, event_payload: [u8; 3]) -> Option<Vec<KeyboardReport>> {
        let (layer, scan_code) = (event_payload[1], event_payload[2]);
        let key_event = self.key_map.clone().find_input(layer, scan_code);
        if key_event.is_none() {
            return None;
        }

        self.key_state
            .handle_key_event(scan_code, key_event.unwrap());

        let active_keys = &self.key_state.active_keys;
        let mut reports: Vec<KeyboardReport> = Vec::new();
        for keys in active_keys {
            let report = KeyboardReport {
                modifier: {
                    keys.1.iter().fold(0u8, |acc, k| {
                        let modded = match k {
                            Keyboard::LeftControl => 1u8 << 0u8,
                            Keyboard::LeftShift => 1 << 1,
                            Keyboard::LeftAlt => 1 << 2,
                            Keyboard::LeftGUI => 1 << 3,
                            Keyboard::RightControl => 1 << 4,
                            Keyboard::RightShift => 1 << 5,
                            Keyboard::RightAlt => 1 << 6,
                            Keyboard::RightGUI => 1 << 7,
                            _ => 0,
                        };

                        acc | modded
                    })
                },
                reserved: 0,
                leds: 0,
                keycodes: {
                    let mut keycodes: [u8; 6] = [0x0u8; 6];
                    for (idx, &key) in keys.1.iter().enumerate() {
                        if idx >= 6 {
                            break;
                        }
                        keycodes[idx] = key.into();
                    }

                    keycodes
                },
            };

            reports.push(report);
        }

        Some(reports)
    }

    fn hid_report(self) -> Vec<KeyboardReport> {
        let mut reports: Vec<KeyboardReport> = Vec::new();
        reports
    }
}

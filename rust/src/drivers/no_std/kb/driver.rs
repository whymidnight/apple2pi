use alloc::{vec, vec::Vec};
use usbd_hid::descriptor::KeyboardReport;
use usbd_human_interface_device::page::Keyboard;

use crate::drivers::shared::kb::*;

use super::{kbmap::KeyMap, state::KeyState};

#[derive(Clone)]
pub struct KbDriver {
    /// a { [scan_code: string]: { key: string, action: string } }.
    /// where `scan_code` is represented in hex
    /// where `key` is the supposed rendered sequence
    /// where `action` is the supposed recorded sequence
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
        // shove into KeyState
        let (layer, scan_code) = (event_payload[1], event_payload[2]);
        let key_event = self.key_map.clone().find_input(layer, scan_code);
        if key_event.is_none() {
            defmt::warn!("key event was none for :: {}", event_payload);
            return None;
        }

        self.key_state
            .handle_key_event(scan_code, key_event.unwrap());

        Some(self.clone().hid_report())
    }

    fn hid_report(self) -> Vec<KeyboardReport> {
        let mut reports: Vec<KeyboardReport> = Vec::new();
        let active_keys = self.key_state.get_active_keys();
        for keys in active_keys {
            let report = KeyboardReport {
                modifier: 0,
                reserved: 0,
                leds: 0,
                keycodes: {
                    let mut keycodes: [u8; 6] = [0x0u8; 6];
                    for (idx, &key) in keys.iter().enumerate() {
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

        reports
    }

    /*
    /// `lookup_scan_code` will search `self.key_map` for an existing
    /// key of `scan_code` (in hexadecimal form) and return existance.
    /// this will false-fail half the time due to key down scan codes
    /// being masqueraded by a bitmask operation. in such case, before
    /// returning `None` as if an invalid scan code, such falsey
    /// existance will forego a bitmask operation to determine key up
    /// scan code (which should **always** exist) - should this fail
    /// then return `None`.
    fn lookup_scan_code(self, scan_code: u8) -> (bool, Option<Key>) {
        let key_map = self.key_map;
        let mut scan_code_fmt = format!("0x{:2X}", scan_code);

        match key_map.clone().find_scan_code(scan_code_fmt.clone()) {
            Some(key) => (true, Some(key)),
            None => {
                let mask = /* (0x0 & MOD_FN) | */ scan_code & KEY_ASCII;
                scan_code_fmt = format!("0x{:2X}", mask);
                (false, key_map.find_scan_code(scan_code_fmt))
            }
        }
    }
    */

    /*
    pub fn emit_to_device(&mut self, state: Arc<KbDriverState>, input: KbDriverInput) {
        /*
        // let device = Arc::get_mut(&mut self.device).unwrap();
        let mut device = Arc::get_mut(&mut self.device).unwrap();
        device.emitter(state, input);

        //self.emit_state(state)
        */
    }

    pub fn emit_state(&self, state: Arc<KbDriverState>) {
        /*
        state
            .clone()
            .print(&|scan_code| self.clone().lookup_scan_code(scan_code.clone()))
            .clone()
        */
    }

    pub fn reset_device(&mut self) {
        /*
        let device = Arc::get_mut(&mut self.device).unwrap();
        device.clear()
        */
    }
    */
}

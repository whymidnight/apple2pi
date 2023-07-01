use std::{borrow::BorrowMut, sync::Arc};

use mio_serial::SerialStream;

use crate::errors::A2PiError;

use super::{
    handshake::handshake,
    input::{KbDriverInput, KEY_ASCII, MOD_FN},
    kbmap::{Key, KeyMap},
    state::KbDriverState,
    vdev::device::VdevDevice,
};

#[derive(Clone)]
pub struct KbDriver {
    /// a { [scan_code: string]: { key: string, action: string } }.
    /// where `scan_code` is represented in hex
    /// where `key` is the supposed rendered sequence
    /// where `action` is the supposed recorded sequence
    pub key_map: KeyMap,
    pub device: Arc<VdevDevice>,
}

impl KbDriver {
    pub fn init(keymap_file: Option<String>) -> KbDriver {
        KbDriver {
            key_map: KeyMap::open(keymap_file),
            device: Arc::new(VdevDevice::init()),
        }
    }

    pub fn handshake(&self, conn: &mut SerialStream, payload: &[u8]) -> Result<(), A2PiError> {
        handshake(conn, payload)
    }

    /// `lookup_scan_code` will search `self.key_map` for an existing
    /// key of `scan_code` (in hexadecimal form) and return existance.
    /// this will false-fail half the time due to key down scan codes
    /// being masqueraded by a bitmask operation. in such case, before
    /// returning `None` as if an invalid scan code, such falsey
    /// existance will forego a bitmask operation to determine key up
    /// scan code (which should **always** exist) - should this fail
    /// then return `None`.
    pub fn lookup_scan_code(self, scan_code: u8) -> (bool, Option<Key>) {
        let key_map = self.key_map;
        let mut scan_code_fmt = format!("0x{}", hex::encode_upper([scan_code]));

        println!("{scan_code_fmt}");
        match key_map.clone().find_scan_code(scan_code_fmt.clone()) {
            Some(key) => (true, Some(key)),
            None => {
                let mask = /* (0x0 & MOD_FN) | */ scan_code & KEY_ASCII;
                scan_code_fmt = format!("0x{}", hex::encode_upper([mask]));
                println!("{scan_code_fmt}");
                println!("FINDING SCAN CODE {:?}", scan_code_fmt);
                (false, key_map.find_scan_code(scan_code_fmt))
            }
        }
    }

    pub fn emit_to_device(&mut self, state: KbDriverState, input: KbDriverInput) {
        let device = self.device.clone();
        device.emitter(state.clone(), input);

        self.emit_state(state)
    }

    pub fn emit_state(&self, state: KbDriverState) {
        state.print(&|scan_code| self.clone().lookup_scan_code(scan_code))
    }
}

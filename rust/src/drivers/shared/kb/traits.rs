use super::Key;

#[cfg(feature = "no-std")]
use crate::drivers::no_std::kb::input::KbDriverInput;

#[cfg(feature = "no-std")]
use usbd_hid::descriptor::KeyboardReport;

#[cfg(feature = "no-std")]
use alloc::vec::Vec;

pub trait KeyboardDriver {
    /*
    #[cfg(feature = "std")]
    fn init(keymap: Option<String>) -> Self;
    */
    #[cfg(feature = "no-std")]
    fn init() -> Self;
    #[cfg(feature = "no-std")]
    fn process_key_event(&mut self, event_payload: [u8; 3]) -> Option<Vec<KeyboardReport>>;
    #[cfg(feature = "no-std")]
    fn hid_report(self) -> Vec<KeyboardReport>;
}

pub trait KeyboardKeyMap {
    #[cfg(feature = "no-std")]
    fn find_input(self, layer: u8, scan_code: u8) -> Option<(Key, KbDriverInput)>;
    /*
    #[cfg(feature = "std")]
    fn find_scan_code(self, scan_code: String) -> Option<Key>;
    */
}

mod hid;

use alloc::format;

use hashbrown::HashMap;
use usbd_human_interface_device::page::Keyboard;

use crate::drivers::shared::kb::{Key, KeyboardKeyMap};

use alloc::string::*;

use alloc::vec::*;

use self::hid::hoist_hid_keyboard_map;

#[cfg(feature = "no-std")]
use super::input::KbDriverInput;

pub type LayoutKeyWithHID = (&'static str, (u8, u8, Vec<Keyboard>));
pub type LayoutKey = (&'static str, (u8, u8));
pub type Layout = [(&'static str, [LayoutKey; 128]); 1];
pub type LayoutWithHID = Vec<(&'static str, Vec<LayoutKeyWithHID>)>;
pub type LayoutFreeWithHID = Vec<LayoutKeyWithHID>;

#[derive(Clone)]
pub struct KeyMap {
    pub layout: HashMap<String, LayoutFreeWithHID>,
}

#[cfg(feature = "no-std")]
impl KeyMap {
    pub fn init() -> KeyMap {
        let hid = hoist_hid_keyboard_map();

        let mut layout = HashMap::new();
        {
            let fmt = hid
                .iter()
                .map(|layer| {
                    let layer_mask = layer.0;
                    let layer_keys: LayoutFreeWithHID = layer
                        .1
                        .iter()
                        .map(|layer_key| {
                            (layer_key.0, (layer_key.1 .0, layer_key.1 .1, Vec::new()))
                        })
                        .collect();
                    (layer_mask.to_string(), layer_keys)
                })
                .collect::<Vec<(String, LayoutFreeWithHID)>>();
            fmt.iter().for_each(|l| {
                layout.insert(l.0.clone(), l.1.clone());
            });
        }
        Self { layout }
    }
}

impl KeyboardKeyMap for KeyMap {
    fn find_input(self, layer: u8, scan_code: u8) -> Option<(Key, KbDriverInput)> {
        defmt::info!("finding input for {} {}", layer, scan_code);

        match self.layout.get(&format!("{layer}")) {
            Some(layout) => {
                let input_found: Option<(Key, KbDriverInput)> =
                    layout.iter().fold(None, |acc, layout_key| {
                        if acc.is_some() {
                            return acc;
                        }

                        let (key_down, key_up, usb_hid) =
                            (layout_key.1 .0, layout_key.1 .1, layout_key.1 .2.clone());

                        match scan_code {
                            _ if key_down == scan_code => Some((
                                Key::define(key_down, key_up, usb_hid),
                                KbDriverInput::KeyDown,
                            )),
                            _ if key_up == scan_code => {
                                Some((Key::define(key_down, key_up, usb_hid), KbDriverInput::KeyUp))
                            }
                            _ => None,
                        }
                    });
                match input_found {
                    Some(input) => {
                        let (input_key, driver_input) = input;
                        match driver_input {
                            KbDriverInput::KeyUp => Some((
                                input_key.clone(),
                                KbDriverInput::from_key(layer, input_key, driver_input),
                            )),
                            KbDriverInput::KeyDown => Some((
                                input_key.clone(),
                                KbDriverInput::from_key(layer, input_key, driver_input),
                            )),
                            _ => None,
                        }
                    }
                    None => None,
                }
            }
            None => None,
        }
    }
}

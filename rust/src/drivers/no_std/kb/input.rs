use crate::drivers::shared::kb::Key;
use alloc::string::*;

// pub const MOD_FN: u8 = 0x80u8;
pub const KEY_ASCII: u8 = 0x7Fu8;

#[derive(Clone)]
#[repr(u8)]
pub enum Modifiers {
    Bare(u8) = 0x0u8,
    OpenApple(u8) = 0x40,
    ClosedApple(u8) = 0x80,
    OpenClosedApple(u8) = 0xc1,
}

impl Modifiers {
    pub fn get(modifier_scan_code: u8) -> Option<Modifiers> {
        match modifier_scan_code {
            bare if modifier_scan_code == 0x0u8 => Some(Modifiers::Bare(bare)),
            open if modifier_scan_code == 0x40u8 => Some(Modifiers::OpenApple(open)),
            closed if modifier_scan_code == 0x80u8 => Some(Modifiers::ClosedApple(closed)),
            open_closed if modifier_scan_code == 0xC0u8 => {
                Some(Modifiers::OpenClosedApple(open_closed))
            }
            _ => None,
        }
    }
    pub fn inner(self) -> u8 {
        match self {
            Self::Bare(bare) => bare,
            Self::OpenApple(open) => open,
            Self::ClosedApple(closed) => closed,
            Self::OpenClosedApple(open_closed) => open_closed,
        }
    }
    pub fn outer_as_string(self) -> String {
        match self {
            Self::Bare(_) => "bare".to_string(),
            Self::OpenApple(_) => "open".to_string(),
            Self::ClosedApple(_) => "closed".to_string(),
            Self::OpenClosedApple(_) => "open_closed".to_string(),
        }
    }
}

impl PartialEq for Modifiers {
    fn eq(&self, other: &Self) -> bool {
        let cmp_inner = other.clone().inner();
        match self {
            Self::Bare(bare) => bare == &cmp_inner,
            Self::OpenApple(open) => open == &cmp_inner,
            Self::ClosedApple(closed) => closed == &cmp_inner,
            Self::OpenClosedApple(open_closed) => open_closed == &cmp_inner,
        }
    }
}

#[derive(Clone)]
pub enum KbDriverInput {
    KeyDownEvent((Modifiers, u8, String)),
    KeyUpEvent((Modifiers, u8, String)),
    KeyDown,
    KeyUp,
}

impl KbDriverInput {
    pub fn from_key(modifier_scan_code: u8, key: Key, input: Self) -> Self {
        let modifier_got = Modifiers::get(modifier_scan_code);

        match modifier_got {
            Some(modifier) => match input {
                Self::KeyUp => {
                    Self::KeyUpEvent((modifier, key.action.parse().unwrap(), "".to_string()))
                }
                Self::KeyDown => {
                    Self::KeyDownEvent((modifier, key.action.parse().unwrap(), "".to_string()))
                }
                _ => Self::KeyDownEvent((modifier, key.action.parse().unwrap(), "".to_string())),
            },
            None => match input {
                Self::KeyUp => Self::KeyUpEvent((
                    Modifiers::Bare(0),
                    key.action.parse().unwrap(),
                    "".to_string(),
                )),
                Self::KeyDown => Self::KeyDownEvent((
                    Modifiers::Bare(0),
                    key.action.parse().unwrap(),
                    "".to_string(),
                )),
                _ => Self::KeyDownEvent((
                    Modifiers::Bare(0),
                    key.action.parse().unwrap(),
                    "".to_string(),
                )),
            },
        }
    }
}

pub const A2PI_DESCRIPTOR: &[u8] = &[
    0x05, 0x01, // Usage Page (Generic Desktop Ctrls)
    0x09, 0x06, // Usage (Keyboard)
    0xA1, 0x01, // Collection (Application)
    // Modifier Keys
    0x05, 0x07, //   Usage Page (Kbrd/Keypad)
    0x19, 0xE0, //   Usage Minimum (0xE0)
    0x29, 0xE7, //   Usage Maximum (0xE7)
    0x15, 0x00, //   Logical Minimum (0)
    0x25, 0x01, //   Logical Maximum (1)
    0x95, 0x08, //   Report Count (8)
    0x75, 0x01, //   Report Size (1)
    0x81, 0x02, //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    // Reserved Byte
    0x95, 0x01, //   Report Count (1)
    0x75, 0x08, //   Report Size (8)
    0x81, 0x01, //   Input (Const,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    // LEDs
    0x05, 0x08, //   Usage Page (LEDs)
    0x19, 0x01, //   Usage Minimum (Num Lock)
    0x29, 0x05, //   Usage Maximum (Kana)
    0x95, 0x05, //   Report Count (5)
    0x75, 0x01, //   Report Size (1)
    0x91,
    0x02, //   Output (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    // LED Padding
    0x95, 0x01, //   Report Count (1)
    0x75, 0x03, //   Report Size (3)
    0x91,
    0x01, //   Output (Const,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    // Keycodes
    0x05, 0x07, //   Usage Page (Kbrd/Keypad)
    0x19, 0x00, //   Usage Minimum (0x00)
    0x29, 0xDD, //   Usage Maximum (0xDD) - TODO - double check this
    0x15, 0x00, //   Logical Minimum (0)
    0x26, 0xFF,
    0x00, //   Logical Maximum (255) - TOOD - double check max and trailing 0x00 byte
    0x95, 0x06, //   Report Count (6)
    0x75, 0x08, //   Report Size (8)
    0x81, 0x00, //   Input (Data,Array,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0xC0, // End Collection
];

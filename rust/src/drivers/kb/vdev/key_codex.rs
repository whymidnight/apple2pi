use std::collections::HashMap;

use enigo::Key;

#[derive(Clone)]
pub struct VdevKeyMacroSequenceEntrant {
    to: String,
    until: String,
}

impl VdevKeyMacroSequenceEntrant {
    pub fn from_spec() {}
}

/// see `${PROJECT_DIR}/rust/spec.configure_keyboard_layout.md` for shape.
pub type VdevKeyMacro = HashMap<String, VdevKeyMacroSequenceEntrant>;

#[derive(Clone)]
pub enum VdevKey {
    None(Key),
    Remap(Key),
    Macro(VdevKeyMacro),
}

#[derive(Clone)]
pub struct VdevKeys {
    pub codex: HashMap<String, VdevKey>,
}

impl VdevKeys {
    pub fn init() -> VdevKeys {
        // TODO(@dom): apply layers/remaps here
        Self {
            codex: HashMap::from([
                /* ALPA-NUMERIC */
                ("a".to_string(), VdevKey::None(Key::Layout('a'))),
                ("A".to_string(), VdevKey::None(Key::Layout('A'))),
                ("b".to_string(), VdevKey::None(Key::Layout('b'))),
                ("B".to_string(), VdevKey::None(Key::Layout('B'))),
                ("c".to_string(), VdevKey::None(Key::Layout('c'))),
                ("C".to_string(), VdevKey::None(Key::Layout('C'))),
                ("d".to_string(), VdevKey::None(Key::Layout('d'))),
                ("D".to_string(), VdevKey::None(Key::Layout('D'))),
                ("e".to_string(), VdevKey::None(Key::Layout('e'))),
                ("E".to_string(), VdevKey::None(Key::Layout('E'))),
                ("f".to_string(), VdevKey::None(Key::Layout('f'))),
                ("F".to_string(), VdevKey::None(Key::Layout('F'))),
                ("g".to_string(), VdevKey::None(Key::Layout('g'))),
                ("G".to_string(), VdevKey::None(Key::Layout('G'))),
                ("h".to_string(), VdevKey::None(Key::Layout('h'))),
                ("H".to_string(), VdevKey::None(Key::Layout('H'))),
                ("i".to_string(), VdevKey::None(Key::Layout('i'))),
                ("I".to_string(), VdevKey::None(Key::Layout('I'))),
                ("j".to_string(), VdevKey::None(Key::Layout('j'))),
                ("J".to_string(), VdevKey::None(Key::Layout('J'))),
                ("k".to_string(), VdevKey::None(Key::Layout('k'))),
                ("K".to_string(), VdevKey::None(Key::Layout('K'))),
                ("l".to_string(), VdevKey::None(Key::Layout('l'))),
                ("L".to_string(), VdevKey::None(Key::Layout('L'))),
                ("m".to_string(), VdevKey::None(Key::Layout('m'))),
                ("M".to_string(), VdevKey::None(Key::Layout('M'))),
                ("n".to_string(), VdevKey::None(Key::Layout('n'))),
                ("N".to_string(), VdevKey::None(Key::Layout('N'))),
                ("o".to_string(), VdevKey::None(Key::Layout('o'))),
                ("O".to_string(), VdevKey::None(Key::Layout('O'))),
                ("p".to_string(), VdevKey::None(Key::Layout('p'))),
                ("P".to_string(), VdevKey::None(Key::Layout('P'))),
                ("q".to_string(), VdevKey::None(Key::Layout('q'))),
                ("Q".to_string(), VdevKey::None(Key::Layout('Q'))),
                ("r".to_string(), VdevKey::None(Key::Layout('r'))),
                ("R".to_string(), VdevKey::None(Key::Layout('R'))),
                ("s".to_string(), VdevKey::None(Key::Layout('s'))),
                ("S".to_string(), VdevKey::None(Key::Layout('S'))),
                ("t".to_string(), VdevKey::None(Key::Layout('t'))),
                ("T".to_string(), VdevKey::None(Key::Layout('T'))),
                ("u".to_string(), VdevKey::None(Key::Layout('u'))),
                ("U".to_string(), VdevKey::None(Key::Layout('U'))),
                ("v".to_string(), VdevKey::None(Key::Layout('v'))),
                ("V".to_string(), VdevKey::None(Key::Layout('V'))),
                ("w".to_string(), VdevKey::None(Key::Layout('w'))),
                ("W".to_string(), VdevKey::None(Key::Layout('W'))),
                ("x".to_string(), VdevKey::None(Key::Layout('x'))),
                ("X".to_string(), VdevKey::None(Key::Layout('X'))),
                ("y".to_string(), VdevKey::None(Key::Layout('y'))),
                ("Y".to_string(), VdevKey::None(Key::Layout('Y'))),
                ("z".to_string(), VdevKey::None(Key::Layout('z'))),
                ("Z".to_string(), VdevKey::None(Key::Layout('Z'))),
                /* NUMERIC */
                /* CONTROLS */
                ("Enter".to_string(), VdevKey::None(Key::Raw(0x0D))),
                ("BS".to_string(), VdevKey::None(Key::Backspace)),
                ("ESCAPE".to_string(), VdevKey::None(Key::Escape)),
                ("CTRL-I".to_string(), VdevKey::None(Key::Tab)),
            ]),
        }
    }
    pub fn get_vdev_key(self, key_character: String) -> Option<VdevKey> {
        self.codex.get(&key_character).cloned()
    }
}

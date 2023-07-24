use alloc::vec::Vec;
use usbd_human_interface_device::page::Keyboard;

/*
pub type Layout = [
    (&'static str, [ // layer
     (
         &'static str, // key up scan code
         (u8, u8, Keyboard) // tuple of (key down, key up, usbd hid)
     ); 6 // scan codes
    ]);
3];
*/
pub type LayoutKeyWithHID = (&'static str, (u8, u8, Vec<Keyboard>));
pub type LayoutKey = (&'static str, (u8, u8));
pub type Layout = [(&'static str, [LayoutKey; 128]); 1];
pub type LayoutWithHID = Vec<(&'static str, Vec<LayoutKeyWithHID>)>;
pub type LayoutFreeWithHID = Vec<LayoutKeyWithHID>;

#[cfg(feature = "layout-iso")]
pub static LAYOUT: Layout = [
    (
        "0x00",
        [
            (
                "0x00",
                (0x80, 0x00), // CTRL-@ :: CTRL+SHIFT+KEY_2
            ),
            (
                "0x01",
                (0x81, 0x01), // CTRL-A :: CTRL+KEY_A
            ),
            (
                "0x02",
                (0x82, 0x02), // CTRL-B :: CTRL+KEY_B
            ),
            (
                "0x03",
                (0x83, 0x03), // CTRL-C :: CTRL+KEY_C
            ),
            (
                "0x04",
                (0x84, 0x04), // CTRL-D :: CTRL+KEY_D
            ),
            (
                "0x05",
                (0x85, 0x05), // CTRL-E :: CTRL+KEY_E
            ),
            (
                "0x06",
                (0x86, 0x06), // CTRL-F :: CTRL+KEY_F
            ),
            (
                "0x07",
                (0x87, 0x07), // CTRL-G :: CTRL+KEY_G
            ),
            (
                "0x08",
                (0x88, 0x08), // CTRL-H :: KEY_LEFT
            ),
            (
                "0x09",
                (0x89, 0x09), // CTRL-I :: KEY_TAB
            ),
            (
                "0x0a",
                (0x8a, 0x0a), // CTRL-J :: KEY_DOWN
            ),
            (
                "0x0b",
                (0x8b, 0x0b), // CTRL-K :: KEY_UP
            ),
            (
                "0x0c",
                (0x8c, 0x0c), // CTRL-L :: CTRL+KEY_L
            ),
            (
                "0x0d",
                (0x8d, 0x0d), // Enter :: KEY_ENTER
            ),
            (
                "0x0e",
                (0x8e, 0x0e), // CTRL-N :: CTRL+KEY_N
            ),
            (
                "0x0f",
                (0x8f, 0x0f), // CTRL-O :: CTRL+KEY_O
            ),
            (
                "0x10",
                (0x90, 0x10), // CTRL-P :: CTRL+KEY_P
            ),
            (
                "0x11",
                (0x91, 0x11), // CTRL-Q :: CTRL+KEY_Q
            ),
            (
                "0x12",
                (0x92, 0x12), // CTRL-R :: CTRL+KEY_R
            ),
            (
                "0x13",
                (0x93, 0x13), // CTRL-S :: CTRL+KEY_S
            ),
            (
                "0x14",
                (0x94, 0x14), // CTRL-T :: CTRL+KEY_T
            ),
            (
                "0x15",
                (0x95, 0x15), // CTRL-U :: KEY_RIGHT
            ),
            (
                "0x16",
                (0x96, 0x16), // CTRL-V :: CTRL+KEY_V
            ),
            (
                "0x17",
                (0x97, 0x17), // CTRL-W :: CTRL+KEY_W
            ),
            (
                "0x18",
                (0x98, 0x18), // CTRL-X :: CTRL+KEY_X
            ),
            (
                "0x19",
                (0x99, 0x19), // CTRL-Y :: CTRL+KEY_Y
            ),
            (
                "0x1a",
                (0x9a, 0x1a), // CTRL-Z :: CTRL+KEY_Z
            ),
            (
                "0x1b",
                (0x9b, 0x1b), // ESCAPE :: KEY_ESC
            ),
            (
                "0x1c",
                (0x9c, 0x1c), // CTRL-\ :: CTRL+KEY_BACKSLASH
            ),
            (
                "0x1d",
                (0x9d, 0x1d), // CTRL-] :: CTRL+KEY_RIGHTBRACE
            ),
            (
                "0x1e",
                (0x9e, 0x1e), // CTRL-6 :: CTRL+KEY_6
            ),
            (
                "0x1f",
                (0x9f, 0x1f), // CTRL-- :: CTRL+KEY_MINUS
            ),
            (
                "0x20",
                (0xa0, 0x20), // ' ' :: KEY_SPACE
            ),
            (
                "0x21",
                (0xa1, 0x21), // ! :: SHIFT+KEY_1
            ),
            (
                "0x22",
                (0xa2, 0x22), // " :: SHIFT+KEY_APOSTROPHE
            ),
            (
                "0x23",
                (0xa3, 0x23), // # :: SHIFT+KEY_3
            ),
            (
                "0x24",
                (0xa4, 0x24), // $ :: SHIFT+KEY_4
            ),
            (
                "0x25",
                (0xa5, 0x25), // % :: SHIFT+KEY_5
            ),
            (
                "0x26",
                (0xa6, 0x26), // & :: SHIFT+KEY_7
            ),
            (
                "0x27",
                (0xa7, 0x27), // ' :: KEY_APOSTROPHE
            ),
            (
                "0x28",
                (0xa8, 0x28), // ( :: SHIFT+KEY_9
            ),
            (
                "0x29",
                (0xa9, 0x29), // ) :: SHIFT+KEY_0
            ),
            (
                "0x2a",
                (0xaa, 0x2a), // * :: SHIFT+KEY_8
            ),
            (
                "0x2b",
                (0xab, 0x2b), // + :: SHIFT+KEY_EQUAL
            ),
            (
                "0x2c",
                (0xac, 0x2c), // , :: KEY_COMMA
            ),
            (
                "0x2d",
                (0xad, 0x2d), // - :: KEY_MINUS
            ),
            (
                "0x2e",
                (0xae, 0x2e), // . :: KEY_DOT
            ),
            (
                "0x2f",
                (0xaf, 0x2f), // / :: KEY_SLASH
            ),
            (
                "0x30",
                (0xb0, 0x30), // 0 :: KEY_0
            ),
            (
                "0x31",
                (0xb1, 0x31), // 1 :: KEY_1
            ),
            (
                "0x32",
                (0xb2, 0x32), // 2 :: KEY_2
            ),
            (
                "0x33",
                (0xb3, 0x33), // 3 :: KEY_3
            ),
            (
                "0x34",
                (0xb4, 0x34), // 4 :: KEY_4
            ),
            (
                "0x35",
                (0xb5, 0x35), // 5 :: KEY_5
            ),
            (
                "0x36",
                (0xb6, 0x36), // 6 :: KEY_6
            ),
            (
                "0x37",
                (0xb7, 0x37), // 7 :: KEY_7
            ),
            (
                "0x38",
                (0xb8, 0x38), // 8 :: KEY_8
            ),
            (
                "0x39",
                (0xb9, 0x39), // 9 :: KEY_9
            ),
            (
                "0x3a",
                (0xba, 0x3a), // : :: SHIFT+KEY_SEMICOLON
            ),
            (
                "0x3b",
                (0xbb, 0x3b), // ; :: KEY_SEMICOLON
            ),
            (
                "0x3c",
                (0xbc, 0x3c), // < :: SHIFT+KEY_COMMA
            ),
            (
                "0x3d",
                (0xbd, 0x3d), // = :: KEY_EQUAL
            ),
            (
                "0x3e",
                (0xbe, 0x3e), // > :: SHIFT+KEY_DOT
            ),
            (
                "0x3f",
                (0xbf, 0x3f), // ? :: SHIFT+KEY_SLASH
            ),
            (
                "0x40",
                (0xc0, 0x40), // @ :: SHIFT+KEY_2
            ),
            (
                "0x41",
                (0xc1, 0x41), // A :: SHIFT+KEY_A
            ),
            (
                "0x42",
                (0xc2, 0x42), // B :: SHIFT+KEY_B
            ),
            (
                "0x43",
                (0xc3, 0x43), // C :: SHIFT+KEY_C
            ),
            (
                "0x44",
                (0xc4, 0x44), // D :: SHIFT+KEY_D
            ),
            (
                "0x45",
                (0xc5, 0x45), // E :: SHIFT+KEY_E
            ),
            (
                "0x46",
                (0xc6, 0x46), // F :: SHIFT+KEY_F
            ),
            (
                "0x47",
                (0xc7, 0x47), // G :: SHIFT+KEY_G
            ),
            (
                "0x48",
                (0xc8, 0x48), // H :: SHIFT+KEY_H
            ),
            (
                "0x49",
                (0xc9, 0x49), // I :: SHIFT+KEY_I
            ),
            (
                "0x4a",
                (0xca, 0x4a), // J :: SHIFT+KEY_J
            ),
            (
                "0x4b",
                (0xcb, 0x4b), // K :: SHIFT+KEY_K
            ),
            (
                "0x4c",
                (0xcc, 0x4c), // L :: SHIFT+KEY_L
            ),
            (
                "0x4d",
                (0xcd, 0x4d), // M :: SHIFT+KEY_M
            ),
            (
                "0x4e",
                (0xce, 0x4e), // N :: SHIFT+KEY_N
            ),
            (
                "0x4f",
                (0xcf, 0x4f), // O :: SHIFT+KEY_O
            ),
            (
                "0x50",
                (0xd0, 0x50), // P :: SHIFT+KEY_P
            ),
            (
                "0x51",
                (0xd1, 0x51), // Q :: SHIFT+KEY_Q
            ),
            (
                "0x52",
                (0xd2, 0x52), // R :: SHIFT+KEY_R
            ),
            (
                "0x53",
                (0xd3, 0x53), // S :: SHIFT+KEY_S
            ),
            (
                "0x54",
                (0xd4, 0x54), // T :: SHIFT+KEY_T
            ),
            (
                "0x55",
                (0xd5, 0x55), // U :: SHIFT+KEY_U
            ),
            (
                "0x56",
                (0xd6, 0x56), // V :: SHIFT+KEY_V
            ),
            (
                "0x57",
                (0xd7, 0x57), // W :: SHIFT+KEY_W
            ),
            (
                "0x58",
                (0xd8, 0x58), // X :: SHIFT+KEY_X
            ),
            (
                "0x59",
                (0xd9, 0x59), // Y :: SHIFT+KEY_Y
            ),
            (
                "0x5a",
                (0xda, 0x5a), // Z :: SHIFT+KEY_Z
            ),
            (
                "0x5b",
                (0xdb, 0x5b), // [ :: KEY_LEFTBRACE
            ),
            (
                "0x5c",
                (0xdc, 0x5c), // \ :: KEY_BACKSLASH
            ),
            (
                "0x5d",
                (0xdd, 0x5d), // ] :: KEY_RIGHTBRACE
            ),
            (
                "0x5e",
                (0xde, 0x5e), // ^ :: SHIFT+KEY_6
            ),
            (
                "0x5f",
                (0xdf, 0x5f), // _ :: SHIFT+KEY_MINUS
            ),
            (
                "0x60",
                (0xe0, 0x60), // ` :: KEY_GRAVE
            ),
            (
                "0x61",
                (0xe1, 0x61), // a :: KEY_A
            ),
            (
                "0x62",
                (0xe2, 0x62), // b :: KEY_B
            ),
            (
                "0x63",
                (0xe3, 0x63), // c :: KEY_C
            ),
            (
                "0x64",
                (0xe4, 0x64), // d :: KEY_D
            ),
            (
                "0x65",
                (0xe5, 0x65), // e :: KEY_E
            ),
            (
                "0x66",
                (0xe6, 0x66), // f :: KEY_F
            ),
            (
                "0x67",
                (0xe7, 0x67), // g :: KEY_G
            ),
            (
                "0x68",
                (0xe8, 0x68), // h :: KEY_H
            ),
            (
                "0x69",
                (0xe9, 0x69), // i :: KEY_I
            ),
            (
                "0x6a",
                (0xea, 0x6a), // j :: KEY_J
            ),
            (
                "0x6b",
                (0xeb, 0x6b), // k :: KEY_K
            ),
            (
                "0x6c",
                (0xec, 0x6c), // l :: KEY_L
            ),
            (
                "0x6d",
                (0xed, 0x6d), // m :: KEY_M
            ),
            (
                "0x6e",
                (0xee, 0x6e), // n :: KEY_N
            ),
            (
                "0x6f",
                (0xef, 0x6f), // o :: KEY_O
            ),
            (
                "0x70",
                (0xf0, 0x70), // p :: KEY_P
            ),
            (
                "0x71",
                (0xf1, 0x71), // q :: KEY_Q
            ),
            (
                "0x72",
                (0xf2, 0x72), // r :: KEY_R
            ),
            (
                "0x73",
                (0xf3, 0x73), // s :: KEY_S
            ),
            (
                "0x74",
                (0xf4, 0x74), // t :: KEY_T
            ),
            (
                "0x75",
                (0xf5, 0x75), // u :: KEY_U
            ),
            (
                "0x76",
                (0xf6, 0x76), // v :: KEY_V
            ),
            (
                "0x77",
                (0xf7, 0x77), // w :: KEY_W
            ),
            (
                "0x78",
                (0xf8, 0x78), // x :: KEY_X
            ),
            (
                "0x79",
                (0xf9, 0x79), // y :: KEY_Y
            ),
            (
                "0x7a",
                (0xfa, 0x7a), // z :: KEY_Z
            ),
            (
                "0x7b",
                (0xfb, 0x7b), // { :: SHIFT+KEY_LEFTBRACE
            ),
            (
                "0x7c",
                (0xfc, 0x7c), // | :: SHIFT+KEY_BACKSLASH
            ),
            (
                "0x7d",
                (0xfd, 0x7d), // } :: SHIFT+KEY_RIGHTBRACE
            ),
            (
                "0x7e",
                (0xfe, 0x7e), // ~ :: SHIFT+KEY_GRAVE
            ),
            (
                "0x7f",
                (0xff, 0x7f), // BS :: KEY_BACKSPACE
            ),
        ],
    ),
    /*
    (
        "0x00",
        [
            ("0x00", (0, 0)),
            ("0x00", (0, 0)),
            ("0x00", (0, 0)),
            ("0x00", (0, 0)),
            ("0x00", (0, 0)),
            ("0x00", (0, 0)),
        ],
    ),

    (
        "0x40",
        [
            ("0x00", (0, 0)),
            ("0x00", (0, 0)),
            ("0x00", (0, 0)),
            ("0x00", (0, 0)),
            ("0x00", (0, 0)),
            ("0x00", (0, 0)),
        ],
    ),
    (
        "0x80",
        [
            ("0x00", (0, 0)),
            ("0x00", (0, 0)),
            ("0x00", (0, 0)),
            ("0x00", (0, 0)),
            ("0x00", (0, 0)),
            ("0x00", (0, 0)),
        ],
    ),
    (
        "0xc0",
        [
            ("0x00", (0, 0)),
            ("0x00", (0, 0)),
            ("0x00", (0, 0)),
            ("0x00", (0, 0)),
            ("0x00", (0, 0)),
            ("0x00", (0, 0)),
        ],
    ),
    */
];

#[cfg(feature = "layout-ansi")]
pub static LAYOUT: Layout = [(
    "0x00",
    [
        ("0x00", (0, 0)),
        ("0x00", (0, 0)),
        ("0x00", (0, 0)),
        ("0x00", (0, 0)),
        ("0x00", (0, 0)),
        ("0x00", (0, 0)),
    ],
)];

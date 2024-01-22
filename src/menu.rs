use crate::wasm4::{ self };

const DUO_TITLE_WIDTH: u32 = 140;
const DUO_TITLE_HEIGHT: u32 = 28;
const DUO_TITLE_FLAGS: u32 = 1; // BLIT_2BPP
const DUO_TITLE: [u8; 980] = [
    0x0a, 0xaa, 0xaa, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2a, 0x00, 0x2a, 0x80,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x0a, 0x80, 0x0a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa8,
    0x00, 0x00, 0x00, 0x0a, 0x55, 0x56, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa6,
    0x80, 0x26, 0xa0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x29, 0x60, 0x25, 0x80, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x02, 0xa8, 0x00, 0x00, 0x00, 0x29, 0x55, 0x55, 0xa0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x95, 0x80, 0xa5, 0xa8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x25, 0x68, 0xa5, 0xa0, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x02, 0x9a, 0x00, 0x00, 0x00, 0x25, 0x55, 0x55, 0x58, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x95, 0x80, 0xa5, 0x58, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa5, 0x68,
    0x95, 0x68, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x5a, 0x00, 0x00, 0x00, 0xa5, 0x56, 0x95, 0x5a,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x95, 0x80, 0x95, 0x58, 0x00, 0x00, 0x00, 0x00,
    0x00, 0xa5, 0x58, 0x95, 0x58, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x56, 0x80, 0x00, 0x00, 0xa5,
    0x56, 0xa5, 0x5a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x95, 0x80, 0x95, 0x58, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x95, 0x56, 0x95, 0x58, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x55, 0x80,
    0x00, 0x00, 0x95, 0x56, 0x25, 0x56, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x95, 0xa0,
    0x95, 0x58, 0x00, 0x00, 0x00, 0x00, 0x00, 0x95, 0x56, 0x55, 0x5a, 0x82, 0xa0, 0x00, 0x00, 0x00,
    0x02, 0x55, 0x80, 0x00, 0x00, 0x95, 0x56, 0x09, 0x5a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x0a, 0x55, 0x58, 0x95, 0x5a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x95, 0x5a, 0x95, 0x5a, 0xaa, 0x68,
    0x00, 0x0a, 0xa0, 0x09, 0x55, 0x80, 0x00, 0x00, 0x95, 0x56, 0x29, 0x5a, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x29, 0x55, 0x5a, 0x95, 0x56, 0x80, 0x2a, 0xa0, 0x00, 0x00, 0x95, 0x5a, 0x95,
    0x5a, 0x29, 0x56, 0x02, 0xa9, 0xa8, 0xa9, 0x55, 0x80, 0x2a, 0x80, 0x95, 0x56, 0xa5, 0x68, 0x00,
    0xaa, 0x80, 0x02, 0xa8, 0x00, 0x00, 0x25, 0x55, 0x56, 0x95, 0x55, 0xa0, 0x26, 0xaa, 0x00, 0x00,
    0x95, 0x56, 0x55, 0x56, 0xa5, 0x56, 0x8a, 0x55, 0x6a, 0xa5, 0x55, 0x80, 0xa5, 0xa8, 0x95, 0x56,
    0x95, 0xa8, 0x02, 0x96, 0xa0, 0x0a, 0x9a, 0x00, 0x00, 0x25, 0x55, 0x56, 0x95, 0x55, 0xa8, 0x95,
    0x5a, 0x00, 0x00, 0x95, 0x55, 0x55, 0x56, 0x95, 0x55, 0xaa, 0x55, 0x5a, 0x95, 0x55, 0x82, 0x95,
    0x68, 0x95, 0x55, 0x56, 0x80, 0x0a, 0x55, 0x68, 0x29, 0x56, 0x80, 0x00, 0x29, 0x55, 0x56, 0x95,
    0x55, 0x56, 0x55, 0x56, 0x00, 0x00, 0x95, 0x55, 0x55, 0x56, 0x95, 0x55, 0xa9, 0x55, 0x5a, 0x55,
    0x55, 0x8a, 0x55, 0x58, 0x95, 0x55, 0x56, 0xa0, 0x29, 0x55, 0x5a, 0x25, 0xa5, 0x60, 0x00, 0x2a,
    0x55, 0x56, 0x95, 0x55, 0x5a, 0x56, 0x96, 0x00, 0x00, 0x95, 0x55, 0x55, 0x56, 0x95, 0x95, 0xa5,
    0x59, 0x6a, 0x55, 0x55, 0xa9, 0x5a, 0x58, 0x95, 0x55, 0x55, 0xa8, 0xa5, 0x59, 0x58, 0x95, 0xa5,
    0x60, 0x00, 0x0a, 0x55, 0xaa, 0x95, 0x55, 0x59, 0x56, 0x96, 0x00, 0x00, 0x95, 0x55, 0x55, 0x56,
    0x95, 0xa5, 0xa5, 0x59, 0xa9, 0x6a, 0x55, 0xa5, 0x5a, 0x58, 0x95, 0x56, 0xa5, 0x6a, 0xa5, 0x62,
    0x58, 0x96, 0xa5, 0x60, 0x00, 0x02, 0x95, 0xa0, 0x95, 0x55, 0x59, 0x56, 0x96, 0x00, 0x00, 0x95,
    0x55, 0x55, 0x56, 0x96, 0xa5, 0x65, 0x5a, 0xa9, 0x5a, 0x55, 0x95, 0x6a, 0x58, 0x95, 0x5a, 0x09,
    0x5a, 0xa5, 0x59, 0x58, 0x95, 0x55, 0xa0, 0x00, 0x00, 0x95, 0x80, 0x95, 0x95, 0x59, 0x55, 0x56,
    0x00, 0x00, 0x95, 0x55, 0x55, 0x56, 0x95, 0xa5, 0xa5, 0x58, 0x09, 0x5a, 0x55, 0x95, 0x55, 0x58,
    0x95, 0x5a, 0x09, 0x55, 0xa5, 0x59, 0x58, 0x95, 0x55, 0xa0, 0x00, 0x00, 0x95, 0x80, 0x95, 0x95,
    0x59, 0x55, 0x56, 0x00, 0x00, 0x95, 0x55, 0x95, 0x56, 0x95, 0x55, 0xa5, 0x58, 0x09, 0x55, 0x55,
    0x95, 0x95, 0x68, 0x95, 0x56, 0xa5, 0x55, 0xa5, 0x55, 0x68, 0x96, 0x55, 0xa0, 0x00, 0x00, 0x95,
    0x80, 0x95, 0xa5, 0x59, 0x55, 0x56, 0x00, 0x00, 0x95, 0x56, 0x95, 0x56, 0x95, 0x55, 0x65, 0x58,
    0x09, 0x55, 0x55, 0x96, 0xa5, 0x68, 0x95, 0x56, 0x95, 0x55, 0xa5, 0x55, 0xa8, 0x96, 0x95, 0x80,
    0x00, 0x00, 0x95, 0x80, 0x95, 0xa5, 0x59, 0x56, 0x96, 0x00, 0x00, 0x95, 0x56, 0x95, 0x56, 0x95,
    0x55, 0x65, 0x58, 0x09, 0x55, 0x55, 0x95, 0xaa, 0xa8, 0x95, 0x55, 0x55, 0x55, 0xa5, 0x56, 0xaa,
    0x95, 0xaa, 0xa0, 0x00, 0x00, 0x95, 0xaa, 0x95, 0xa5, 0x59, 0x5a, 0xaa, 0x80, 0x00, 0x95, 0x56,
    0x95, 0x56, 0x95, 0x55, 0x65, 0x58, 0x0a, 0x55, 0x55, 0x95, 0x6a, 0xaa, 0x95, 0x55, 0x55, 0x55,
    0xa5, 0x56, 0x9a, 0x95, 0x55, 0x6a, 0x00, 0x00, 0x95, 0x56, 0x95, 0xa5, 0x59, 0x56, 0x95, 0x80,
    0x00, 0x95, 0x5a, 0x95, 0x56, 0x95, 0x55, 0x65, 0x58, 0x02, 0x55, 0x55, 0x95, 0x55, 0x5a, 0xa5,
    0x55, 0x55, 0x55, 0xa5, 0x55, 0x56, 0x95, 0x55, 0x5a, 0x00, 0x00, 0x95, 0x56, 0x95, 0xa5, 0x59,
    0x55, 0x55, 0x80, 0x00, 0x95, 0x5a, 0x95, 0x56, 0x95, 0x55, 0xa5, 0x58, 0x02, 0x55, 0x55, 0xa5,
    0x55, 0x56, 0x25, 0x55, 0x55, 0x56, 0xa9, 0x55, 0x56, 0xa5, 0x55, 0x5a, 0x00, 0x00, 0x95, 0x56,
    0x95, 0xa5, 0x5a, 0x55, 0x55, 0x80, 0x00, 0xa5, 0x5a, 0xa5, 0x5a, 0xa5, 0x56, 0xa5, 0x58, 0x02,
    0x55, 0x55, 0xa5, 0x55, 0x56, 0x29, 0x55, 0x55, 0x5a, 0x09, 0x55, 0x5a, 0xa9, 0x55, 0x5a, 0x00,
    0x00, 0x95, 0x5a, 0x95, 0xa9, 0x6a, 0x95, 0x55, 0x80, 0x00, 0x29, 0x6a, 0xa9, 0x58, 0xa9, 0x5a,
    0xa9, 0x58, 0x02, 0x55, 0x55, 0xa9, 0x55, 0x5a, 0x2a, 0x95, 0x55, 0xa8, 0x02, 0x55, 0x5a, 0x09,
    0x55, 0x6a, 0x00, 0x00, 0x95, 0x5a, 0x95, 0xa9, 0x62, 0x95, 0x55, 0x80, 0x00, 0x29, 0x68, 0x29,
    0xaa, 0xa9, 0x5a, 0xa9, 0xa8, 0x02, 0x95, 0x5a, 0xa9, 0x55, 0x5a, 0x00, 0xa5, 0x56, 0xa0, 0x02,
    0x55, 0x68, 0x02, 0x56, 0xa0, 0x00, 0x00, 0xa5, 0x6a, 0xa6, 0xa9, 0x60, 0xa5, 0x6a, 0x80, 0x00,
    0x29, 0x68, 0x09, 0xa8, 0x29, 0x6a, 0x09, 0x60, 0x00, 0xaa, 0x6a, 0x0a, 0x55, 0x6a, 0x00, 0xaa,
    0xaa, 0x80, 0x00, 0xaa, 0xa0, 0x00, 0xaa, 0xa0, 0x00, 0x00, 0x2a, 0xa8, 0x2a, 0x0a, 0xa0, 0x2a,
    0xaa, 0x00, 0x00, 0x0a, 0xa0, 0x0a, 0xa0, 0x0a, 0xa8, 0x0a, 0xa0, 0x00, 0x2a, 0xa0, 0x00, 0xaa,
    0xa8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xa0, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
];
// PlayButton
const PLAY_BUTTON_WIDTH: u32 = 40;
const PLAY_BUTTON_HEIGHT: u32 = 20;
const PLAY_BUTTON_FLAGS: u32 = 1; // BLIT_2BPP
const PLAY_BUTTON: [u8; 200] = [
    0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x54, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x55, 0x55, 0x50, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05,
    0xaa, 0xa5, 0x94, 0x15, 0x05, 0x51, 0x54, 0x00, 0x00, 0x16, 0xaa, 0xa5, 0xa4, 0x55, 0x45, 0x55,
    0x54, 0x00, 0x00, 0x5a, 0x95, 0xa5, 0xa5, 0x6a, 0x55, 0x95, 0x64, 0x00, 0x00, 0x1a, 0x95, 0xa5,
    0xa5, 0xaa, 0x55, 0xa5, 0xa4, 0x00, 0x00, 0x16, 0xaa, 0xa5, 0xa6, 0xa6, 0x95, 0xa5, 0xa5, 0x00,
    0x00, 0x06, 0xaa, 0x95, 0x66, 0x56, 0xa5, 0xa5, 0xa5, 0x00, 0x00, 0x06, 0xa5, 0x55, 0x65, 0x55,
    0xa5, 0xa5, 0xa4, 0x00, 0x00, 0x06, 0xa4, 0x05, 0xa6, 0xaa, 0xa5, 0xa9, 0xa4, 0x00, 0x00, 0x06,
    0xa4, 0x05, 0xa6, 0x95, 0xa5, 0x5a, 0xa4, 0x00, 0x00, 0x06, 0xa4, 0x05, 0xa6, 0xaa, 0xa5, 0x5a,
    0x94, 0x00, 0x00, 0x05, 0x54, 0x05, 0x55, 0xaa, 0x55, 0x5a, 0x50, 0x00, 0x00, 0x01, 0x40, 0x00,
    0x51, 0x55, 0x69, 0xa9, 0x40, 0x00, 0x00, 0x00, 0x40, 0x00, 0x01, 0x00, 0x6a, 0xa9, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x55, 0xa5, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x05, 0x55, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x54, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00,
];

pub struct Menu {
    play_selected: bool,
    score_selected: bool,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            play_selected: true,
            score_selected: true,
        }
    }

    pub fn display(&mut self) {
        wasm4::blit(&DUO_TITLE, 10, 10, DUO_TITLE_WIDTH, DUO_TITLE_HEIGHT, DUO_TITLE_FLAGS);
        wasm4::blit(&PLAY_BUTTON, 60, 80, PLAY_BUTTON_WIDTH, PLAY_BUTTON_HEIGHT, DUO_TITLE_FLAGS)
    }
}

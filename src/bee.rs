use crate::wasm4::{ self };
use crate::palette::set_draw_color;
//use tiny_rng::{ Rng, Rand };
// BeeBody
// const BEE_BODY_WIDTH: u32 = 8;
// const BEE_BODY_HEIGHT: u32 = 8;
const BEE_BODY_FLAGS: u32 = 1; // BLIT_2BPP
const BEE_BODY: [u8; 16] = [
    0x55, 0x55, 0x59, 0x65, 0x56, 0x95, 0x54, 0x15, 0x56, 0x95, 0x54, 0x15, 0x55, 0x55, 0x55, 0x55,
];

const WINGS_UP: [u8; 8] = [
    0b11111111, 0b11111111, 0b11111111, 0b10111101, 0b11011011, 0b11111111, 0b11111111, 0b11111111,
];
const WINGS_DOWN: [u8; 8] = [
    0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11011011, 0b10111101, 0b11111111, 0b11111111,
];

pub struct Bee {
    pub xpos: i32,
    pub ypos: i32,
    pub flags: u32,
    pub first_draw: bool,
    width: u32,
    height: u32,
    wingsup: bool,
    direction: u32,
    pathx: array,
    pathy: Vec<(i32, u32)>,
    dirx: u32,
    diry: u32,
}

impl Bee {
    pub fn new(xpos: i32, ypos: i32) -> Self {
        Self {
            width: 8,
            height: 8,
            xpos: xpos,
            ypos: ypos,
            flags: BEE_BODY_FLAGS,
            wingsup: true,
            direction: 0,
            pathx: Vec::new(),
            pathy: Vec::new(),
            dirx: 0,
            diry: 0,
            first_draw: false,
        }
    }

    pub fn draw(&self) {
        set_draw_color(0x0204);
        wasm4::blit(&BEE_BODY, self.xpos, self.ypos, self.width, self.height, 1 | self.direction);
        set_draw_color(0x0002);
        if self.wingsup == true {
            wasm4::blit(
                &WINGS_UP,
                self.xpos,
                self.ypos,
                self.width,
                self.height,
                0 | self.direction
            );
        } else {
            wasm4::blit(
                &WINGS_DOWN,
                self.xpos,
                self.ypos,
                self.width,
                self.height,
                0 | self.direction
            );
        }
    }
    #[allow(unused)]
    pub fn update(&mut self) {
        if self.wingsup == true {
            self.wingsup = false;
        } else {
            self.wingsup = true;
        }
    }

    pub fn attack(&mut self, clock: u32) {}

    pub fn calculate_path(&mut self) {
        if self.xpos < 80 {
            for position in (self.xpos..80).rev() {
                self.pathx.push((position, 270u32));
            }
        } else {
            for position in 80..self.xpos {
                self.pathx.push((position, 90u32));
            }
        }
        if self.ypos < 80 {
            for position in (self.ypos..80).rev() {
                self.pathy.push((position, 180u32));
            }
        } else {
            for position in 80..self.ypos {
                self.pathy.push((position, 0u32));
            }
        }
        self.first_draw = true;
        // for (x, _y) in self.pathx.iter() {
        //     let s = x.to_string();
        //     trace(s);
        // }
    }
}

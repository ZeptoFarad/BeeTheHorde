use crate::wasm4::{ self };
use crate::palette::set_draw_color;
use tiny_rng::{ Rng, Rand };
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
    pathx: [i32; 70],
    pathy: [i32; 70],
    dirx: u32,
    diry: u32,
    direction: u32,
    currentx: usize,
    currenty: usize,
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
            pathx: [0; 70],
            pathy: [0; 70],
            dirx: 0,
            diry: 0,
            first_draw: false,
            direction: 0,
            currentx: 0,
            currenty: 0,
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
    pub fn buzz_wings(&mut self) {
        if self.wingsup == true {
            self.wingsup = false;
        } else {
            self.wingsup = true;
        }
    }

    pub fn attack(&mut self, clock: u32) -> bool {
        let mut rng = Rng::from_seed(clock as u64);
        let rand = rng.rand_range_u32(1, 3);
        if rand == 1 && self.currentx != self.pathx.len() {
            self.xpos = self.pathx[self.currentx];
            self.direction = self.dirx;
            if self.currenty != 69 {
                self.currentx += 1;
                return true;
            } else {
                return false;
            }
        } else if rand == 2 && self.currenty != self.pathy.len() {
            self.ypos = self.pathy[self.currenty];
            self.direction = self.diry;
            if self.currenty != 69 {
                self.currenty += 1;
                return true;
            } else {
                return false;
            }
        }
        return false;
    }

    pub fn calculate_path(&mut self) {
        if self.xpos > 80 {
            for (index, _) in self.pathx.into_iter().enumerate() {
                self.pathx[index] = self.xpos - (index as i32);
            }
            self.dirx = 90;
        } else {
            for (index, _) in self.pathx.into_iter().enumerate() {
                self.pathx[index] = self.xpos + (index as i32);
            }
            self.dirx = 270;
        }
        if self.ypos > 80 {
            for (index, _) in self.pathy.into_iter().enumerate() {
                self.pathy[index] = self.ypos - (index as i32);
            }
            self.diry = 0;
        } else {
            for (index, _) in self.pathy.into_iter().enumerate() {
                self.pathy[index] = self.ypos + (index as i32);
            }
            self.diry = 180;
        }

        self.first_draw = true;
    }
}

use crate::wasm4::{ self };

//---Explosion Properties---//
// WIDTH= 8;
// HEIGHT= 8;
// FLAGS= 0; // BLIT_1BPP

const EXPLO1: [u8; 8] = [0xff, 0xff, 0xff, 0xe7, 0xe7, 0xff, 0xff, 0xff];
const EXPLO2: [u8; 8] = [0xff, 0xff, 0xdb, 0xe7, 0xe7, 0xdb, 0xff, 0xff];
const EXPLO3: [u8; 8] = [0xff, 0xdf, 0xed, 0xe3, 0xc7, 0xb7, 0xfb, 0xff];
const EXPLO4: [u8; 8] = [0xdf, 0xef, 0xf6, 0xdd, 0xbb, 0x6f, 0xf7, 0xfb];
const EXPLO5: [u8; 8] = [0xf7, 0xfb, 0xbf, 0x7f, 0xfe, 0xfd, 0xdf, 0xef];
const EXPLO6: [u8; 8] = [0xbf, 0xfe, 0xff, 0xff, 0xff, 0xff, 0x7f, 0xfd];

pub struct Explosion {
    xpos: i32,
    ypos: i32,
    anim_count: i32,
    pub complete: bool,
}

impl Explosion {
    pub fn new(xpos: i32, ypos: i32) -> Self {
        Self {
            xpos: xpos,
            ypos: ypos,
            anim_count: 0,
            complete: false,
        }
    }

    pub fn explode(&mut self) {
        if self.anim_count == 0 {
            wasm4::blit(&EXPLO1, self.xpos, self.ypos, 8, 8, 0);
            self.anim_count += 1;
        } else if self.anim_count == 1 {
            wasm4::blit(&EXPLO2, self.xpos, self.ypos, 8, 8, 0);
            self.anim_count += 1;
        } else if self.anim_count == 2 {
            wasm4::blit(&EXPLO3, self.xpos, self.ypos, 8, 8, 0);
            self.anim_count += 1;
        } else if self.anim_count == 3 {
            wasm4::blit(&EXPLO4, self.xpos, self.ypos, 8, 8, 0);
            self.anim_count += 1;
        } else if self.anim_count == 4 {
            wasm4::blit(&EXPLO5, self.xpos, self.ypos, 8, 8, 0);
            self.anim_count += 1;
        } else if self.anim_count == 5 {
            wasm4::blit(&EXPLO6, self.xpos, self.ypos, 8, 8, 0);
            self.anim_count += 1;
        } else {
            self.complete = true;
        }
    }
}

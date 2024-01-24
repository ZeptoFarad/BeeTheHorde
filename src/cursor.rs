use crate::wasm4::{ self, trace };

pub struct Cursor {
    pub positionx: i32,
    pub positiony: i32,
    loading: bool,
    last_mouse: [i32; 2],
    gamepad_active: bool,
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            positionx: 0,
            positiony: 0,
            loading: false,
            last_mouse: [0; 2],
            gamepad_active: true,
        }
    }
    pub fn position(&mut self, frame_count: u32) -> [i32; 3] {
        let gamepad = unsafe { *wasm4::GAMEPAD1 };
        let mouse_down = unsafe { *wasm4::MOUSE_BUTTONS };
        let mouse_x = (unsafe { *wasm4::MOUSE_X }) as i32;
        let mouse_y = (unsafe { *wasm4::MOUSE_Y }) as i32;
        if gamepad != 0 {
            self.last_mouse = [mouse_x, mouse_y];
            self.gamepad_active = true;
            if gamepad == 64 && self.positionx > 0 {
                self.positiony -= 1;
                return [self.positionx, self.positiony, 0];
            }
            if gamepad == 128 && self.positiony < 160 {
                self.positiony += 1;
                return [self.positionx, self.positiony, 0];
            }
            if gamepad == 16 && self.positionx > 0 {
                self.positionx -= 1;
                return [self.positionx, self.positiony, 0];
            }
            if gamepad == 32 && self.positionx < 160 {
                self.positionx += 1;
                return [self.positionx, self.positiony, 0];
            }
            if gamepad == 1 {
                return [self.positionx, self.positiony, 1];
            }
        } else if
            self.gamepad_active == true &&
            (self.last_mouse[0] > mouse_x + 2 || self.last_mouse[0] < mouse_x - 2) &&
            (self.last_mouse[1] > mouse_y + 2 || self.last_mouse[1] < mouse_y - 2)
        {
            self.gamepad_active = false;
            if mouse_down == 1 && frame_count % 5 == 0 {
                return [mouse_x, mouse_y, 1];
            } else {
                return [mouse_x, mouse_y, 0];
            }
        } else if self.gamepad_active == false {
            if mouse_down == 1 && frame_count % 5 == 0 {
                return [mouse_x, mouse_y, 1];
            } else {
                return [mouse_x, mouse_y, 0];
            }
        }
        return [self.positionx, self.positiony, 0];
    }
}

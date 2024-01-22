use crate::wasm4;
use crate::palette::set_draw_color;

pub struct Base {
    pub xpos: i32,
    pub ypos: i32,
    width: u32,
    height: u32,
}

impl Base {
    pub fn new() -> Self {
        Self {
            width: 20,
            height: 20,
            xpos: 80,
            ypos: 80,
        }
    }
    pub fn draw(&self) {
        set_draw_color(0x2);
        //rect (x, y, width, height)
        wasm4::rect(self.xpos - 10, self.ypos - 10, self.width, self.height)
    }
}

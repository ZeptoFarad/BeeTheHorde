use crate::bee::Bee;
use crate::base::Base;
use crate::explode::Explode;
use crate::wasm4::{ self, trace };

pub struct Game {
    bee: Bee,
    bees: Vec<Bee>,
    base: Base,
    frame_count: u32,
    explode: Explode,
}

impl Game {
    pub fn new() -> Self {
        Self {
            bee: Bee::new(140, 140),
            base: Base::new(),
            frame_count: 0,
            bees: vec![],
            explode: Explode::new(10, 10),
        }
    }
    pub fn update(&mut self) {
        const BOOM: std::ops::Range<i32> = 75..80;
        self.frame_count += 1;
        let s = self.frame_count.to_string();
        trace(s);
        for (_pos, xbee) in self.bees.iter_mut().enumerate() {
            if xbee.first_draw == false {
                xbee.calculate_path();
            }

            xbee.draw();
            if self.frame_count % 15 == 0 {
                if BOOM.contains(&xbee.xpos) && BOOM.contains(&xbee.ypos) {
                    trace("BOOMM!");
                }
                xbee.attack(self.frame_count);
            }
            if self.frame_count % 5 == 0 {
                xbee.update();
            }
        }

        if self.frame_count % 5 == 0 && self.bees.len() < 10 {
            self.input();
        }

        self.base.draw();
    }

    pub fn input(&mut self) {
        let mouse_down = unsafe { *wasm4::MOUSE_BUTTONS };
        let mouse_x = (unsafe { *wasm4::MOUSE_X }) as i32;
        let mouse_y = (unsafe { *wasm4::MOUSE_Y }) as i32;

        if mouse_down == 1 {
            trace(mouse_x.to_string());
            trace(mouse_y.to_string());
            self.bees.push(Bee::new(mouse_x, mouse_y));
        }
    }
}

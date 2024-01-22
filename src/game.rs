use crate::bee::Bee;
use crate::base::Base;
use crate::explode::Explosion;
use crate::palette::set_draw_color;
use crate::wasm4::{ self };
use crate::menu::Menu;

pub struct Game {
    //bee: Bee,
    bees: Vec<Bee>,
    base: Base,
    frame_count: u32,
    booms: Vec<Explosion>,
    menu: Menu,
    value: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
            base: Base::new(),
            frame_count: 0,
            bees: vec![],
            booms: vec![],
            menu: Menu::new(),
            value: true,
        }
    }
    pub fn update(&mut self) {
        set_draw_color(0x0241);
        self.menu.display();

        if self.value == false {
            const BOOM: std::ops::Range<i32> = 60..90;
            self.frame_count += 1;
            // let s = self.frame_count.to_string();
            // trace(s);
            for (pos, xbee) in self.bees.iter_mut().enumerate() {
                if xbee.first_draw == false {
                    xbee.calculate_path();
                }
                if self.frame_count % 15 == 0 {
                    if BOOM.contains(&xbee.xpos) && BOOM.contains(&xbee.ypos) {
                        // trace("BOOM!");
                        self.booms.push(Explosion::new(xbee.xpos, xbee.ypos));
                        self.bees.remove(pos);
                        break;
                    }
                    let status: bool = xbee.attack(self.frame_count);
                    if status == false {
                        self.bees.remove(pos);
                        break;
                    }
                }
                if self.frame_count % 5 == 0 {
                    xbee.buzz_wings();
                }
                xbee.draw();
            }
            self.base.draw();
            set_draw_color(0x2);
            for (pos, boom) in self.booms.iter_mut().enumerate() {
                if boom.complete == true {
                    self.booms.remove(pos);
                    break;
                }
                if self.frame_count % 8 == 0 {
                    boom.explode();
                }
            }

            if self.frame_count % 5 == 0 && self.bees.len() < 8 {
                self.input();
            }

            set_draw_color(0x2);
            wasm4::text("Bees:", 2, 2);
            let mut left: String = (8 - self.bees.len()).to_string().to_owned();
            let slash: &str = "/8";
            left.push_str(slash);
            wasm4::text(left, 50, 2);
        }
    }

    pub fn input(&mut self) {
        let mouse_down = unsafe { *wasm4::MOUSE_BUTTONS };
        let mouse_x = (unsafe { *wasm4::MOUSE_X }) as i32;
        let mouse_y = (unsafe { *wasm4::MOUSE_Y }) as i32;

        if mouse_down == 1 {
            // trace(mouse_x.to_string());
            // trace(mouse_y.to_string());
            self.bees.push(Bee::new(mouse_x, mouse_y));
        }
    }
}

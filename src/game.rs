use crate::bee::Bee;
use crate::base::Base;
use crate::explode::Explosion;
use crate::palette::set_draw_color;
use crate::wasm4::{ self };
use crate::menu::Menu;
use crate::cursor::Cursor;

pub struct Game {
    //bee: Bee,
    bees: Vec<Bee>,
    base: Base,
    frame_count: u32,
    booms: Vec<Explosion>,
    menu: Menu,
    value: bool,
    cursor: Cursor,
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
            cursor: Cursor::new(),
        }
    }
    pub fn update(&mut self) {
        if self.value == true {
            set_draw_color(0x0241);
            self.menu.display();
        }
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
                        self.booms.push(Explosion::new(xbee.xpos, xbee.ypos));
                        self.bees.remove(pos);
                        self.base.base_hit();
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

            set_draw_color(0x2);
            wasm4::text("Bees:", 2, 2);
            let mut left: String = (8 - self.bees.len()).to_string().to_owned();
            let slash: &str = "/8";
            left.push_str(slash);
            wasm4::text(left, 50, 2);
        }
        let button_areax: std::ops::Range<i32> = 60..100;
        let button_areay: std::ops::Range<i32> = 80..100;

        let [xpos, ypos, click] = self.cursor.position(self.frame_count);
        let mut upclick: bool = false;
        if
            button_areax.contains(&xpos) &&
            button_areay.contains(&ypos) &&
            click == 1 &&
            self.value == true
        {
            self.value = false;
        }
        if click == 1 && self.value == false && self.frame_count % 20 == 0 && upclick == false {
            self.bees.push(Bee::new(xpos, ypos));
            upclick = true;
        }

        set_draw_color(0x02);
        wasm4::line(xpos, ypos - 2, xpos, ypos - 3);
        wasm4::line(xpos, ypos + 2, xpos, ypos + 3);
        wasm4::line(xpos - 2, ypos, xpos - 3, ypos);
        wasm4::line(xpos + 2, ypos, xpos + 3, ypos);
    }
}

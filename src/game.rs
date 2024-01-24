use crate::bee::Bee;
use crate::base::Base;
use crate::explode::Explosion;
use crate::palette::set_draw_color;
use crate::wasm4::{ self, blit, rect, text, trace };
use crate::menu::Menu;
use crate::cursor::Cursor;
use crate::rules::RULES;
use crate::win::YOU_WIN;

pub struct Game {
    //bee: Bee,
    bees: Vec<Bee>,
    base: Base,
    frame_count: u32,
    booms: Vec<Explosion>,
    menu: Menu,
    game_running: bool,
    cursor: Cursor,
    game_win: bool,
    display_rules: bool,
    rules_timer: i32,
    base_destroyed: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
            base: Base::new(),
            frame_count: 0,
            bees: vec![],
            booms: vec![],
            menu: Menu::new(),
            game_running: false,
            game_win: false,
            display_rules: false,
            cursor: Cursor::new(),
            rules_timer: 120,
            base_destroyed: false,
        }
    }
    pub fn update(&mut self) {
        if self.game_running == false && self.display_rules == false && self.game_win == false {
            set_draw_color(0x0241);
            self.menu.display();
        } else if self.game_running == false && self.display_rules == true {
            set_draw_color(0x020);
            blit(&RULES, 20, 20, 120, 120, 0);
            set_draw_color(0x002);
            text("Click to continue", 10, 150);
            self.rules_timer -= 1;
        } else if self.game_running == false && self.game_win == true {
            set_draw_color(0x0240);
            blit(&YOU_WIN, 20, 60, 120, 40, 1);
            let timer_string = (self.rules_timer / 60).to_string();
            text("Reset in:", 30, 150);
            text(timer_string, 120, 150);
            self.rules_timer -= 1;
            if self.rules_timer <= 0 {
                self.game_running = false;
                self.game_win = false;
                self.display_rules = false;
            }
        }
        if self.game_running == true {
            const BOOM: std::ops::Range<i32> = 60..90;
            self.frame_count += 1;
            for (pos, xbee) in self.bees.iter_mut().enumerate() {
                if xbee.first_draw == false {
                    xbee.calculate_path();
                }
                if self.frame_count % 15 == 0 {
                    if BOOM.contains(&xbee.xpos) && BOOM.contains(&xbee.ypos) {
                        self.booms.push(Explosion::new(xbee.xpos, xbee.ypos));
                        self.bees.remove(pos);
                        self.base_destroyed = self.base.base_hit();
                        if self.base_destroyed == true {
                            self.game_running = false;
                            self.game_win = true;
                            self.rules_timer = 5 * 60;
                        }
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
            // Safety Area
            set_draw_color(0x30);
            rect(60, 60, 35, 35);
            //Current Bees Count
            set_draw_color(0x2);
            text("Bees out:", 2, 2);
            let mut left: String = (8 - self.bees.len()).to_string().to_owned();
            let slash: &str = "/8";
            left.push_str(slash);
            wasm4::text(left, 80, 2);
            //Total Bees Left
        }
        let button_areax: std::ops::Range<i32> = 60..100;
        let button_areay: std::ops::Range<i32> = 80..100;
        let safe_area: std::ops::Range<i32> = 60..95;

        let [xpos, ypos, click] = self.cursor.position(self.frame_count);

        if
            button_areax.contains(&xpos) &&
            button_areay.contains(&ypos) &&
            click == 1 &&
            self.game_running == false
        {
            trace("CLICK");
            self.display_rules = true;
        }
        // Display Rules Screen
        if click == 1 && self.display_rules == true && self.rules_timer <= 0 {
            self.display_rules = false;
            self.game_running = true;
        }
        //Add New Bees
        if
            click == 1 &&
            self.game_running == true &&
            self.frame_count % 5 == 0 &&
            !safe_area.contains(&xpos) &&
            !safe_area.contains(&ypos)
        {
            trace("Click");
            if self.bees.len() < 8 {
                let new_bee = Bee::new(xpos, ypos);
                self.bees.push(new_bee);

                trace("Click");
            }
        } else if
            click == 1 &&
            self.game_running == true &&
            self.frame_count % 5 == 0 &&
            safe_area.contains(&xpos) &&
            safe_area.contains(&ypos)
        {
            trace("Inside Safe Area");
        }

        set_draw_color(0x02);
        wasm4::line(xpos, ypos - 2, xpos, ypos - 3);
        wasm4::line(xpos, ypos + 2, xpos, ypos + 3);
        wasm4::line(xpos - 2, ypos, xpos - 3, ypos);
        wasm4::line(xpos + 2, ypos, xpos + 3, ypos);
    }
}

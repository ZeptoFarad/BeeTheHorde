#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
mod palette;
mod explode;
mod bee;
mod game;
mod base;
mod menu;
mod cursor;

use game::Game;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref BEE_GAME: Mutex<Game> = Mutex::new(Game::new());
}

#[rustfmt::skip]

#[no_mangle]
fn start() {
    //[Light Brown(9d654c), Dark Magenta(210b1b), Medium Magenta(4d222c), Yellow Gold(cfab51)]
    palette::set_palette([0x9d654c, 0x210b1b, 0x4d222c,0xcfab51]);
}

#[no_mangle]
fn update() {
    BEE_GAME.lock().expect("game_state").update();
}

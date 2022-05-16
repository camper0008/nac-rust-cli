mod io;
mod logic;
mod utils;

use crate::logic::game_loop;

fn main() {
    loop {
        game_loop();
    }
}

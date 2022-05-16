mod io;
mod logic;
mod utils;

use crate::io::{display_prompt, read_stdin};
use crate::logic::game_loop;
use std::process::exit;

fn main() {
    io::clear_screen();
    loop {
        game_loop();
        loop {
            display_prompt("Play again? [y/n]: ");
            let stdin_result = read_stdin();
            if let Ok(input) = stdin_result {
                match input.chars().nth(0).unwrap() {
                    'y' | 'Y' => break,
                    'n' | 'N' => exit(0),
                    invalid => println!("Unrecognized input '{invalid}'."),
                }
            }
        }
    }
}

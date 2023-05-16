//extern crate pancurses;
//use std::collections::HashMap;

pub mod food;
pub mod game;
pub mod point;
pub mod snake;

pub use game::Game;

use pancurses::{curs_set, endwin, initscr, napms, noecho, Input, Window};

struct Key<'a> {
    printable: &'a str,
    input: Input,
}

const PAUSE_KEY: Key = Key {
    printable: "Backspace",
    input: Input::KeyBackspace,
};

const QUIT_KEY: Key = Key {
    printable: "Delete",
    input: Input::KeyDC,
};

fn print_welcome_text(window: &Window, game: &Game) {
    let lines: [String; 3] = [
        "Welcome to Snake (written in Rust)".to_owned(),
        format!("Press {0} to pause!", PAUSE_KEY.printable).to_owned(),
        format!("Press {0} to quit!", QUIT_KEY.printable),
    ];
    for (index, line) in lines.iter().enumerate() {
        window.mvprintw(
            game.height / 2 + index as i32,
            (game.width - line.len() as i32) / 2,
            line,
        );
    }
    window.refresh();
    napms(2000);
}

fn main() {
    let window = initscr();
    //window.printw("Type things, press delete to quit\n");
    window.refresh();
    let mut game = Game::new();
    window.keypad(true);
    window.timeout(1); // Do not wait for input
    noecho();
    curs_set(0);

    print_welcome_text(&window, &game);

    loop {
        match window.getch() {
            Some(Input::Character(c)) => {
                if !game.is_paused() {
                    game.snake.change_dir(c);
                }
                //window.addch(c);
            }
            Some(Input::KeyDC) => break,
            Some(Input::KeyBackspace) => {
                game.pause();
                //Draw Paused text
                if game.is_paused() {
                    window.mvprintw(game.height / 2, game.width / 2 - 7, "Game is paused");
                }
            }
            Some(_input) => {
                //    window.addstr(&format!("{:?}", input));
            }
            None => (),
        }
        if game.is_running() {
            game.update(&window);
        }
    }
    endwin();
}

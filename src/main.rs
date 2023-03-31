//extern crate pancurses;
pub mod food;
pub mod game;
pub mod point;
pub mod snake;

pub use game::Game;

use pancurses::{curs_set, endwin, initscr, noecho, Input};

fn main() {
    let window = initscr();
    //window.printw("Type things, press delete to quit\n");
    window.refresh();
    let mut game = Game::new();
    window.keypad(true);
    window.timeout(1); // Do not wait for input
    noecho();
    curs_set(0);
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

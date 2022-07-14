use pancurses::Window;
use std::time::{Instant};

use crate::food::Food;
use crate::snake::Snake;

const SNAKE_DELAY:u128 = 100;

#[derive(Copy, Clone)]
enum GameState{
    Running,
    Paused,
    GameOver,
}

pub struct Game {
    pub width: i32,
    pub height: i32,
    pub snake: Snake,
    last_tick: Instant,
    food: Food,
    state: GameState,
}

impl Game {
    pub fn new() -> Game {
        let width = 40;
        let height = 20;
        Game {
            width: width,
            height: height,
            snake: Snake::new(),
            last_tick: Instant::now(),
            food: Food::new(width,height),
            state: GameState::Running,
        }
    }

    pub fn food_without_collision(&mut self){
        let mut food_to_spawn = Food::new(self.width,self.height);
        while self.snake.check_for_collision_with(food_to_spawn.get_pos()) {
            food_to_spawn = Food::new(self.width,self.height);
        }
        self.food = food_to_spawn;
    }

    pub fn pause(&mut self){
        self.state = match self.state{
            GameState::Running => GameState::Paused,
            GameState::Paused => GameState::Running,
            GameState::GameOver => GameState::Running,
        }
    }

    fn draw_frame(&self, win: &Window) -> () {
        //Draw Border
        const WALL_CHAR:char = '#';
        for y in 0..self.height {
            win.mvaddch(y, 0, WALL_CHAR);
            win.mvaddch(y, self.width, WALL_CHAR);
        }
        for x in 0..self.width + 1 {
            win.mvaddch(0, x, WALL_CHAR);
            win.mvaddch(self.height, x, WALL_CHAR);
        }

        //Draw Snake
        self.snake.draw_snake(win);

        //Draw Food
        self.food.draw_food(win);
    }

    fn game_over(&mut self) {
        self.snake = Snake::new();
        self.food_without_collision();
    }

    pub fn is_running(&self) -> bool{
        match self.state{
            GameState::Running => true,
            GameState::Paused => false,
            GameState::GameOver => false,
        }
    }

    pub fn is_paused(&self) -> bool{
        match self.state{
            GameState::Running => false,
            GameState::Paused => true,
            GameState::GameOver => false,
        }
    }
    fn update_game(&mut self) {
        self.snake.move_snake();
        //Check if any food has been eaten
        if self.snake.get_head() == self.food.get_pos() {
            self.snake.eat_food();
            self.food_without_collision();
        }
        //Check for collision with wall and for collision with self 
        if self.snake.get_head().get_x() >= self.width
            || self.snake.get_head().get_x() <= 0
            || self.snake.get_head().get_y() >= self.height
            || self.snake.get_head().get_y() <= 0
            || self.snake.check_for_self_collision()
        {
            self.state = GameState::GameOver;
        }
    }

    pub fn update(&mut self, win: &Window) -> () {
        let delta_time = self.last_tick.elapsed();
        //let delta_time = Instant::now().duration_since(self.last_tick);
        //std::thread::sleep(Duration::new(0,500000000)-delta_time); // This resulted in unresponsive keys
        if delta_time.as_millis() >= SNAKE_DELAY {
            self.last_tick = Instant::now();
            self.update_game();
            win.clear();
            self.draw_frame(win);
            match self.state{
                GameState::GameOver =>{
                    self.game_over();
                    win.mvprintw(self.height/2, self.width/2 - 4, "Game Over");
                    win.mvprintw(self.height/2+1, self.width/2 - 11, "Backspace to try again");
                    win.mvprintw(self.height/2+2, self.width/2 - 7, "Delete to quit");
                }
                _ => ()
        }
            win.refresh();
        }
    }
}

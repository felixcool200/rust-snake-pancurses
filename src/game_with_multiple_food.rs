use pancurses::Window;
use rand::Rng;
use std::time::{Instant};

use crate::food::Food;
use crate::point::Point;
use crate::snake::Snake;

const SNAKE_DELAY:u128 = 100;

pub struct Game {
    width: i32,
    height: i32,
    pub snake: Snake,
    last_tick: Instant,
    foods: Vec<Food>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            width: 40,
            height: 20,
            snake: Snake::new(),
            last_tick: Instant::now(),
            foods: Vec::new(),
        }
    }

    fn draw_frame(&self, win: &Window) -> () {
        //Draw Border
        for y in 0..self.height {
            win.mvaddch(y, 0, '#');
            win.mvaddch(y, self.width, '#');
        }
        for x in 0..self.width + 1 {
            win.mvaddch(0, x, '#');
            win.mvaddch(self.height, x, '#');
        }

        //Draw Snake
        self.snake.draw_snake(win);

        //Draw Food
        for food in &self.foods {
            food.draw_food(win);
        }
    }

    fn game_over(&mut self) {
        self.snake = Snake::new();
        self.foods = Vec::new();
    }

    fn update_game(&mut self) {
        self.snake.move_snake();
        //Check if any food has been eaten
        for food_index in 0..self.foods.len() {
            if self.snake.get_head() == self.foods[food_index].get_pos() {
                self.snake.eat_food();
                self.foods.swap_remove(food_index);
                break;
            }
        }
        //Check for collision with wall and for collision with self 
        if self.snake.get_head().get_x() >= self.width
            || self.snake.get_head().get_x() <= 0
            || self.snake.get_head().get_y() >= self.height
            || self.snake.get_head().get_y() <= 0
            || self.snake.check_for_self_collision()
        {
            self.game_over();
        }
    }

    fn spawn_food(&mut self){
        loop{
            let point_to_spawn = Point::new(rand::thread_rng().gen_range(1..self.width),rand::thread_rng().gen_range(1..self.height));
            
            if self.snake.check_for_self_collision() {
                continue;
            }
            for food in &self.foods{
                if food.get_pos() == point_to_spawn{
                    continue;
                }
            }
            
            self.foods.push(Food::new(point_to_spawn));
            break;
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
            win.refresh();
            if self.foods.len() == 0{
                self.spawn_food();
            }
        }
    }
}

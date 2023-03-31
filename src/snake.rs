use crate::point::Point;
use pancurses::Window;
use std::collections::VecDeque;

#[derive(PartialEq, Eq, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub struct Snake {
    dir: Direction,
    last_dir: Direction,
    body: VecDeque<Point>,
    foods_eaten: u32,
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            dir: Direction::Right,
            last_dir: Direction::Right,
            body: VecDeque::from([Point::new(10, 10), Point::new(11, 10)]),
            foods_eaten: 0,
        }
    }

    pub fn change_dir(&mut self, c: char) {
        match c {
            'w' => {
                if self.last_dir != Direction::Down {
                    self.dir = Direction::Up;
                }
            }
            'a' => {
                if self.last_dir != Direction::Right {
                    self.dir = Direction::Left
                }
            }
            's' => {
                if self.last_dir != Direction::Up {
                    self.dir = Direction::Down
                }
            }
            'd' => {
                if self.last_dir != Direction::Left {
                    self.dir = Direction::Right
                }
            }
            _ => (),
        };
    }

    pub fn draw_snake(&self, win: &Window) {
        for body_piece in &self.body {
            win.mv(body_piece.get_y(), body_piece.get_x());
            win.addch('S');
        }
    }

    pub fn get_head(&self) -> Point {
        self.body[0]
    }

    pub fn check_for_self_collision(&self) -> bool {
        for i in 1..self.body.len() {
            if self.body[i] == self.get_head() {
                return true;
            }
        }
        return false;
    }

    pub fn check_for_collision_with(&self, other: Point) -> bool {
        for i in 1..self.body.len() {
            if self.body[i] == other {
                return true;
            }
        }
        return false;
    }

    pub fn eat_food(&mut self) {
        self.foods_eaten += 1;
    }

    pub fn move_snake(&mut self) {
        match self.dir {
            Direction::Left => self.body.push_front(self.get_head().dec_x()),
            Direction::Right => self.body.push_front(self.get_head().inc_x()),
            Direction::Up => self.body.push_front(self.get_head().dec_y()),
            Direction::Down => self.body.push_front(self.get_head().inc_y()),
        }
        self.last_dir = self.dir.clone();
        //Add size if snake has eaten any food
        if self.foods_eaten > 0 {
            self.foods_eaten -= 1;
        } else {
            self.body.pop_back();
        }
    }
}

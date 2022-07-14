use crate::point::Point;
use pancurses::Window;
use rand::Rng;

pub struct Food {
    pos: Point,
}

impl Food {
    pub fn spawn_food_at (p: Point) -> Food{
        Food{pos:p}
    }
    
    pub fn new(width:i32, height:i32) -> Food{
        Food{pos:Point::new(rand::thread_rng().gen_range(1..width),rand::thread_rng().gen_range(1..height))}
    }

    pub fn get_pos(&self) -> Point {
        self.pos
    }
    
    pub fn draw_food(&self, win: &Window) {
        win.mv(self.pos.get_y(), self.pos.get_x());
        win.addch('F');
    }
    
}

#[derive(Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
    pub fn get_x(&self) -> i32 {
        self.x
    }
    pub fn get_y(&self) -> i32 {
        self.y
    }
    pub fn set_x(&mut self, value: i32) {
        self.x = value
    }
    pub fn set_y(&mut self, value: i32) {
        self.y = value
    }

    pub fn inc_x(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }
    pub fn inc_y(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }
    pub fn dec_x(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }
    pub fn dec_y(&self) -> Point {
        Point {
            x: self.x,
            y: self.y - 1,
        }
    }
}
impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

//pub fn inc_y(&mut self){self.y += 1}
//pub fn dec_x(&mut self){self.x -= 1}
//pub fn dec_y(&mut self){self.y -= 1}

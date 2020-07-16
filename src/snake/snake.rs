
pub enum Direction {
    Up,
    Down,
    Right,
    Left
}

pub struct Snake {
    pub direction: Direction,
    pub body: Vec<u32>,
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            direction: Direction::Down,
            body: vec![524,525,526,527,528]
            //body: vec![10,11]
        }
    }
}

pub type Position = (usize, usize);

#[derive(Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct SnakeGame {
    width: usize,
    height: usize,
    snake: Vec<Position>, // Head is the first item and Tail is the last
    direction: Direction,
    food: Position,
}

impl SnakeGame {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            snake: vec![((width - 2).max(0), height / 2)],
            direction: Direction::Left,
            food: (2.min(width - 1), height / 2),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::SnakeGame;
    #[test]
    fn test() {
        println!("{:?}", SnakeGame::new(10, 10));
    }
}

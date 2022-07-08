mod random;
use std::{collections::VecDeque, vec};

use random::random_range;

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
    snake: VecDeque<Position>, // Head is the first item and Tail is the last
    direction: Direction,
    food: Position,
    endgame: bool,
}

impl SnakeGame {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            snake: vec![((width - 2).max(0), height / 2)].into_iter().collect(),
            direction: Direction::Left,
            food: (2.min(width - 1), height / 2),
            endgame: false,
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        match (&self.direction, direction) {
            (Direction::Up, Direction::Up)
            | (Direction::Up, Direction::Down)
            | (Direction::Right, Direction::Right)
            | (Direction::Right, Direction::Left)
            | (Direction::Down, Direction::Up)
            | (Direction::Down, Direction::Down)
            | (Direction::Left, Direction::Right)
            | (Direction::Left, Direction::Left) => {}
            (_, direction) => self.direction = direction,
        }
    }
    pub fn is_valid(&mut self, (x, y): Position) -> bool {
        x < self.width && y < self.height
    }
    pub fn tick(&mut self) {
        // endgame
        if self.endgame && self.snake.len() == 0 {
            return;
        }
        // Move

        let (x, y) = self.snake[0];
        let new_head = match &self.direction {
            Direction::Up => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
        };

        if !self.is_valid(new_head) || self.snake.contains(&new_head) {
            self.endgame = true;
        } else {
            if new_head != self.food {
                self.snake.pop_back();
            } else {
                //| Identify a valid food position
                let free_positions = (0..self.height)
                    .flat_map(|y| (0..self.width).map(move |x| (x, y)))
                    .filter(|pos| !self.snake.contains(pos))
                    .collect::<Vec<_>>();

                //| Ensure that the food is not a part of the existing snake
                if free_positions.is_empty() {
                    self.endgame = true;
                    return;
                }

                //| Randomly generate a new food position
                self.food = free_positions[random_range(0, free_positions.len())];
            }
        }
        self.snake.push_front(new_head);
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

use super::Direction;
use std::ops::{Add, AddAssign, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Position([f32; 2]);
impl Position {
    pub fn new(x: f32, y: f32) -> Position {
        Position([x, y])
    }

    pub fn x(&self) -> f32 {
        self.0[0]
    }

    pub fn y(&self) -> f32 {
        self.0[1]
    }

    pub fn get_data(&self) -> [f32; 2] {
        self.0
    }

    pub fn distance(self, rhs: Self) -> f32 {
        self.distance_squared(rhs).sqrt()
    }

    pub fn distance_squared(self, rhs: Self) -> f32 {
        (self.0[0] - rhs.0[0]).powi(2) + (self.0[1] - rhs.0[1]).powi(2)
    }
}

impl Sub<Position> for Position {
    type Output = Direction;
    fn sub(self, rhs: Self) -> Direction {
        Direction::new(self.x() - rhs.x(), self.y() - rhs.y())
    }
}

impl Add<Direction> for Position {
    type Output = Position;
    fn add(self, rhs: Direction) -> Self::Output {
        Position::new(self.x() + rhs.x(), self.y() + rhs.y())
    }
}

impl AddAssign<Direction> for Position {
    fn add_assign(&mut self, rhs: Direction) {
        self.0[0] += rhs.x();
        self.0[1] += rhs.y();
    }
}

use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

pub mod config;

pub struct Direction([f32; 2]);
impl Direction {
    pub fn new(x: f32, y: f32) -> Direction {
        Direction([x, y])
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

    pub fn normalize(&mut self) {
        let len = self.length();
        if len != 0.0 {
            self.0[0] = self.0[0] / len;
            self.0[1] = self.0[1] / len;
        }
    }

    pub fn length(&self) -> f32 {
        (self.0[0] * self.0[0] + self.0[1] * self.0[1]).sqrt()
    }
}

impl Mul<f32> for Direction {
    type Output = Direction;
    fn mul(self, rhs: f32) -> Self::Output {
        Direction::new(self.x() * rhs, self.y() * rhs)
    }
}

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

pub struct Color([f32; 4]);
impl Color {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color([r, g, b, a])
    }

    pub fn r(&self) -> f32 {
        self.0[0]
    }

    pub fn g(&self) -> f32 {
        self.0[1]
    }

    pub fn b(&self) -> f32 {
        self.0[2]
    }

    pub fn a(&self) -> f32 {
        self.0[3]
    }

    pub fn get_data(&self) -> [f32; 4] {
        self.0
    }
}

pub const GREEN: Color = Color::new(0.0, 1.0, 0.0, 1.0);
pub const BLUE: Color = Color::new(0.0, 0.0, 1.0, 1.0);
pub const RED: Color = Color::new(1.0, 0.0, 0.0, 1.0);
pub const BLACK: Color = Color::new(0.0, 0.0, 0.0, 1.0);

pub struct Rotation(f32);
impl Rotation {
    pub fn new_rad(rad: f32) -> Rotation {
        Rotation(rad)
    }

    pub fn new_deg(deg: f32) -> Rotation {
        Rotation(deg * (std::f32::consts::PI / 180.0))
    }

    pub fn get_rad(&self) -> f32 {
        self.0
    }

    pub fn get_deg(&self) -> f32 {
        self.0 * (180.0 / std::f32::consts::PI)
    }
}
impl Add<f32> for Rotation {
    type Output = Rotation;

    fn add(self, rhs: f32) -> Self::Output {
        Rotation::new_rad(self.get_rad() + rhs)
    }
}
impl AddAssign<f32> for Rotation {
    fn add_assign(&mut self, rhs: f32) {
        self.0 += rhs;
    }
}
impl SubAssign<f32> for Rotation {
    fn sub_assign(&mut self, rhs: f32) {
        self.0 -= rhs;
    }
}

#[derive(Copy, Clone)]
pub struct Size([f32; 2]);
impl Size {
    pub const fn new(x: f32, y: f32) -> Size {
        Size([x, y])
    }

    pub fn x(&self) -> f32 {
        self.0[0]
    }

    pub fn y(&self) -> f32 {
        self.0[1]
    }
}

use crate::animals::ant::Ant;
use crate::items::food::FoodPellet;
use std::cell::RefCell;
use std::rc::Weak;

pub enum Vision {
    Ant(Weak<RefCell<Ant>>, f32),
    Food(Weak<RefCell<FoodPellet>>, f32),
}

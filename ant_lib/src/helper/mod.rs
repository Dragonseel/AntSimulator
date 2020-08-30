use crate::animals::ant::Ant;
use crate::items::food::FoodPellet;
use std::cell::RefCell;
use std::rc::Weak;

pub mod config;

mod color;
mod rotation;

mod vector2d;

pub use color::*;
pub use rotation::*;
pub use vector2d::*;

pub enum Vision {
    Ant(Weak<RefCell<Ant>>, f32),
    Food(Weak<RefCell<FoodPellet>>, f32),
}

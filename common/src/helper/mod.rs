use crate::animals::ant::Ant;
use crate::items::food::FoodPellet;

pub mod config;

mod color;
mod rotation;

mod vector2d;

pub use color::*;
pub use rotation::*;
pub use vector2d::*;

pub enum Vision {
    Ant(Ant, f32),
    Food(FoodPellet, f32),
}

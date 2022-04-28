use crate::animals::ant::Ant;
use crate::items::food::FoodPellet;

pub mod config;

mod color;
mod rotation;

mod vector2d;

pub use color::*;
pub use rotation::*;
pub use vector2d::*;

#[repr(C)]
pub struct Vision {
    pub object: SeenObject,
    pub distance: f32,
}

#[repr(C)]
pub enum SeenObject {
    Ant(Ant),
    Food(FoodPellet),
}

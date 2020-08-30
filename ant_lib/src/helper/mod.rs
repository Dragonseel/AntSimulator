use crate::animals::ant::Ant;
use crate::items::food::FoodPellet;
use std::cell::RefCell;
use std::rc::Weak;

pub mod config;

mod color;
mod direction;
mod position;
mod rotation;
mod size;

pub use color::*;
pub use direction::*;
pub use position::*;
pub use rotation::*;
pub use size::*;

pub enum Vision {
    Ant(Weak<RefCell<Ant>>, f32),
    Food(Weak<RefCell<FoodPellet>>, f32),
}

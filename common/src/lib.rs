use animals::ant::{Action, Ant};
use helper::Vision;

pub mod animals;
pub mod helper;
pub mod items;

pub trait AntLogic {
    fn update(&self, ant: &Ant, vision: &[Vision]) -> Action;
}

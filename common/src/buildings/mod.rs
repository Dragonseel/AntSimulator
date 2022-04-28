use crate::helper::Vector2D;

#[repr(C)]
pub enum NestAction {
    Nothing,
    SpawnAnts(usize),
}

#[repr(C)]
pub struct Nest {
    pub id: usize,
    pub energy: u32,
    pub pos: Vector2D,
}

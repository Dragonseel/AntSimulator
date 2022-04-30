use crate::helper::Vector2D;

#[repr(C)]
pub enum NestAction {
    Nothing,
    SpawnAnts(usize),
}

#[repr(C)]
pub struct Nest {
    pub id: usize,
    pub pos: Vector2D,
    pub energy: u32,
    pub rounds_to_energy_loss: u32,
}

impl Nest {
    pub fn is_alive(&self) -> bool {
        self.energy > 0
    }
}

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

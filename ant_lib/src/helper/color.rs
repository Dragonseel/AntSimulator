#[derive(Copy, Clone)]
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

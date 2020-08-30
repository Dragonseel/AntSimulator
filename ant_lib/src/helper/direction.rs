use std::ops::Mul;

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

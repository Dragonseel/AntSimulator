use std::ops::{Add, AddAssign, SubAssign};

#[derive(Copy, Clone)]
pub struct Rotation(f32);
impl Rotation {
    pub fn new_rad(rad: f32) -> Rotation {
        Rotation(rad)
    }

    pub fn new_deg(deg: f32) -> Rotation {
        Rotation(deg * (std::f32::consts::PI / 180.0))
    }

    pub fn get_rad(&self) -> f32 {
        self.0
    }

    pub fn get_deg(&self) -> f32 {
        self.0 * (180.0 / std::f32::consts::PI)
    }
}

impl Add<f32> for Rotation {
    type Output = Rotation;

    fn add(self, rhs: f32) -> Self::Output {
        Rotation::new_rad(self.get_rad() + rhs)
    }
}

impl AddAssign<f32> for Rotation {
    fn add_assign(&mut self, rhs: f32) {
        self.0 += rhs;
    }
}

impl SubAssign<f32> for Rotation {
    fn sub_assign(&mut self, rhs: f32) {
        self.0 -= rhs;
    }
}

use std::ops::{Add, AddAssign, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vector2D([f32; 2]);
impl Vector2D {
    pub fn new(x: f32, y: f32) -> Vector2D {
        Vector2D([x, y])
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

    pub fn distance(self, rhs: Self) -> f32 {
        self.distance_squared(rhs).sqrt()
    }

    pub fn distance_squared(self, rhs: Self) -> f32 {
        (self.0[0] - rhs.0[0]).powi(2) + (self.0[1] - rhs.0[1]).powi(2)
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

impl Sub<Vector2D> for Vector2D {
    type Output = Vector2D;
    fn sub(self, rhs: Self) -> Vector2D {
        Vector2D::new(self.x() - rhs.x(), self.y() - rhs.y())
    }
}

impl Add<Vector2D> for Vector2D {
    type Output = Vector2D;
    fn add(self, rhs: Vector2D) -> Self::Output {
        Vector2D::new(self.x() + rhs.x(), self.y() + rhs.y())
    }
}

impl AddAssign<Vector2D> for Vector2D {
    fn add_assign(&mut self, rhs: Vector2D) {
        self.0[0] += rhs.x();
        self.0[1] += rhs.y();
    }
}

impl Mul<f32> for Vector2D {
    type Output = Vector2D;
    fn mul(self, rhs: f32) -> Self::Output {
        Vector2D::new(self.x() * rhs, self.y() * rhs)
    }
}

impl Mul<Vector2D> for f32 {
    type Output = Vector2D;
    fn mul(self, rhs: Vector2D) -> Self::Output {
        Vector2D::new(self * rhs.x(), self * rhs.y())
    }
}
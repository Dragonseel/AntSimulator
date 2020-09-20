use crate::helper::*;
use crate::support::camera::Camera;
use glium::{Display, Frame};

pub struct FoodPellet {
    position: Vector2D,
    nutrition: u32,
    bite_size: u32,

    //technical
    rect: crate::primitives::rectangle::Rectangle,
}

impl FoodPellet {
    pub fn new(nut:u32, display: &Display, bite_size: u32) -> FoodPellet {
        FoodPellet {
            position: Vector2D::new(0.0, 0.0),
            nutrition: nut,
            bite_size,
            rect: crate::primitives::rectangle::Rectangle::new(
                Vector2D::new(5.0, 5.0),
                Vector2D::new(2.5, 2.5),
                Rotation::new_rad(0.0),
                BLUE,
                display,
            ),
        }
    }

    pub fn new_at_pos(pos: Vector2D, nut:u32, display: &Display, bite_size: u32) -> FoodPellet {
        FoodPellet {
            position: pos,
            nutrition: nut,
            bite_size,
            rect: crate::primitives::rectangle::Rectangle::new(
                Vector2D::new(5.0, 5.0),
                pos,
                Rotation::new_rad(0.0),
                BLUE,
                display,
            ),
        }
    }

    pub fn draw(&mut self, target: &mut Frame, cam: &Camera) {
        self.rect.draw(target, cam);
    }

    pub fn get_position(&self) -> Vector2D {
        self.position
    }

    pub fn get_eaten(&mut self) -> u32 {
        if self.nutrition >= self.bite_size {
            self.nutrition -= self.bite_size;
            self.bite_size
        } else {
            self.nutrition
        }
    }

    pub fn is_some_left(&self) -> bool {
        self.nutrition > 0
    }
}

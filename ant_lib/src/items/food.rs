use crate::{helper::*, support::camera::Camera};
use glium::{Display, Frame};
pub struct FoodPellet {
    position: Position,
    nutrition: i32,
    bite_size: i32,

    //technical
    rect: crate::primitives::rectangle::Rectangle,
}

impl FoodPellet {
    pub fn new(nut: i32, display: &Display, bite_size: i32) -> FoodPellet {
        FoodPellet {
            position: Position::new(0.0, 0.0),
            nutrition: nut,
            bite_size,
            rect: crate::primitives::rectangle::Rectangle::new(
                [5.0, 5.0],
                [0.0, 0.0],
                0.0f32,
                BLUE.get_data(),
                display,
            ),
        }
    }

    pub fn new_at_pos(pos: Position, nut: i32, display: &Display, bite_size: i32) -> FoodPellet {
        FoodPellet {
            position: pos,
            nutrition: nut,
            bite_size,
            rect: crate::primitives::rectangle::Rectangle::new(
                [5.0, 5.0],
                [pos.x(), pos.y()],
                0.0f32,
                BLUE.get_data(),
                display,
            ),
        }
    }

    pub fn draw(&mut self, target: &mut Frame, cam: &Camera) {
        self.rect.draw(target, cam);
    }

    pub fn get_position(&self) -> Position {
        self.position
    }

    pub fn get_eaten(&mut self) -> i32 {
        if self.nutrition > 0 {
            self.nutrition -= self.bite_size;
            self.bite_size
        } else {
            0
        }
    }

    pub fn is_some_left(&self) -> bool {
        self.nutrition > 0
    }
}

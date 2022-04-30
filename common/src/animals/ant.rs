use crate::helper::*;
use crate::items::food::FoodPellet;

#[repr(C)]
pub enum AntAction {
    Nothing,
    RotateLeft(f32),
    RotateRight(f32),
    GoForward(f32),
    EatFood(FoodPellet),
}

impl std::fmt::Display for AntAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            AntAction::Nothing => write!(f, "Nothing"),
            AntAction::RotateLeft(angle) => write!(f, "RotateLeft({})", angle),
            AntAction::RotateRight(angle) => write!(f, "RotateRight({})", angle),
            AntAction::GoForward(length) => write!(f, "GoForward({})", length),
            AntAction::EatFood(_) => write!(f, "EatFood"),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Ant {
    pub position: Vector2D,
    pub energy: u32,
    pub id: usize,
    pub color: Color,
    pub rotation: Rotation,
    pub size: Vector2D,
    pub speed: f32,
    pub angular_speed: f32,
    pub max_energy: u32,
    pub mouth_reach: f32,
    pub rounds_to_energy_loss: u32,
}

impl Ant {
    pub fn is_alive(&self) -> bool {
        self.energy != 0
    }
}

// Getters
// impl Ant {
//     fn get_mouth_position(&self) -> Vector2D {
//         Vector2D::new(self.position.x()+0.5*self.size.x(), self.position.y())
//     }
// }

// Actions
impl Ant {
    pub fn eat_food(&mut self, food: &mut FoodPellet) {
        // let food_pos = food.borrow().get_position();
        // let food_dir = food_pos - self.get_mouth_position();
        // let dist = food_dir.length();

        // if dist < self.mouth_reach {
        //     self.energy += food.borrow_mut().get_eaten();
        //     self.energy = self.energy.min(self.max_energy);
        // }
        self.energy += food.get_eaten();
        self.energy = self.energy.min(self.max_energy);
    }

    pub fn go_forward(&mut self, length: f32) {
        let movement_amount = self.speed.min(length).max(-self.speed);

        self.position += Vector2D::new(
            self.rotation.get_rad().cos(),
            -self.rotation.get_rad().sin(),
        ) * movement_amount;
    }

    pub fn rotate_left(&mut self, angle: f32) {
        self.rotation -= self.angular_speed.min(angle);
    }

    pub fn rotate_right(&mut self, angle: f32) {
        self.rotation += self.angular_speed.min(angle);
    }
}

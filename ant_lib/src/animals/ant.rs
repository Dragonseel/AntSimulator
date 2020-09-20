use glium::{Display, Frame};

use std::cell::RefCell;
use std::{rc::Rc, time::Duration};

use crate::helper::{config::AntConfig, *};
use crate::items::food::FoodPellet;
use crate::primitives::rectangle::Rectangle;
use crate::support::camera::Camera;
use crate::AntLogic;

pub enum Action {
    Nothing,
    RotateLeft(f32),
    RotateRight(f32),
    GoForward(f32),
    EatFood(Rc<RefCell<FoodPellet>>),
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Action::Nothing => write!(f, "Nothing"),
            Action::RotateLeft(angle) => write!(f, "RotateLeft({})", angle),
            Action::RotateRight(angle) => write!(f, "RotateRight({})", angle),
            Action::GoForward(length) => write!(f, "GoForward({})", length),
            Action::EatFood(_) => write!(f, "EatFood"),
        }
    }
}

pub struct Ant {
    pub position: Vector2D,
    pub energy: u32,
    pub id: i32,
    color: Color,
    pub rotation: Rotation,
    size: Vector2D,
    speed: f32,
    pub angular_speed: f32,
    max_energy: u32,
    pub mouth_reach: f32,
    // Graphics
    rect: Rectangle,
}

impl Ant {
    pub fn new(id: i32, config: &AntConfig, display: &Display) -> Ant {
        let size = Vector2D::new(16.0, 7.0);
        let position = Vector2D::new(50.0, 50.0);
        let rotation = Rotation::new_rad(0.0f32);
        let color = Color::new(1.0f32, 0.0f32, 0.0f32, 1.0f32);

        Ant {
            id,
            position,
            color,
            rotation,
            size: size,
            speed: config.speed,
            angular_speed: config.angular_speed,
            energy: config.max_energy,
            max_energy: config.max_energy,
            mouth_reach: config.mouth_reach,
            rect: Rectangle::new(size, position, rotation, color, display),
        }
    }

    pub fn new_at(id: i32, config: &AntConfig, pos: Vector2D, display: &Display) -> Ant {
        let mut ant = Ant::new(id, config, display);
        ant.position = pos;
        ant
    }

    pub fn is_alive(&self) -> bool {
        self.energy != 0
    }

    pub fn update<F>(&mut self, other_elements: Vec<Vision>, ant_func: &mut F, _dt: Duration)
    where
        F: AntLogic,
    {
        match ant_func.update(&self, &other_elements) {
            Action::Nothing => {}
            Action::GoForward(length) => self.go_forward(length),
            Action::RotateLeft(angle) => self.rotate_left(angle),
            Action::RotateRight(angle) => self.rotate_right(angle),
            Action::EatFood(food) => self.eat_food(food),
        }
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
    fn eat_food(&mut self, food: Rc<RefCell<FoodPellet>>) {
        // let food_pos = food.borrow().get_position();
        // let food_dir = food_pos - self.get_mouth_position();
        // let dist = food_dir.length();

        // if dist < self.mouth_reach {
        //     self.energy += food.borrow_mut().get_eaten();
        //     self.energy = self.energy.min(self.max_energy);
        // }
        self.energy += food.borrow_mut().get_eaten();
        self.energy = self.energy.min(self.max_energy);
    }

    fn go_forward(&mut self, length: f32) {
        let movement_amount = self.speed.min(length).max(-self.speed);

        self.position += Vector2D::new(
            self.rotation.get_rad().cos(),
            -self.rotation.get_rad().sin(),
        ) * movement_amount;
    }

    fn rotate_left(&mut self, angle: f32) {
        self.rotation -= self.angular_speed.min(angle);
    }

    fn rotate_right(&mut self, angle: f32) {
        self.rotation += self.angular_speed.min(angle);
    }
}

// Graphics
impl Ant {
    pub fn draw(&mut self, target: &mut Frame, cam: &Camera) {
        self.rect.position = self.position; //- 0.5 * self.size; // * self.rotation);
        self.rect.rotation = self.rotation;
        self.rect.draw(target, cam);
    }
}

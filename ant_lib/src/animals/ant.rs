use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use std::cell::RefCell;
use std::{rc::Rc, time::Duration};

use crate::helper::*;
use crate::items::food::FoodPellet;
use crate::{primitives::rectangle::Rectangle, support::camera::Camera, AntLogic};
use glium::{Display, Frame};

pub enum Action {
    Nothing,
    RotateLeft(f32),
    RotateRight(f32),
    GoForward(f32),
    EatFood(Rc<RefCell<FoodPellet>>),
}

impl Distribution<Action> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Action {
        match rng.gen_range(0, 6) {
            0 => Action::RotateLeft(90.0),
            1 => Action::RotateRight(90.0),
            2 => Action::GoForward(100.0),
            3 => Action::GoForward(100.0),
            4 => Action::GoForward(100.0),
            5 => Action::GoForward(100.0),
            _ => Action::GoForward(100.0),
        }
    }
}

pub struct Ant {
    pub position: Position,
    color: Color,
    pub rotation: Rotation,
    size: Size,
    pub speed: f32,
    pub angular_speed: f32,
    pub energy: i32,
    pub mouth_reach: f32,
    pub id: i32,
    // technical
    rect: Rectangle,
}

impl Ant {
    pub fn new(display: &Display, id: i32) -> Ant {
        Ant {
            id,
            position: Position::new(50.0, 50.0),
            color: Color::new(1.0f32, 0.0f32, 0.0f32, 1.0f32),
            rotation: Rotation::new_rad(0.0f32),
            size: Size::new(15.0f32, 7.0f32),
            speed: 1.0,
            angular_speed: 0.1,
            energy: 1000,
            mouth_reach: 10.0,
            rect: Rectangle::new([15.0, 7.0], [50.0, 50.0], 0.0, RED.get_data(), display),
        }
    }

    pub fn eat_food(&mut self, food: Rc<RefCell<FoodPellet>>) {
        let food_pos = food.borrow().get_position();
        let food_dir = food_pos - self.position;
        let dist = food_dir.length();

        if dist < self.mouth_reach {
            self.energy += food.borrow_mut().get_eaten();
            if self.energy > 1000 {
                self.energy = 1000
            }
        }
    }

    pub fn go_forward(&mut self, length: f32) {
        let movement_amount = if length > self.speed {
            self.speed
        } else if length < -self.speed {
            -self.speed
        } else {
            length
        };

        self.position += Direction::new(
            self.rotation.get_rad().cos(),
            -self.rotation.get_rad().sin(),
        ) * movement_amount;
    }

    pub fn rotate_left(&mut self, angle: f32) {
        if angle < self.angular_speed {
            self.rotation -= angle;
        } else {
            self.rotation -= self.angular_speed;
        }
    }

    pub fn rotate_right(&mut self, angle: f32) {
        if angle < self.angular_speed {
            self.rotation += angle;
        } else {
            self.rotation += self.angular_speed;
        }
    }

    pub fn is_alive(&self) -> bool {
        self.energy > 0
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

impl Ant {
    pub fn draw(&mut self, target: &mut Frame, cam: &Camera) {
        self.rect.position = self.position.get_data();
        self.rect.rotation = self.rotation.get_rad();
        self.rect.draw(target, cam);
    }
}

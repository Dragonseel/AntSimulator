use ant_lib::prelude::*;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use rand::Rng;

struct Logic {}

impl AntLogic for Logic {
    fn update(&self, ant: &Ant, vision: &Vec<Vision>) -> Action {
        let mut closest_food: Option<&Weak<RefCell<FoodPellet>>> = None;
        let mut min_dist = std::f32::MAX;

        for item in vision {
            match item {
                Vision::Ant(_other_ant, _dist) => {
                    // println!("Found an ant.");
                }
                Vision::Food(food, dist) => {
                    if *dist < min_dist {
                        closest_food = Some(food);
                        min_dist = *dist;
                    }
                }
            }
        }

        let action = if let Some(food) = closest_food {
            if let Some(food) = food.upgrade() {
                if min_dist < ant.get_mouth_reach() {
                    // Ant is at Food

                    Action::EatFood(Rc::clone(&food))
                } else {
                    // Go To Food

                    let food_pos = food.borrow().get_position();

                    let own_direction = Vector2D::new(
                        ant.get_rotation().get_rad().cos(),
                        -ant.get_rotation().get_rad().sin(),
                    );

                    let mut food_direction: Vector2D = food_pos - ant.position;
                    food_direction.normalize();

                    let angle = food_direction.y().atan2(food_direction.x())
                        - own_direction.y().atan2(own_direction.x());

                    if angle > 0.5 * ant.get_angular_speed() {
                        Action::RotateLeft(angle.abs())
                    } else if angle < -0.5 * ant.get_angular_speed() {
                        Action::RotateRight(angle.abs())
                    } else {
                        Action::GoForward(100.0)
                    }
                }
            } else {
                Action::Nothing
            }
        } else {
            match rand::thread_rng().gen_range(0, 7) {
                0 => Action::RotateLeft(90.0),
                1 => Action::RotateRight(90.0),
                2 => Action::GoForward(100.0),
                3 => Action::GoForward(100.0),
                4 => Action::GoForward(100.0),
                5 => Action::GoForward(100.0),
                _ => Action::GoForward(100.0),
            }
        };

        action
    }
}

fn main() {
    let logic = Logic {};

    ant_lib::start_simulation(logic);
}

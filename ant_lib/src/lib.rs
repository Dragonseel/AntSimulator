use std::sync::Mutex;

use common::{
    animals::ant::{Ant, AntAction},
    buildings::{Nest, NestAction},
    helper::{SeenObject, Vector2D, Vision},
    items::food::FoodPellet,
};
use rand::Rng;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref FRAMECOUNTER: std::sync::Mutex<usize> = Mutex::new(0);
}

#[no_mangle]
pub extern "C" fn ant_update(ant: &Ant, vision: &Vec<Vision>) -> AntAction {
    let mut closest_food: Option<FoodPellet> = None;
    let mut min_dist = std::f32::MAX;

    for item in vision.iter() {
        match item.object {
            SeenObject::Ant(_other_ant) => {
                // println!("Found an ant.");
            }
            SeenObject::Food(food) => {
                if item.distance < min_dist {
                    closest_food = Some(food);
                    min_dist = item.distance;
                }
            }
        }
    }

    if let Some(food) = closest_food {
        if min_dist < ant.mouth_reach {
            // Ant is at Food
            AntAction::EatFood(food)
        } else {
            // Go To Food
            let food_pos = food.get_position();

            let own_direction =
                Vector2D::new(ant.rotation.get_rad().cos(), -ant.rotation.get_rad().sin());

            let mut food_direction: Vector2D = food_pos - ant.position;
            food_direction.normalize();

            let angle = food_direction.y().atan2(food_direction.x())
                - own_direction.y().atan2(own_direction.x());

            if angle > 0.5 * ant.angular_speed {
                AntAction::RotateLeft(angle.abs())
            } else if angle < -0.5 * ant.angular_speed {
                AntAction::RotateRight(angle.abs())
            } else {
                AntAction::GoForward(100.0)
            }
        }
    } else {
        match rand::thread_rng().gen_range::<u8>(0, 7) {
            0 => AntAction::RotateLeft(90.0),
            1 => AntAction::RotateRight(90.0),
            2 => AntAction::GoForward(100.0),
            3 => AntAction::GoForward(100.0),
            4 => AntAction::GoForward(100.0),
            5 => AntAction::GoForward(100.0),
            _ => AntAction::GoForward(100.0),
        }
    }
}

#[no_mangle]
pub extern "C" fn nest_update(nest: &Nest) -> NestAction {
    if nest.energy > 500 && *FRAMECOUNTER.lock().unwrap() >= 60 {
        *FRAMECOUNTER.lock().unwrap() = 0;
        NestAction::SpawnAnts(1)
    } else {
        *FRAMECOUNTER.lock().unwrap() += 1;
        NestAction::Nothing
    }
}

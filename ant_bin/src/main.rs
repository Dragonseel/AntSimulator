use std::cell::RefCell;
use std::rc::Rc;

use common::animals::ant::{Action, Ant};
use common::helper::{Vector2D, Vision};
use common::items::food::FoodPellet;

use rand::Rng;
use support::simulator::Simulator;

mod drawables;
mod ground;
mod primitives;
mod support;

struct Logic {}

impl common::AntLogic for Logic {
    fn update(&self, ant: &Ant, vision: &[Vision]) -> Action {
        let mut closest_food: Option<FoodPellet> = None;
        let mut min_dist = std::f32::MAX;

        for item in vision.iter() {
            match item {
                Vision::Ant(_other_ant, _dist) => {
                    // println!("Found an ant.");
                }
                Vision::Food(food, dist) => {
                    if *dist < min_dist {
                        closest_food = Some(*food);
                        min_dist = *dist;
                    }
                }
            }
        }

        let action = if let Some(food) = closest_food {
            if min_dist < ant.mouth_reach {
                // Ant is at Food
                Action::EatFood(food)
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
                    Action::RotateLeft(angle.abs())
                } else if angle < -0.5 * ant.angular_speed {
                    Action::RotateRight(angle.abs())
                } else {
                    Action::GoForward(100.0)
                }
            }
        } else {
            match rand::thread_rng().gen_range(0..7) {
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

    let system = crate::support::init(file!());

    let app = 
        Rc::new(RefCell::new(Simulator::new(&system.display, logic)));

    let app_ui = Rc::clone(&app);
    let app_update = Rc::clone(&app);
    let app_draw = Rc::clone(&app);

    system.main_loop(
        move |_run, ui| {
            crate::support::ui::camera_control(ui, &app_ui);

            crate::support::ui::simulation_control(ui, &app_ui);

            crate::support::ui::statistics(ui, &app_ui);
        },
        move |dt, display| {
            app_update.borrow_mut().update(dt, display);
        },
        move |target, _display| {
            app_draw.borrow_mut().draw(target);
        },
    );
}

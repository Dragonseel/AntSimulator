use std::{collections::HashMap, sync::Mutex};

use common::{
    animals::ant::{Ant, AntAction},
    buildings::{Nest, NestAction},
    helper::{SeenObject, Vector2D, Vision},
    items::food::FoodPellet,
};
use rand::Rng;

#[macro_use]
extern crate lazy_static;

#[derive(Clone, Copy)]
enum AntObjective {
    Searching,
    Eating,
    GoToFood,
    BringingFoodHome,
}

struct AntMemory {
    objective: AntObjective,
    nest_pos: Option<Vector2D>,
}

impl AntMemory {
    fn new() -> AntMemory {
        AntMemory {
            objective: AntObjective::Searching,
            nest_pos: None,
        }
    }
}

impl Default for AntMemory {
    fn default() -> Self {
        AntMemory::new()
    }
}

lazy_static! {
    static ref FRAMECOUNTER: std::sync::Mutex<usize> = Mutex::new(0);
    static ref ANTMEMORY: std::sync::Mutex<HashMap<usize, AntMemory>> = Mutex::new(HashMap::new());
}

fn switch_to_objective(ant: &usize, objective: AntObjective) {
    ANTMEMORY.lock().unwrap().get_mut(ant).unwrap().objective = objective;
}

#[no_mangle]
pub extern "C" fn ant_update(ant: &Ant, vision: &Vec<Vision>) -> AntAction {
    let mut closest_food: Option<FoodPellet> = None;
    let mut min_dist = std::f32::MAX;

    if ANTMEMORY
        .lock()
        .unwrap()
        .entry(ant.id)
        .or_default()
        .nest_pos
        .is_none()
    {
        // This is the ants first update tick, it has not moved, so it is at the nest
        // The ant can remember this position as the nest position
        ANTMEMORY
            .lock()
            .unwrap()
            .entry(ant.id)
            .or_default()
            .nest_pos = Some(ant.position);
    }

    loop {
        // Determine the closest item
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

        let current_objective = ANTMEMORY
            .lock()
            .unwrap()
            .entry(ant.id)
            .or_default()
            .objective;

        match current_objective {
            AntObjective::Searching => {
                if closest_food.is_some() {
                    switch_to_objective(&ant.id, AntObjective::GoToFood);
                } else {
                    // Move randomly around
                    match rand::thread_rng().gen_range::<u8>(0, 7) {
                        0 => return AntAction::RotateLeft(90.0),
                        1 => return AntAction::RotateRight(90.0),
                        2 => return AntAction::GoForward(100.0),
                        3 => return AntAction::GoForward(100.0),
                        4 => return AntAction::GoForward(100.0),
                        5 => return AntAction::GoForward(100.0),
                        _ => return AntAction::GoForward(100.0),
                    }
                }
            }
            AntObjective::GoToFood => {
                if let Some(food) = closest_food {
                    if min_dist < ant.mouth_reach {
                        // Ant is at Food
                        switch_to_objective(&ant.id, AntObjective::Eating);
                    } else {
                        // Go To Food
                        let food_pos = food.get_position();

                        let own_direction = Vector2D::new(
                            ant.rotation.get_rad().cos(),
                            -ant.rotation.get_rad().sin(),
                        );

                        let mut food_direction: Vector2D = food_pos - ant.position;
                        food_direction.normalize();

                        let angle = food_direction.y().atan2(food_direction.x())
                            - own_direction.y().atan2(own_direction.x());

                        if angle > 0.5 * ant.angular_speed {
                            return AntAction::RotateLeft(angle.abs());
                        } else if angle < -0.5 * ant.angular_speed {
                            return AntAction::RotateRight(angle.abs());
                        } else {
                            return AntAction::GoForward(100.0);
                        }
                    }
                } else {
                    switch_to_objective(&ant.id, AntObjective::Searching);
                }
            }
            AntObjective::Eating => {
                // Eat and if satiated decide what to do next
                if let Some(food) = closest_food {
                    if ant.energy >= (ant.max_energy - food.bite_size) {
                        switch_to_objective(&ant.id, AntObjective::BringingFoodHome);

                        return AntAction::CarryFood(food);
                    } else {
                        return AntAction::EatFood(food);
                    }
                } else {
                    switch_to_objective(&ant.id, AntObjective::Searching);
                }
            }
            AntObjective::BringingFoodHome => {
                if ant.energy < ant.max_energy / 5 {
                    // We need to eat some of our carried food
                    switch_to_objective(&ant.id, AntObjective::Eating);
                    return AntAction::UnloadFood;
                }

                let nest_option = ANTMEMORY
                    .lock()
                    .unwrap()
                    .entry(ant.id)
                    .or_default()
                    .nest_pos;

                if let Some(nest_pos) = nest_option {
                    if ant.position.distance(nest_pos) <= ant.mouth_reach {
                        // We are there, we can unload at the nest and continue searching
                        println!("Ant wants to unload.");
                        switch_to_objective(&ant.id, AntObjective::Searching);
                        println!("Returning UnloadFood");
                        return AntAction::UnloadFood;
                    }

                    // Go To Nest
                    let own_direction =
                        Vector2D::new(ant.rotation.get_rad().cos(), -ant.rotation.get_rad().sin());

                    let mut nest_direction: Vector2D = nest_pos - ant.position;
                    nest_direction.normalize();

                    let angle = nest_direction.y().atan2(nest_direction.x())
                        - own_direction.y().atan2(own_direction.x());

                    if angle > 0.5 * ant.angular_speed {
                        return AntAction::RotateLeft(angle.abs());
                    } else if angle < -0.5 * ant.angular_speed {
                        return AntAction::RotateRight(angle.abs());
                    } else {
                        return AntAction::GoForward(100.0);
                    }
                }

                return AntAction::Nothing; // This ant has forgotten where the nest is, it is doomed.
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn nest_update(nest: &Nest) -> NestAction {
    /*
    // for debugging, only spawn a single ant ever
    if *FRAMECOUNTER.lock().unwrap() == 0 {
        *FRAMECOUNTER.lock().unwrap() += 1;
        NestAction::SpawnAnts(1)
    } else {
        NestAction::Nothing
    }
    */

    if nest.energy > 1000 && *FRAMECOUNTER.lock().unwrap() >= 60 {
        *FRAMECOUNTER.lock().unwrap() = 0;
        NestAction::SpawnAnts(1)
    } else {
        *FRAMECOUNTER.lock().unwrap() += 1;
        NestAction::Nothing
    }
}

#[no_mangle]
pub extern "C" fn reset() {
    println!("Resetting");
    *FRAMECOUNTER.lock().unwrap() = 0;

    ANTMEMORY.lock().unwrap().clear();
}

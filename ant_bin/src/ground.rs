use crate::drawables::{AntDrawable, FoodPelletDrawable, NestDrawable};
use crate::support::camera::Camera;
use crate::{AntFunc, NestFunc};
use common::animals::ant::AntAction;
use common::helper::*;

use rand::prelude::*;

use config::Config;
use glium::{Display, Frame};
use std::time::Duration;

pub struct Ground {
    size: Vector2D,
    food: Vec<FoodPelletDrawable>,
    ants: Vec<AntDrawable>,
    nests: Vec<NestDrawable>,
    food_timer: i32,

    pub config: Config,

    // technical
    next_food_id: usize,
    next_colony_id: usize,
    next_ant_id: usize,
    rng: ThreadRng,
    rect: crate::primitives::rectangle::Rectangle,
}

impl Ground {
    pub fn new_empty(size: Vector2D, display: &Display) -> Ground {
        let config = Config::new();

        Ground {
            food: Vec::new(),
            ants: Vec::new(),
            nests: Vec::new(),
            size,
            food_timer: config.food.spawn_time,
            next_food_id: 0,
            next_colony_id: 0,
            next_ant_id: 0,
            rng: rand::thread_rng(),
            rect: crate::primitives::rectangle::Rectangle::new(
                size,
                0.5 * size, //Vector2D::new(0.0, 0.0),
                Rotation::new_rad(0.0),
                GREEN,
                display,
            ),
            config,
        }
    }
}

impl Ground {
    pub fn num_ants(&self) -> usize {
        self.ants.len()
    }

    pub fn num_foods(&self) -> usize {
        self.food.len()
    }

    pub fn num_colonies(&self) -> usize {
        self.nests.len()
    }

    pub fn start_new_round(&mut self, display: &Display) {
        self.ants.clear();
        self.food.clear();
        self.nests.clear();

        self.next_food_id = 0;
        self.next_colony_id = 0;
        self.next_ant_id = 0;

        self.generate_colonies(display);
        self.generate_random_food(self.config.food.start_amount, display);
    }
}

impl Ground {
    fn generate_random_food(&mut self, amount: i32, display: &Display) {
        for _i in 0..amount {
            let x: f32 = self.rng.gen::<f32>() * self.size.x();
            let y: f32 = self.rng.gen::<f32>() * self.size.y();

            let new_food = FoodPelletDrawable::new_at_pos(
                self.next_food_id,
                Vector2D::new(x, y),
                self.config.food.nutrition,
                display,
                self.config.food.eaten_value,
            );
            self.food.push(new_food);

            self.next_food_id += 1;
        }
    }

    fn generate_ants(&mut self, nest_pos: Vector2D, amount: i32, display: &Display) {
        for _ in 0..amount {
            let ant =
                AntDrawable::new_at_pos(self.next_ant_id, &self.config.ants, nest_pos, display);
            self.ants.push(ant);
            self.next_ant_id += 1;
        }
    }

    fn generate_colonies(&mut self, display: &Display) {
        let x: f32 = self.rng.gen::<f32>() * self.size.x();
        let y: f32 = self.rng.gen::<f32>() * self.size.y();

        let new_colony = NestDrawable::new_at_pos(
            self.next_colony_id,
            Vector2D::new(x, y),
            &self.config.nests,
            display,
        );

        self.nests.push(new_colony);

        self.next_colony_id += 1;
    }

    pub fn reset_food_time(&mut self) {
        self.food_timer = self.config.food.spawn_time;
    }

    fn push_ant_into_boundary(ant_drawable: &mut AntDrawable, size: Vector2D) {
        if ant_drawable.ant.position.x() < 0.0 {
            let old_pos = ant_drawable.ant.position.y();
            ant_drawable.ant.position = Vector2D::new(0.0, old_pos);
        }

        if ant_drawable.ant.position.x() > size.x() {
            let old_pos = ant_drawable.ant.position.y();
            ant_drawable.ant.position = Vector2D::new(size.x(), old_pos);
        }

        if ant_drawable.ant.position.y() < 0.0 {
            let old_pos = ant_drawable.ant.position.x();
            ant_drawable.ant.position = Vector2D::new(old_pos, 0.0);
        }

        if ant_drawable.ant.position.y() > size.y() {
            let old_pos = ant_drawable.ant.position.x();
            ant_drawable.ant.position = Vector2D::new(old_pos, size.y());
        }
    }

    fn update_nests(&mut self, _dt: Duration, nest_func: NestFunc, display: &Display) {
        for i in 0..self.num_colonies() {
            let nest_action = nest_func(&self.nests[i].nest);

            match nest_action {
                common::buildings::NestAction::Nothing => (),
                common::buildings::NestAction::SpawnAnts(mut num) => {
                    while self.nests[i].nest.energy > 500 && num > 0 {
                        self.generate_ants(self.nests[i].nest.pos, 1, display);
                        self.nests[i].nest.energy -= 500;
                        num -= 1;
                    }
                }
            }

            self.nests[i].nest.rounds_to_energy_loss -= 1;
            if self.nests[i].nest.rounds_to_energy_loss <= 0 {
                self.nests[i].nest.energy = self.nests[i]
                    .nest
                    .energy
                    .saturating_sub(self.config.nests.energy_loss_amount); // Nests have to spend energy to be alive

                self.nests[i].nest.rounds_to_energy_loss = self.config.nests.energy_loss_rounds;
            }
        }
    }

    fn update_ants(&mut self, _dt: Duration, ant_func: AntFunc) {
        let num_ants = self.ants.len();
        let num_foods = self.food.len();

        for i in 0..num_ants {
            // fill ant vision of food and other ants
            let ant_vision = self.config.ants.vision_range;
            let mut close_by: Vec<Vision> = Vec::new();
            for j in 0..num_ants {
                if i != j {
                    let other_ant = self.ants[j].ant;

                    let distance = self.ants[i].ant.position.distance(other_ant.position);

                    if distance < ant_vision {
                        close_by.push(Vision {
                            object: SeenObject::Ant(other_ant),
                            distance,
                        });
                    }
                }
            }

            for j in 0..num_foods {
                let food_item = &self.food[j];

                let distance = self.ants[i]
                    .ant
                    .position
                    .distance(food_item.food.get_position());

                if distance < ant_vision {
                    close_by.push(Vision {
                        object: SeenObject::Food(food_item.food),
                        distance,
                    });
                }
            }

            let ant_action = ant_func(&self.ants[i].ant, &close_by);
            match ant_action {
                AntAction::Nothing => {}
                AntAction::GoForward(length) => self.ants[i].ant.go_forward(length),
                AntAction::RotateLeft(angle) => self.ants[i].ant.rotate_left(angle),
                AntAction::RotateRight(angle) => self.ants[i].ant.rotate_right(angle),
                AntAction::EatFood(food) => {
                    // Find the corresponding food on the ground, not the cloned proxy element
                    for orig_food_item in &mut self.food {
                        if orig_food_item.food == food {
                            self.ants[i].ant.eat_food(&mut orig_food_item.food);
                        }
                    }
                }
                AntAction::CarryFood(food) => {
                    // Find the corresponding food on the ground, not the cloned proxy element
                    for orig_food_item in &mut self.food {
                        if orig_food_item.food == food {
                            self.ants[i].ant.carry_food(&mut orig_food_item.food, &self.config.ants);
                        }
                    }
                }
            }

            self.ants[i].ant.rounds_to_energy_loss -= 1;
            if self.ants[i].ant.rounds_to_energy_loss <= 0 {
                self.ants[i].ant.energy = self.ants[i]
                    .ant
                    .energy
                    .saturating_sub(self.config.ants.energy_loss_amount); // Ants have to spend energy to be alive

                self.ants[i].ant.rounds_to_energy_loss = self.config.ants.energy_loss_rounds;
            }

            Ground::push_ant_into_boundary(&mut self.ants[i], self.size);
        }
    }

    fn cleanup_ground(&mut self, _dt: Duration) {
        self.ants.retain(|x| x.ant.is_alive());
        self.food.retain(|x| x.food.is_some_left());
        self.nests.retain(|x| x.nest.is_alive());
    }

    fn spawn_new_food(&mut self, _dt: Duration, display: &Display) {
        self.food_timer -= 1;
        if self.food_timer == 0 {
            self.food_timer = self.config.food.spawn_time;
            self.generate_random_food(1, display);
        }
    }

    pub fn update(
        &mut self,
        dt: Duration,
        display: &Display,
        ant_func: AntFunc,
        nest_func: NestFunc,
    ) {
        self.update_nests(dt, nest_func, display);
        self.update_ants(dt, ant_func);

        self.cleanup_ground(dt);

        self.spawn_new_food(dt, display);
    }

    pub fn ant_list(&self) -> &Vec<AntDrawable> {
        &self.ants
    }

    pub fn nest_list(&self) -> &Vec<NestDrawable> {
        &self.nests
    }
}

impl Ground {
    pub fn draw(&mut self, target: &mut Frame, cam: &Camera) {
        self.rect.draw(target, cam);

        for colony in &mut self.nests {
            colony.draw(target, cam);
        }

        for pellet in &mut self.food {
            pellet.draw(target, cam);
        }

        for ant in &mut self.ants {
            ant.draw(target, cam);
        }
    }
}

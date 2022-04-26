use crate::animals::ant::{Action, Ant, AntDrawable};
use crate::ant_impl::AntLogic;
use crate::helper::*;
use crate::{items::food::*, support::camera::Camera};

use rand::prelude::*;

use config::Config;
use glium::{Display, Frame};
use std::time::Duration;

pub struct Ground<F: AntLogic> {
    size: Vector2D,
    food: Vec<FoodPelletDrawable>,
    ants: Vec<AntDrawable>,
    food_timer: i32,

    pub config: Config,

    // technical
    next_food_id: usize,
    rng: ThreadRng,
    rect: crate::primitives::rectangle::Rectangle,
    ant_func: F,
}

impl<F> Ground<F>
where
    F: AntLogic,
{
    pub fn new_empty(size: Vector2D, ant_func: F, display: &Display) -> Ground<F> {
        let config = Config::new();

        Ground {
            food: Vec::new(),
            ants: Vec::new(),
            size,
            food_timer: config.food.spawn_time,
            next_food_id: 0,
            rng: rand::thread_rng(),
            rect: crate::primitives::rectangle::Rectangle::new(
                size,
                0.5 * size, //Vector2D::new(0.0, 0.0),
                Rotation::new_rad(0.0),
                GREEN,
                display,
            ),
            config,
            ant_func,
        }
    }
}

// Generate Stuff
impl<F> Ground<F>
where
    F: AntLogic,
{
    pub fn num_ants(&self) -> usize {
        self.ants.len()
    }

    pub fn num_foods(&self) -> usize {
        self.food.len()
    }

    pub fn start_new_round(&mut self, display: &Display) {
        self.ants.clear();
        self.food.clear();

        self.next_food_id = 0;

        self.generate_ants(self.config.ants.start_amount, display);
        self.generate_random_food(self.config.food.start_amount, display);
    }

    pub fn generate_random_food(&mut self, amount: i32, display: &Display) {
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

    pub fn generate_ants(&mut self, amount: i32, display: &Display) {
        for i in 0..amount {
            let x: f32 = self.rng.gen::<f32>() * self.size.x();
            let y: f32 = self.rng.gen::<f32>() * self.size.y();

            let ant = AntDrawable::new_at(i, &self.config.ants, Vector2D::new(x, y), display);
            self.ants.push(ant);
        }
    }
}

// Update functions
impl<F> Ground<F>
where
    F: AntLogic,
{
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

    fn update_ants(&mut self, dt: Duration) {
        let num_ants = self.ants.len();
        let num_foods = self.food.len();

        for i in 0..num_ants {
            // fill ant vision of food and other ants
            let ant_vision = self.config.ants.vision_range;
            let mut close_by: Vec<Vision> = Vec::new();
            for j in 0..num_ants {
                if i != j {
                    let other_ant = self.ants[j].ant.clone();

                    let distance = self.ants[i].ant.position.distance(other_ant.position);

                    if distance < ant_vision {
                        close_by.push(Vision::Ant(other_ant, distance));
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
                    close_by.push(Vision::Food(food_item.food.clone(), distance));
                }
            }

            let ant_action = self.ants[i].ant.update(&close_by, &mut self.ant_func, dt);
            match ant_action {
                Action::Nothing => {}
                Action::GoForward(length) => self.ants[i].ant.go_forward(length),
                Action::RotateLeft(angle) => self.ants[i].ant.rotate_left(angle),
                Action::RotateRight(angle) => self.ants[i].ant.rotate_right(angle),
                Action::EatFood(food) => {
                    // Find the corresponding food on the ground, not the cloned proxy element
                    for orig_food_item in &mut self.food {
                        if orig_food_item.food == food {
                            self.ants[i].ant.eat_food(&mut orig_food_item.food);
                        }
                    }
                }
            }
            self.ants[i].ant.energy -= self.config.ants.energy_loss; // Ants have to spend energy to be alive

            Ground::<F>::push_ant_into_boundary(&mut self.ants[i], self.size);
        }
    }

    fn cleanup_ground(&mut self, _dt: Duration) {
        self.ants.retain(|x| x.ant.is_alive());
        self.food.retain(|x| x.food.is_some_left());
    }

    fn spawn_new_food(&mut self, _dt: Duration, display: &Display) {
        self.food_timer -= 1;
        if self.food_timer == 0 {
            self.food_timer = self.config.food.spawn_time;
            self.generate_random_food(1, display);
        }
    }

    pub fn update(&mut self, dt: Duration, display: &Display) {
        self.update_ants(dt);

        self.cleanup_ground(dt);

        self.spawn_new_food(dt, display);
    }

    pub fn ant_list(&self) -> &Vec<AntDrawable> {
        &self.ants
    }
}

impl<F> Ground<F>
where
    F: AntLogic,
{
    pub fn draw(&mut self, target: &mut Frame, cam: &Camera) {
        // Todo
        self.rect.draw(target, cam);

        for pellet in &mut self.food {
            pellet.draw(target, cam);
        }

        for ant in &mut self.ants {
            ant.draw(target, cam);
        }
    }
}

use crate::animals::ant::Ant;
use crate::helper::*;
use crate::{items::food::*, support::camera::Camera, AntLogic};

use rand::prelude::*;

use config::Config;
use glium::{Display, Frame};
use std::cell::RefCell;
use std::{rc::Rc, time::Duration};

pub struct Ground<F: AntLogic> {
    size: Vector2D,
    food: Vec<Rc<RefCell<FoodPellet>>>,
    ants: Vec<Rc<RefCell<Ant>>>,
    food_timer: i32,

    pub config: Config,

    // technical
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

        self.generate_ants(self.config.ants.start_amount, display);
        self.generate_random_food(self.config.food.start_amount, display);
    }

    pub fn generate_random_food(&mut self, amount: i32, display: &Display) {
        for _i in 0..amount {
            let x: f32 = self.rng.gen::<f32>() * self.size.x();
            let y: f32 = self.rng.gen::<f32>() * self.size.y();

            let new_food = FoodPellet::new_at_pos(
                Vector2D::new(x, y),
                self.config.food.nutrition,
                display,
                self.config.food.eaten_value,
            );
            self.food.push(Rc::new(RefCell::new(new_food)));
        }
    }

    pub fn generate_ants(&mut self, amount: i32, display: &Display) {
        for i in 0..amount {
            let x: f32 = self.rng.gen::<f32>() * self.size.x();
            let y: f32 = self.rng.gen::<f32>() * self.size.y();

            let ant = Ant::new_at(i, &self.config.ants, Vector2D::new(x, y), display);
            self.ants.push(Rc::new(RefCell::new(ant)));
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

    fn push_ant_into_boundary(ant: &Rc<RefCell<Ant>>, size: Vector2D) {
        if ant.borrow_mut().position.x() < 0.0 {
            let old_pos = ant.borrow().position.y();
            ant.borrow_mut().position = Vector2D::new(0.0, old_pos);
        }

        if ant.borrow_mut().position.x() > size.x() {
            let old_pos = ant.borrow().position.y();
            ant.borrow_mut().position = Vector2D::new(size.x(), old_pos);
        }

        if ant.borrow_mut().position.y() < 0.0 {
            let old_pos = ant.borrow().position.x();
            ant.borrow_mut().position = Vector2D::new(old_pos, 0.0);
        }

        if ant.borrow_mut().position.y() > size.y() {
            let old_pos = ant.borrow().position.x();
            ant.borrow_mut().position = Vector2D::new(old_pos, size.y());
        }
    }

    fn update_ants(&mut self, dt: Duration) {
        let num_ants = self.ants.len();
        let num_foods = self.food.len();

        for i in 0..num_ants {
            let ant = &self.ants[i];

            // fill ant vision of food and other ants
            let ant_vision = self.config.ants.vision_range;
            let mut close_by: Vec<Vision> = Vec::new();
            for j in 0..num_ants {
                if i != j {
                    let other_ant = &self.ants[j];

                    let distance = ant.borrow().position.distance(other_ant.borrow().position);

                    if distance < ant_vision {
                        close_by.push(Vision::Ant(Rc::downgrade(other_ant), distance));
                    }
                }
            }

            for j in 0..num_foods {
                let food_item = &self.food[j];

                let distance = ant
                    .borrow()
                    .position
                    .distance(food_item.borrow().get_position());

                if distance < ant_vision {
                    close_by.push(Vision::Food(Rc::downgrade(food_item), distance));
                }
            }

            ant.borrow_mut().update(close_by, &mut self.ant_func, dt);
            ant.borrow_mut().energy -= self.config.ants.energy_loss; // Ants have to spend energy to be alive

            Ground::<F>::push_ant_into_boundary(&ant, self.size);
        }
    }

    fn cleanup_ground(&mut self, _dt: Duration) {
        self.ants.retain(|x| x.borrow().is_alive());
        self.food.retain(|x| x.borrow().is_some_left());
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

    pub fn ant_list(&self) -> &Vec<Rc<RefCell<Ant>>> {
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

        for pellet in &self.food {
            pellet.borrow_mut().draw(target, cam);
        }

        for ant in &self.ants {
            ant.borrow_mut().draw(target, cam);
        }
    }
}

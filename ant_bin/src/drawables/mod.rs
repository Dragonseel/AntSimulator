use common::{
    buildings::Nest,
    helper::{
        config::{AntConfig, NestConfig},
        Color, Rotation, Vector2D, BLUE,
    },
    items::food::FoodPellet,
};
use glium::{Display, Frame};

use crate::{primitives::rectangle::Rectangle, support::camera::Camera};

pub struct AntDrawable {
    pub ant: common::animals::ant::Ant,
    rect: Rectangle,
}

impl AntDrawable {
    pub fn new(id: usize, config: &AntConfig, display: &Display) -> AntDrawable {
        let size = Vector2D::new(16.0, 7.0);
        let position = Vector2D::new(50.0, 50.0);
        let rotation = Rotation::new_rad(0.0f32);
        let color = Color::new(1.0f32, 0.0f32, 0.0f32, 1.0f32);

        AntDrawable {
            ant: common::animals::ant::Ant {
                id,
                position,
                color,
                rotation,
                size,
                speed: config.speed,
                angular_speed: config.angular_speed,
                energy: config.max_energy,
                max_energy: config.max_energy,
                mouth_reach: config.mouth_reach,
                rounds_to_energy_loss: config.energy_loss_rounds,
            },
            rect: Rectangle::new(size, position, rotation, color, display),
        }
    }

    pub fn new_at_pos(
        id: usize,
        config: &AntConfig,
        pos: Vector2D,
        display: &Display,
    ) -> AntDrawable {
        let mut ant_drawable = AntDrawable::new(id, config, display);
        ant_drawable.ant.position = pos;
        ant_drawable
    }

    pub fn draw(&mut self, target: &mut Frame, cam: &Camera) {
        self.rect.position = self.ant.position; //- 0.5 * self.size; // * self.rotation);
        self.rect.rotation = self.ant.rotation;
        self.rect.draw(target, cam);
    }
}

pub struct FoodPelletDrawable {
    pub food: FoodPellet,
    rect: crate::primitives::rectangle::Rectangle,
}

impl FoodPelletDrawable {
    pub fn new_at_pos(
        id: usize,
        pos: Vector2D,
        nut: u32,
        display: &Display,
        bite_size: u32,
    ) -> FoodPelletDrawable {
        FoodPelletDrawable {
            food: FoodPellet {
                position: pos,
                nutrition: nut,
                bite_size,
                id,
            },
            rect: crate::primitives::rectangle::Rectangle::new(
                Vector2D::new(5.0, 5.0),
                pos,
                Rotation::new_rad(0.0),
                BLUE,
                display,
            ),
        }
    }

    pub fn draw(&mut self, target: &mut Frame, cam: &Camera) {
        self.rect.draw(target, cam);
    }
}

pub struct NestDrawable {
    pub nest: Nest,
    rect: crate::primitives::rectangle::Rectangle,
}

impl NestDrawable {
    pub fn new_at_pos(
        id: usize,
        pos: Vector2D,
        config: &NestConfig,
        display: &Display,
    ) -> NestDrawable {
        NestDrawable {
            nest: Nest {
                id,
                energy: config.start_energy,
                pos,
                rounds_to_energy_loss: config.energy_loss_rounds,
            },
            rect: crate::primitives::rectangle::Rectangle::new(
                Vector2D::new(50.0, 50.0),
                pos,
                Rotation::new_rad(0.0),
                common::helper::BLACK,
                display,
            ),
        }
    }

    pub fn draw(&mut self, target: &mut Frame, cam: &Camera) {
        self.rect.draw(target, cam);
    }
}

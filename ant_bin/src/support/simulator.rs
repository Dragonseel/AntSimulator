use crate::{
    ground,
    support::{self, camera::Camera},
    AntFunc, NestFunc,
};
use common::helper::Vector2D;
use glium::{Display, Frame, Surface};
use std::time::Duration;

pub struct Simulator {
    pub ground: ground::Ground,
    pub cam: Camera,
    pub new_round_pending: bool,

    // technical
    pub size: [f32; 2],
}

impl Simulator {
    pub fn new(display: &Display) -> Simulator {
        let mut ground = ground::Ground::new_empty(Vector2D::new(1000.0, 1000.0), display);
        ground.generate_colonies(display);
        ground.generate_random_food(10, display);

        Simulator {
            ground,
            cam: {
                let mut cam = support::camera::Camera::new();
                cam.position = [490.0, 470.0, -962.0];
                cam
            },
            new_round_pending: false,
            size: [1.0, 1.0],
        }
    }

    pub fn update(
        &mut self,
        dt: Duration,
        display: &Display,
        ant_func: AntFunc,
        nest_func: NestFunc,
    ) {
        if self.new_round_pending {
            self.ground.start_new_round(display);
            self.new_round_pending = false;
        }

        self.cam.update_view();
        self.ground.update(dt, display, ant_func, nest_func);
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        self.size = [
            frame.get_dimensions().0 as f32,
            frame.get_dimensions().1 as f32,
        ];
        self.cam.update_proj(frame);
        self.ground.draw(frame, &self.cam);
    }
}

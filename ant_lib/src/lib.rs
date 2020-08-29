#![allow(dead_code)]

use glium::{Display, Frame, Surface};
use helper::Size;
use imgui::{im_str, ChildWindow, Condition, Window};
use std::{cell::RefCell, rc::Rc, time::Duration};

mod animals;
mod ground;
mod helper;
mod items;
mod primitives;
mod support;

pub mod prelude {
    pub use crate::animals::ant::Action;
    pub use crate::animals::ant::Ant;
    pub use crate::helper::Direction;
    pub use crate::helper::Vision;
    pub use crate::items::food::FoodPellet;
    pub use crate::AntLogic;
}

use prelude::*;

pub trait AntLogic {
    fn update(&self, ant: &Ant, vision: &Vec<Vision>) -> Action;
}

struct Simulator<F>
where
    F: AntLogic,
{
    ground: ground::Ground<F>,
    cam: support::camera::Camera,
    new_round_pending: bool,

    // technical
    pub size: [f32; 2],
}

impl<F> Simulator<F>
where
    F: AntLogic,
{
    pub fn new(display: &Display, ant_func: F) -> Simulator<F>
    where
        F: AntLogic,
    {
        let mut ground = ground::Ground::new_empty(Size::new(1000.0, 1000.0), ant_func, display);
        ground.generate_ants(10, display);
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

    pub fn update(&mut self, dt: Duration, display: &Display) {
        if self.new_round_pending {
            self.ground.start_new_round(display);
            self.new_round_pending = false;
        }

        self.cam.update_view();
        self.ground.update(dt, display)
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

pub fn start_simulation<F>(ant_func: F)
where
    F: AntLogic + 'static,
{
    let system = support::init(file!());

    let app: Rc<RefCell<Simulator<F>>> =
        Rc::new(RefCell::new(Simulator::new(&system.display, ant_func)));

    let app_ui = Rc::clone(&app);
    let app_update = Rc::clone(&app);
    let app_draw = Rc::clone(&app);

    system.main_loop(
        move |_run, ui| {
            Window::new(im_str!("Camera Control"))
                .size([50.0, 300.0], Condition::FirstUseEver)
                .position([50.0, 50.0], Condition::FirstUseEver)
                .build(ui, || {
                    if ui.button(im_str!("forwards"), [30.0, 30.0]) {
                        app_ui.borrow_mut().cam.move_forwards();
                    }

                    if ui.button(im_str!("backwards"), [30.0, 30.0]) {
                        app_ui.borrow_mut().cam.move_backwards();
                    }

                    if ui.button(im_str!("up"), [30.0, 30.0]) {
                        app_ui.borrow_mut().cam.move_up();
                    }

                    if ui.button(im_str!("down"), [30.0, 30.0]) {
                        app_ui.borrow_mut().cam.move_down();
                    }

                    if ui.button(im_str!("left"), [30.0, 30.0]) {
                        app_ui.borrow_mut().cam.move_left();
                    }

                    if ui.button(im_str!("right"), [30.0, 30.0]) {
                        app_ui.borrow_mut().cam.move_right();
                    }
                });

            Window::new(im_str!("Simulation Control"))
                .size([300.0, 300.0], Condition::FirstUseEver)
                .position([50.0, 400.0], Condition::FirstUseEver)
                .build(ui, || {
                    if ui.button(im_str!("Start new sim"), [150.0, 50.0]) {
                        app_ui.borrow_mut().new_round_pending = true;
                    }

                    ui.separator();
                    ui.text_colored(helper::RED.get_data(), im_str!("Food"));

                    if ui
                        .drag_int(
                            im_str!("Food Time"),
                            &mut app_ui.borrow_mut().ground.config.food.spawn_time,
                        )
                        .build()
                    {
                        app_ui.borrow_mut().ground.reset_food_time();
                    }

                    ui.drag_int(
                        im_str!("Food Value"),
                        &mut app_ui.borrow_mut().ground.config.food.nutrition,
                    )
                    .build();

                    ui.drag_int(
                        im_str!("Food Bite Size"),
                        &mut app_ui.borrow_mut().ground.config.food.eaten_value,
                    )
                    .build();

                    ui.drag_int(
                        im_str!("Food Start Amount"),
                        &mut app_ui.borrow_mut().ground.config.food.start_amount,
                    )
                    .build();

                    ui.separator();
                    ui.text_colored(helper::RED.get_data(), im_str!("Ants"));

                    ui.drag_int(
                        im_str!("Ant Max Energy"),
                        &mut app_ui.borrow_mut().ground.config.ants.max_energy,
                    )
                    .build();

                    ui.drag_float(
                        im_str!("Ant Speed"),
                        &mut app_ui.borrow_mut().ground.config.ants.speed,
                    )
                    .build();

                    ui.drag_float(
                        im_str!("Ant Angular Speed"),
                        &mut app_ui.borrow_mut().ground.config.ants.angular_speed,
                    )
                    .build();

                    ui.drag_float(
                        im_str!("Ant Vision Range"),
                        &mut app_ui.borrow_mut().ground.config.ants.vision_range,
                    )
                    .build();

                    ui.drag_int(
                        im_str!("Ant Energy Loss"),
                        &mut app_ui.borrow_mut().ground.config.ants.energy_loss,
                    )
                    .build();

                    ui.drag_int(
                        im_str!("Ant Start Amount"),
                        &mut app_ui.borrow_mut().ground.config.ants.start_amount,
                    )
                    .build();
                });

            Window::new(im_str!("Statistics"))
                .size([300.0, 300.0], Condition::FirstUseEver)
                .position([app_ui.borrow().size[0] - 350.0, 50.0], Condition::Always)
                .build(ui, || {
                    ui.text(im_str!("Num Ants: {}", app_ui.borrow().ground.num_ants()));

                    ui.columns(2, im_str!("Ant View"), true);
                    ui.text(im_str!("Ant"));
                    ui.next_column();
                    ui.text(im_str!("Food"));
                    ui.next_column();
                    ui.columns(1, im_str!("Main"), false);

                    ChildWindow::new(im_str!("AntList"))
                        .size([250.0, 200.0])
                        .border(true)
                        .scroll_bar(true)
                        .build(ui, || {
                            ui.columns(2, im_str!("AntList_Inner"), true);

                            for ant in app_ui.borrow().ground.ant_list() {
                                ui.text(ant.borrow().id.to_string());
                                ui.next_column();
                                let text = ant.borrow().energy.to_string();
                                ui.text(text);
                                ui.next_column();
                            }

                            ui.columns(1, im_str!("AntList_Inner"), true);
                        });
                    ui.text(im_str!("Testing"));

                    ui.text(im_str!("Num Foods: {}", app_ui.borrow().ground.num_foods()));
                });
        },
        move |dt, display| {
            app_update.borrow_mut().update(dt, display);
        },
        move |target, _display| {
            app_draw.borrow_mut().draw(target);
        },
    );
}

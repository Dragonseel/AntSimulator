#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc};

mod animals;
mod ground;
mod helper;
mod items;
mod primitives;
mod support;

pub mod prelude {
    pub use crate::animals::ant::Action;
    pub use crate::animals::ant::Ant;
    pub use crate::helper::*;
    pub use crate::items::food::FoodPellet;
    pub use crate::AntLogic;
}

use prelude::*;

use support::simulator::Simulator;

pub trait AntLogic {
    fn update(&self, ant: &Ant, vision: &Vec<Vision>) -> Action;
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
            support::ui::camera_control(ui, &app_ui);

            support::ui::simulation_control(ui, &app_ui);

            support::ui::statistics(ui, &app_ui);
        },
        move |dt, display| {
            app_update.borrow_mut().update(dt, display);
        },
        move |target, _display| {
            app_draw.borrow_mut().draw(target);
        },
    );
}

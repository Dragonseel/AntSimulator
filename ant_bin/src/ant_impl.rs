#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc};

use crate::{
    animals::ant::{Action, Ant},
    helper::Vision,
    support::simulator::Simulator,
};

pub trait AntLogic {
    fn update(&self, ant: &Ant, vision: &[Vision]) -> Action;
}

pub fn start_simulation<F>(ant_func: F)
where
    F: AntLogic + 'static,
{
    let system = crate::support::init(file!());

    let app: Rc<RefCell<Simulator<F>>> =
        Rc::new(RefCell::new(Simulator::new(&system.display, ant_func)));

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

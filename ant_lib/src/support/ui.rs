use imgui::Drag;
use imgui::Ui;
use imgui::{im_str, ChildWindow, Condition, Window};
use std::{cell::RefCell, rc::Rc};

use crate::{helper, AntLogic, Simulator};

pub fn camera_control<F: AntLogic + 'static>(ui: &mut Ui, app_ui: &Rc<RefCell<Simulator<F>>>) {
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
}

pub fn simulation_control<F: AntLogic + 'static>(ui: &mut Ui, app_ui: &Rc<RefCell<Simulator<F>>>) {
    Window::new(im_str!("Simulation Control"))
        .size([300.0, 300.0], Condition::FirstUseEver)
        .position([50.0, 400.0], Condition::FirstUseEver)
        .build(ui, || {
            if ui.button(im_str!("Start new sim"), [150.0, 50.0]) {
                app_ui.borrow_mut().new_round_pending = true;
            }

            ui.separator();
            ui.text_colored(helper::RED.get_data(), im_str!("Food"));

            if Drag::new(im_str!("Food Time"))
                .build(ui, &mut app_ui.borrow_mut().ground.config.food.spawn_time)
            {
                app_ui.borrow_mut().ground.reset_food_time();
            }

            Drag::new(im_str!("Food Value"))
                .build(ui, &mut app_ui.borrow_mut().ground.config.food.nutrition);

            Drag::new(im_str!("Food Bite Size"))
                .build(ui, &mut app_ui.borrow_mut().ground.config.food.eaten_value);

            Drag::new(im_str!("Food Start Amount"))
                .build(ui, &mut app_ui.borrow_mut().ground.config.food.start_amount);

            ui.separator();
            ui.text_colored(helper::RED.get_data(), im_str!("Ants"));

            Drag::new(im_str!("Ant Max Energy"))
                .build(ui, &mut app_ui.borrow_mut().ground.config.ants.max_energy);

            Drag::new(im_str!("Ant Speed"))
                .build(ui, &mut app_ui.borrow_mut().ground.config.ants.speed);

            Drag::new(im_str!("Ant Angular Speed")).build(
                ui,
                &mut app_ui.borrow_mut().ground.config.ants.angular_speed,
            );

            Drag::new(im_str!("Ant Vision Range"))
                .build(ui, &mut app_ui.borrow_mut().ground.config.ants.vision_range);

            Drag::new(im_str!("Ant Energy Loss"))
                .build(ui, &mut app_ui.borrow_mut().ground.config.ants.energy_loss);

            Drag::new(im_str!("Ant Start Amount"))
                .build(ui, &mut app_ui.borrow_mut().ground.config.ants.start_amount);
        });
}

pub fn statistics<F: AntLogic + 'static>(ui: &mut Ui, app_ui: &Rc<RefCell<Simulator<F>>>) {
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
}

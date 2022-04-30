use imgui::Drag;
use imgui::Ui;
use imgui::{ChildWindow, Condition, Window};
use std::{cell::RefCell, rc::Rc};

use super::simulator::Simulator;
use common::helper;

pub fn camera_control(ui: &mut Ui, app_ui: &Rc<RefCell<Simulator>>) {
    Window::new("Camera Control")
        .size([50.0, 300.0], Condition::FirstUseEver)
        .position([50.0, 50.0], Condition::FirstUseEver)
        .build(ui, || {
            if ui.button_with_size("forwards", [30.0, 30.0]) {
                app_ui.borrow_mut().cam.move_forwards();
            }

            if ui.button_with_size("backwards", [30.0, 30.0]) {
                app_ui.borrow_mut().cam.move_backwards();
            }

            if ui.button_with_size("up", [30.0, 30.0]) {
                app_ui.borrow_mut().cam.move_up();
            }

            if ui.button_with_size("down", [30.0, 30.0]) {
                app_ui.borrow_mut().cam.move_down();
            }

            if ui.button_with_size("left", [30.0, 30.0]) {
                app_ui.borrow_mut().cam.move_left();
            }

            if ui.button_with_size("right", [30.0, 30.0]) {
                app_ui.borrow_mut().cam.move_right();
            }
        });
}

pub fn simulation_control(ui: &mut Ui, app_ui: &Rc<RefCell<Simulator>>) {
    Window::new("Simulation Control")
        .size([300.0, 300.0], Condition::FirstUseEver)
        .position([50.0, 400.0], Condition::FirstUseEver)
        .build(ui, || {
            if ui.button_with_size("Start new sim", [150.0, 50.0]) {
                app_ui.borrow_mut().new_round_pending = true;
            }

            ui.separator();
            ui.text_colored(helper::RED.get_data(), "Food");

            if Drag::new("Food Time")
                .build(ui, &mut app_ui.borrow_mut().ground.config.food.spawn_time)
            {
                app_ui.borrow_mut().ground.reset_food_time();
            }

            Drag::new("Food Value")
                .build(ui, &mut app_ui.borrow_mut().ground.config.food.nutrition);

            Drag::new("Food Bite Size")
                .build(ui, &mut app_ui.borrow_mut().ground.config.food.eaten_value);

            Drag::new("Food Start Amount")
                .build(ui, &mut app_ui.borrow_mut().ground.config.food.start_amount);

            ui.separator();
            ui.text_colored(helper::RED.get_data(), "Ants");

            Drag::new("Ant Max Energy")
                .build(ui, &mut app_ui.borrow_mut().ground.config.ants.max_energy);

            Drag::new("Ant Speed").build(ui, &mut app_ui.borrow_mut().ground.config.ants.speed);

            Drag::new("Ant Angular Speed").build(
                ui,
                &mut app_ui.borrow_mut().ground.config.ants.angular_speed,
            );

            Drag::new("Ant Vision Range").build(
                ui,
                &mut (app_ui.borrow_mut()).ground.config.ants.vision_range,
            );

            Drag::new("Ant Energy Loss Amount").build(
                ui,
                &mut app_ui.borrow_mut().ground.config.ants.energy_loss_amount,
            );

            Drag::new("Ant lose energy after rounds").build(
                ui,
                &mut app_ui.borrow_mut().ground.config.ants.energy_loss_rounds,
            );

            ui.separator();
            ui.text_colored(helper::RED.get_data(), "Nests");

            Drag::new("Max Energy")
                .build(ui, &mut app_ui.borrow_mut().ground.config.nests.max_energy);
        });
}

pub fn statistics(ui: &mut Ui, app_ui: &Rc<RefCell<Simulator>>) {
    Window::new("Statistics")
        .size([300.0, 300.0], Condition::FirstUseEver)
        .position([app_ui.borrow().size[0] - 350.0, 50.0], Condition::Always)
        .build(ui, || {
            ui.text(format!("Num Ants: {}", app_ui.borrow().ground.num_ants()));

            ui.columns(3, "Ant View", true);
            ui.text("Ant");
            ui.next_column();
            ui.text("Food");
            ui.next_column();
            ui.text("Carrying");
            ui.next_column();
            ui.columns(1, "Main", false);

            ChildWindow::new("AntList")
                .size([250.0, 200.0])
                .border(true)
                .scroll_bar(true)
                .build(ui, || {
                    ui.columns(3, "AntList_Inner", true);

                    for ant_drawable in app_ui.borrow().ground.ant_list() {
                        ui.text(ant_drawable.ant.id.to_string());
                        ui.next_column();
                        ui.text(ant_drawable.ant.energy.to_string());
                        ui.next_column();
                        ui.text(ant_drawable.ant.carrying.to_string());
                        ui.next_column();
                    }

                    ui.columns(1, "AntList_Inner", true);
                });

            ui.text(format!("Num Foods: {}", app_ui.borrow().ground.num_foods()));

            ui.columns(2, "Nest View", true);
            ui.text("Nest");
            ui.next_column();
            ui.text("Energy");
            ui.next_column();
            ui.columns(1, "Main", false);

            ChildWindow::new("NestList")
                .size([250.0, 250.0])
                .border(true)
                .scroll_bar(true)
                .build(ui, || {
                    ui.columns(2, "NestList_Inner", true);

                    for nest_drawable in app_ui.borrow().ground.nest_list() {
                        ui.text(nest_drawable.nest.id.to_string());
                        ui.next_column();
                        ui.text(nest_drawable.nest.energy.to_string());
                        ui.next_column();
                    }

                    ui.columns(1, "AntList_Inner", true);
                });
        });
}

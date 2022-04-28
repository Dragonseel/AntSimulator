use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use common::animals::ant::{Ant, AntAction};
use common::buildings::{Nest, NestAction};
use common::helper::Vision;

use dynamic_reload::{DynamicReload, Symbol};
use plugins::Plugins;
use support::simulator::Simulator;

mod drawables;
mod ground;
mod plugins;
mod primitives;
mod support;

pub type AntFunc<'a> = Symbol<'a, extern "C" fn(&Ant, &Vec<Vision>) -> AntAction>;
pub type NestFunc<'a> = Symbol<'a, extern "C" fn(&Nest) -> NestAction>;

fn main() {
    // let logic = Logic {};

    let mut plugs = plugins::Plugins {
        plugins: Vec::new(),
    };

    // Setup the reload handler. A temporary directory will be created inside the target/debug
    // where plugins will be loaded from. That is because on some OS:es loading a shared lib
    // will lock the file so we can't overwrite it so this works around that issue.
    let mut reload_handler = DynamicReload::new(
        Some(vec!["target/debug"]),
        Some("target/debug"),
        dynamic_reload::Search::Default,
        Duration::from_secs(2),
    );

    unsafe {
        // test_shared is generated in build.rs
        match reload_handler.add_library("dynlib", dynamic_reload::PlatformName::Yes) {
            Ok(lib) => plugs.add_plugin(&lib),
            Err(e) => {
                println!("Unable to load dynamic lib, err {:?}", e);
                return;
            }
        }
    }

    //
    // While this is running (printing a number) change return value in file src/test_shared.rs
    // build the project with cargo build and notice that this code will now return the new value
    //

    let system = crate::support::init(file!());

    let app = Rc::new(RefCell::new(Simulator::new(&system.display)));

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
            unsafe {
                reload_handler.update(&Plugins::reload_callback, &mut plugs);
            }

            if plugs.plugins.len() > 0 {
                let update_fun: AntFunc =
                    unsafe { plugs.plugins[0].lib.get(b"ant_update\0").unwrap() };

                let nest_fun: NestFunc =
                    unsafe { plugs.plugins[0].lib.get(b"nest_update\0").unwrap() };

                app_update
                    .borrow_mut()
                    .update(dt, display, update_fun, nest_fun);
            }
        },
        move |target, _display| {
            app_draw.borrow_mut().draw(target);
        },
    );
}

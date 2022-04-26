use std::sync::Arc;

use dynamic_reload::{Lib, UpdateState};

pub struct Plugins {
    pub plugins: Vec<Arc<Lib>>,
}

impl Plugins {
    pub fn add_plugin(&mut self, plugin: &Arc<Lib>) {
        self.plugins.push(plugin.clone());
    }

    pub fn unload_plugins(&mut self, lib: &Arc<Lib>) {
        for i in (0..self.plugins.len()).rev() {
            if &self.plugins[i] == lib {
                self.plugins.swap_remove(i);
            }
        }
    }

    pub fn reload_plugin(&mut self, lib: &Arc<Lib>) {
        Self::add_plugin(self, lib);
    }

    // called when a lib needs to be reloaded.
    pub fn reload_callback(&mut self, state: UpdateState, lib: Option<&Arc<Lib>>) {
        match state {
            UpdateState::Before => Self::unload_plugins(self, lib.unwrap()),
            UpdateState::After => Self::reload_plugin(self, lib.unwrap()),
            UpdateState::ReloadFailed(_) => println!("Failed to reload"),
        }
    }
}

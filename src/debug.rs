// Debug helpers for bevy
#[cfg(debug_assertions)]
mod only_in_debug
{

use bevy::{
    prelude::*,
    ecs::schedule::ScheduleLabel
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

// Use egui for an inspector plugin
pub fn inspector() -> WorldInspectorPlugin {
    WorldInspectorPlugin::new()
}

// Save the scheduling graphs for system stages
pub fn save_schedule(app : &mut App, stages : &[&'static str]) {
    use bevy_mod_debugdump::*;

    if !std::path::Path::new("graphs").exists() {
        std::fs::create_dir("graphs").unwrap();
    }

    for &name in stages {
        let graph = schedule_graph_dot(app, name_to_stage(name), &schedule_graph::Settings::default());
        std::fs::write(format!("graphs/{}-schedule.dot", name.to_lowercase()), graph).unwrap();
    }
}

fn name_to_stage(name : &'static str) -> &dyn ScheduleLabel {
    match name {
        "PreStartup" => &PreStartup,
        "Startup" => &Startup,
        "PostStartup" => &PostStartup,
        "PreUpdate" => &PreUpdate,
        "Update" => &Update,
        "PostUpdate" => &PostUpdate,
        "FixedUpdate" => &FixedUpdate,
        _ => panic!("Unknown stage name: {}", name)
    }
}

}

#[cfg(debug_assertions)]
pub use only_in_debug::*;
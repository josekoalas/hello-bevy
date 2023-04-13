use bevy::{
    log::LogPlugin,
    prelude::*,
    window::{PresentMode, WindowResolution},
};
use hello_bevy::{save_schedule, GamePlugin, DEBUG};

fn main() {
    let mut game = App::new();
    // Plugins
    game.insert_resource(ClearColor(Color::rgb(0.6, 0.7, 1.0)))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Title".to_string(),
                        resolution: WindowResolution::new(800., 800.),
                        present_mode: PresentMode::AutoVsync,
                        resizable: false, // or use fit_canvas_to_parent: true for resizing on the web
                        canvas: Some("#bevy".to_string()),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    watch_for_changes: DEBUG, // hot realoading
                    ..default()
                })
                .set(LogPlugin {
                    // todo: enable when #8374 is merged
                    // layer: Box::new(|| Box::new(bevy::log::tracing_subscriber::fmt::Layer::default())),
                    ..default()
                }),
        )
        .add_plugin(GamePlugin);

    // Systems
    game.add_systems(Startup, init).add_systems(Update, update);

    // Get debug schedule graphs
    #[cfg(debug_assertions)]
    save_schedule(&mut game, &["Startup", "Update"]);

    // Run
    game.run();
}

// Components
// ...

// Resources
// ...

// Startup systems

fn init(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}

// Update systems

fn update() {}

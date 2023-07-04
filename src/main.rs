use bevy::prelude::PluginGroup;
use bevy::window::{Window, WindowResolution};
use bevy::DefaultPlugins;
use bevy::{prelude::App, window::WindowPlugin};

mod backend;
mod frontend;
mod constants;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(1200.0, 800.0),
            title: format!("Bevy AI Soccer {}", env!("CARGO_PKG_VERSION")).to_string(),
            ..Default::default()
        }),
        ..Default::default()
    }));
    
    app.add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin::new());

    app.add_plugin(backend::SoccerBackendPlugin::default());
    app.add_plugin(frontend::SoccerFrontendPlugin::default());
    app.run();
}

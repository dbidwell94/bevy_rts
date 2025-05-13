#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

mod core;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy RTS".into(),
                    ..default()
                }),
                ..default()
            }),
            EguiPlugin {
                enable_multipass_for_primary_context: true,
            },
            WorldInspectorPlugin::new(),
        ))
        .add_plugins(core::Plugin)
        .run()
}

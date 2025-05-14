#![allow(clippy::type_complexity)]

#[cfg(feature = "dev")]
use bevy::ecs::error::{GLOBAL_ERROR_HANDLER, error};
use bevy::prelude::*;
#[cfg(feature = "dev")]
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

mod core;

fn main() -> AppExit {
    #[cfg(feature = "dev")]
    GLOBAL_ERROR_HANDLER.set(error).unwrap();

    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy RTS".into(),
                    ..default()
                }),
                ..default()
            }),
            #[cfg(feature = "dev")]
            EguiPlugin {
                enable_multipass_for_primary_context: true,
            },
            #[cfg(feature = "dev")]
            WorldInspectorPlugin::new(),
        ))
        .add_plugins(core::Plugin)
        .run()
}

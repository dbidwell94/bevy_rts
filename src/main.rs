#![allow(clippy::type_complexity)]

#[cfg(feature = "dev")]
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
#[cfg(feature = "dev")]
use bevy::ecs::error::{GLOBAL_ERROR_HANDLER, error};
#[cfg(feature = "dev")]
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::{WgpuFeatures, WgpuSettings};
#[cfg(feature = "dev")]
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

mod core;

fn main() -> AppExit {
    #[cfg(feature = "dev")]
    GLOBAL_ERROR_HANDLER.set(error).unwrap();

    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins
            .set(RenderPlugin {
                render_creation: bevy::render::settings::RenderCreation::Automatic(WgpuSettings {
                    features: WgpuFeatures::POLYGON_MODE_LINE,
                    ..default()
                }),
                ..default()
            })
            .set(WindowPlugin {
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
        #[cfg(feature = "dev")]
        FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    font_size: 20.,
                    ..default()
                },
                ..default()
            },
        },
        #[cfg(feature = "dev")]
        WireframePlugin::default(),
    ))
    .add_plugins(core::Plugin);

    #[cfg(feature = "dev")]
    app.insert_resource(WireframeConfig::default());

    app.run()
}

pub mod terrain;

use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_butler::*;

use super::camera::{Moon, Sun};

#[derive(Component)]
pub struct Terrain;

#[butler_plugin]
#[add_plugin(to_plugin = super::Plugin)]
pub struct Plugin;

#[derive(Resource, Deref)]
#[insert_resource(plugin = Plugin)]
pub struct Seed(u32);

impl Default for Seed {
    fn default() -> Self {
        Self(rand::random())
    }
}

#[add_system(plugin = Plugin, schedule = Update)]
fn day_night_cycle(
    mut sun_query: Query<&mut Transform, (With<DirectionalLight>, With<Sun>, Without<Moon>)>,
    mut moon_query: Query<&mut Transform, (With<DirectionalLight>, With<Moon>, Without<Sun>)>,
    time: Res<Time>,
) -> Result {
    let mut sun_transform = sun_query.single_mut()?;
    let mut moon_transform = moon_query.single_mut()?;

    sun_transform.rotate_x(-time.delta_secs() * PI / 100.);
    moon_transform.rotation = sun_transform.rotation.inverse();

    Ok(())
}

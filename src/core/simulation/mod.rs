pub mod terrain;

use bevy::prelude::*;
use bevy_butler::*;

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

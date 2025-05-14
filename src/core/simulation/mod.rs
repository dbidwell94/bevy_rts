mod terrain;

use bevy::prelude::*;
use bevy_butler::*;

#[derive(Component)]
pub struct Terrain;

#[butler_plugin]
#[add_plugin(to_plugin = super::Plugin)]
pub struct Plugin;

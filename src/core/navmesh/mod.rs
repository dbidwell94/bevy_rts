use bevy::prelude::*;
use bevy_butler::*;

#[butler_plugin]
#[add_plugin(to_plugin = super::Plugin)]
struct Plugin;

#[insert_resource(plugin = Plugin)]
#[derive(Resource, Default)]
pub struct Navmesh();

#[derive(PartialEq, Eq, Clone)]
pub struct Navtile(u8);

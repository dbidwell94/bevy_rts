use bevy::prelude::*;
use bevy_butler::*;
use std::collections::HashMap;

#[butler_plugin]
#[add_plugin(to_plugin = super::Plugin)]
struct Plugin;

#[insert_resource(plugin = Plugin)]
#[derive(Resource, Default)]
pub struct Navmesh(HashMap<u32, HashMap<u32, Navtile>>);

impl Navmesh {
    #[inline]
    pub fn get_tile_at(&self, x: u32, y: u32) -> Option<&Navtile> {
        self.0.get(&x).and_then(|map| map.get(&y))
    }

    #[inline]
    pub fn get_tile_at_mut(&mut self, x: u32, y: u32) -> Option<&mut Navtile> {
        self.0.get_mut(&x).and_then(|map| map.get_mut(&y))
    }

    /// Sets the value at [x, y] to the given Navtile. Overwrites the existing tile if exists
    #[inline]
    pub fn set_at(&mut self, x: u32, y: u32, tile: Navtile) {
        self.0.entry(x).or_default().insert(y, tile);
    }
}

#[derive(PartialEq, Clone)]
pub struct Navtile(u8, f32);

impl Navtile {
    #[inline]
    pub fn height(&self) -> f32 {
        self.1
    }
}

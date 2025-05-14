mod camera;
pub(crate) mod input;
mod navmesh;
mod simulation;

use bevy::prelude::{Plugin as BevyPlugin, *};
use bevy_butler::*;

#[derive(States, Clone, Eq, PartialEq, Hash, Debug)]
enum GameState {
    Spawning,
    Simulating,
}

pub struct Plugin;

#[butler_plugin]
impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameState::Spawning);
    }
}

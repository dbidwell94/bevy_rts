use bevy::prelude::*;
use bevy_butler::*;

#[butler_plugin]
#[add_plugin(to_plugin = super::Plugin)]
pub struct Plugin;

#[add_system(plugin = Plugin, schedule = Startup)]
fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // build the ground mesh plane
    let plane_mesh = meshes.add(Plane3d::default().mesh().size(100., 100.).subdivisions(50));

    commands.spawn((
        Mesh3d(plane_mesh),
        MeshMaterial3d(materials.add(Color::linear_rgb(0.1, 0.1, 0.1))),
    ));
}

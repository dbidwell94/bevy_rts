use bevy::color::palettes::tailwind::*;
#[cfg(feature = "dev")]
use bevy::pbr::wireframe::Wireframe;
use bevy::{prelude::*, render::mesh::VertexAttributeValues};
use bevy_butler::*;
use noise::{MultiFractal, NoiseFn, Perlin};

const TERRAIN_HEIGHT: f32 = 25.;

#[derive(Component)]
pub struct Terrain;

#[butler_plugin]
#[add_plugin(to_plugin = super::Plugin)]
pub struct Plugin;

#[add_system(plugin = Plugin, schedule = Startup)]
fn setup_ground_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let noise = noise::BasicMulti::<Perlin>::default().set_octaves(500);
    let mut terrain = Mesh::from(
        Plane3d::default()
            .mesh()
            .size(1000., 1000.)
            .subdivisions(750),
    );

    if let Some(VertexAttributeValues::Float32x3(positions)) =
        terrain.attribute_mut(Mesh::ATTRIBUTE_POSITION)
    {
        for pos in positions.iter_mut() {
            pos[1] =
                noise.get([pos[0] as f64 / 200., pos[2] as f64 / 200.]) as f32 * TERRAIN_HEIGHT;
        }

        let colors: Vec<[f32; 4]> = positions
            .iter()
            .map(|[_, height, _]| {
                let g = *height / TERRAIN_HEIGHT * 2.;

                match g {
                    g if g > 0.5 => Color::WHITE,
                    g if g > -0.15 => Color::from(AMBER_600),
                    g if g > -0.85 => Color::BLACK,
                    _ => Color::from(BLUE_800),
                }
                .to_linear()
                .to_f32_array()
            })
            .collect();

        terrain.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    }

    terrain.compute_normals();

    // build the ground mesh plane
    let plane_mesh = meshes.add(terrain);

    commands.spawn((
        Mesh3d(plane_mesh),
        MeshMaterial3d(materials.add(Color::linear_rgb(0.1, 0.1, 0.1))),
        //#[cfg(feature = "dev")]
        //Wireframe,
        Terrain,
    ));
}

use super::Seed;
use avian3d::prelude::*;
use bevy::color::palettes::tailwind::*;
use bevy::{prelude::*, render::mesh::VertexAttributeValues};
use bevy_butler::*;
use leafwing_input_manager::prelude::*;
use noise::{MultiFractal, NoiseFn, Perlin};

pub const TERRAIN_HEIGHT: f32 = 100.;
const TERRAIN_SIZE: u32 = 2000;
const TERRAIN_DENSITY: u32 = 500;

#[derive(Component)]
pub struct Terrain;

#[butler_plugin]
#[add_plugin(to_plugin = super::Plugin)]
pub struct Plugin;

#[add_system(plugin = Plugin, schedule = Startup)]
pub fn setup_ground_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    seed: Res<Seed>,
) -> Result {
    let noise = noise::BasicMulti::<Perlin>::new(**seed).set_octaves(100);
    let mut terrain = Mesh::from(
        Plane3d::default()
            .mesh()
            .size(TERRAIN_SIZE as f32, TERRAIN_SIZE as f32)
            .subdivisions(TERRAIN_DENSITY),
    );

    if let Some(VertexAttributeValues::Float32x3(positions)) =
        terrain.attribute_mut(Mesh::ATTRIBUTE_POSITION)
    {
        info!(
            "Mesh created: scaler: {}, total_vertexes: {}",
            positions.len() / TERRAIN_DENSITY as usize * 4,
            positions.len()
        );
        let mut colors: Vec<[f32; 4]> = Vec::new();
        for pos in positions.iter_mut() {
            pos[1] =
                noise.get([pos[0] as f64 / 1000., pos[2] as f64 / 1000.]) as f32 * TERRAIN_HEIGHT;

            let g = pos[1] / TERRAIN_HEIGHT * 2.;
            colors.push(
                match g {
                    g if g > 0.5 => Color::WHITE,
                    g if g > -0.15 => Color::from(STONE_800),
                    g if g >= -0.85 => Color::BLACK,
                    _ => Color::from(BLUE_600),
                }
                .to_linear()
                .to_f32_array(),
            );
        }

        terrain.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    }

    terrain.compute_normals();

    // build the ground mesh plane
    let plane_mesh = meshes.add(terrain);

    commands.spawn((
        ColliderConstructor::TrimeshFromMeshWithConfig(TrimeshFlags::empty()),
        Mesh3d(plane_mesh),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Terrain,
        DebugRender::none(),
        Name::new("Terrain Mesh"),
    ));
    Ok(())
}

#[cfg(feature = "dev")]
mod dev {
    use super::*;

    #[add_system(plugin = super::Plugin, schedule = Update)]
    fn debug_toggle_wireframe(
        mut commands: Commands,
        dev_input_query: Query<&ActionState<crate::core::input::DebugInput>>,
        has_wireframe_query: Query<
            (Entity, Option<&bevy::pbr::wireframe::Wireframe>),
            With<Terrain>,
        >,
    ) -> Result {
        use crate::core::input::DebugInput;
        use bevy::pbr::wireframe::Wireframe;

        let (terrain_entity, wireframe_opt) = has_wireframe_query.single()?;
        let debug_controls = dev_input_query.single()?;
        let has_wireframe = wireframe_opt.is_some();

        if debug_controls.just_pressed(&DebugInput::ToggleWireframe) {
            match has_wireframe {
                true => commands.entity(terrain_entity).remove::<Wireframe>(),
                false => commands.entity(terrain_entity).insert(Wireframe),
            };
        }

        Ok(())
    }
}

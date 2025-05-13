mod input;
mod simulation;

use bevy::prelude::*;
use bevy_butler::*;
use input::CameraAction;
use leafwing_input_manager::prelude::*;

#[butler_plugin]
pub struct Plugin;

#[derive(Component)]
#[require(Transform)]
struct CameraTarget;

#[derive(Component)]
#[require(Transform)]
struct GroundTarget;

#[add_system(schedule = Startup, plugin = Plugin)]
fn init_environment(mut commands: Commands) {
    let input_map = InputMap::default()
        .with_dual_axis(CameraAction::Pan, VirtualDPad::wasd())
        .with_axis(CameraAction::Zoom, MouseScrollAxis::Y)
        .with_axis(
            CameraAction::Rotate,
            VirtualAxis::new(KeyCode::KeyQ, KeyCode::KeyE),
        );

    let camera_transform =
        Transform::from_xyz(0., 50., 50.).looking_at(Vec3::new(0., 0., 0.), Dir3::Y);

    commands.spawn((
        GroundTarget,
        children![(CameraTarget, camera_transform, input_map)],
    ));

    commands.spawn((Camera3d::default(), camera_transform));

    commands.spawn((
        DirectionalLight {
            illuminance: 1500.0,
            ..default()
        },
        Transform::from_xyz(0., 150., 0.).looking_at(Vec3::new(0., 0., 0.), Dir3::Y),
    ));
}

#[add_system(plugin = Plugin, schedule = Update)]
fn move_camera(
    mut camera_target_query: Query<
        (&mut Transform, &ActionState<CameraAction>),
        (With<CameraTarget>, Without<Camera3d>),
    >,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<CameraTarget>)>,
    time: Res<Time>,
) -> Result {
    let (mut target_transform, camera_action) = camera_target_query.single_mut()?;
    let mut camera_transform = camera_query.single_mut()?;

    let move_axis = camera_action
        .clamped_axis_pair(&CameraAction::Pan)
        .normalize_or_zero();
    let rotation_axis = camera_action.clamped_value(&CameraAction::Rotate);
    // TODO! use this zoom axis to update the distance from camera to world ground
    let _zoom_axis = camera_action.clamped_value(&CameraAction::Zoom);

    target_transform.translation +=
        Vec3::new(move_axis.x, 0.0, -move_axis.y) * time.delta_secs() * 100.;

    camera_transform.translation = camera_transform
        .translation
        .lerp(target_transform.translation, 0.0625);

    camera_transform.rotation = camera_transform
        .rotation
        .lerp(target_transform.rotation, 0.0625);

    Ok(())
}

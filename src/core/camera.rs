//! Contains the core logic for camera systems, camera input, and lighting

use super::{
    input::CameraAction,
    simulation::terrain::{TERRAIN_HEIGHT, Terrain},
};
use avian3d::prelude::*;
use bevy::color::palettes::tailwind::*;
use bevy::{
    core_pipeline::{
        auto_exposure::{AutoExposure, AutoExposureCompensationCurve, AutoExposurePlugin},
        bloom::Bloom,
        motion_blur::MotionBlur,
        smaa::Smaa,
        tonemapping::Tonemapping,
    },
    math::cubic_splines::LinearSpline,
    pbr::Atmosphere,
    prelude::{Plugin as BevyPlugin, *},
    render::{
        camera::{Exposure, PhysicalCameraParameters},
        primitives::Aabb,
        view::{ColorGrading, ColorGradingGlobal, ColorGradingSection},
    },
};
use bevy_butler::*;
use leafwing_input_manager::prelude::*;

const ZOOM_MIN: f32 = 5.;
const ZOOM_MAX: f32 = 1000.;
const CAMERA_PAN_SPEED: f32 = 1000.;

#[add_plugin(to_plugin = super::Plugin)]
pub struct Plugin;

#[butler_plugin]
impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AutoExposurePlugin);
    }
}

#[derive(Component)]
#[require(Transform)]
struct CameraTarget;

#[derive(Component)]
#[require(Transform)]
struct GroundTarget;

#[derive(Component)]
#[require(Transform)]
struct GroundCaster;

#[derive(Component)]
pub struct Sun;

#[derive(Component)]
pub struct Moon;

#[add_system(schedule = Startup, plugin = Plugin)]
fn init_camera(
    mut commands: Commands,
    mut compensation_curve: ResMut<Assets<AutoExposureCompensationCurve>>,
) -> Result {
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

    commands.spawn((RayCaster::new(Vec3::ZERO, -Dir3::Y), GroundCaster));

    commands.spawn((
        Camera3d::default(),
        Atmosphere::EARTH,
        // MotionBlur::default(),
        Msaa::Off,
        ColorGrading {
            highlights: ColorGradingSection { ..default() },
            shadows: ColorGradingSection {
                contrast: 1.00625,
                ..default()
            },
            global: ColorGradingGlobal {
                exposure: -0.5,
                ..default()
            },
            ..default()
        },
        Smaa::default(),
        AutoExposure {
            speed_darken: 1.0,
            speed_brighten: 0.8,
            range: -20.0..=180.0,
            filter: 0.011..=0.99,
            compensation_curve: compensation_curve.add(AutoExposureCompensationCurve::from_curve(
                LinearSpline::new([vec2(-4.0, -2.0), vec2(0., 0.), vec2(2.0, 0.), vec2(4.0, 2.)]),
            )?),
            ..default()
        },
        Tonemapping::AcesFitted,
        Camera {
            hdr: true,
            ..default()
        },
        camera_transform,
    ));

    // sunlight
    let sun_transform = Transform::from_xyz(0., 150., 0.);
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::RAW_SUNLIGHT,
            color: Color::from(YELLOW_100),
            shadows_enabled: true,
            ..default()
        },
        sun_transform,
        Sun,
    ));

    // moonlight

    let mut moon_transform = sun_transform;
    moon_transform.rotation = sun_transform.rotation.inverse();
    commands.spawn((
        DirectionalLight {
            illuminance: 10_000.0,
            color: Color::from(SLATE_300),
            shadows_enabled: true,
            ..default()
        },
        moon_transform,
        Moon,
    ));
    Ok(())
}

#[add_system(plugin = Plugin, schedule = Update)]
fn handle_camera_pan(
    action_query: Query<&ActionState<CameraAction>>,
    mut ground_target_query: Query<
        &mut Transform,
        (
            Without<CameraTarget>,
            Without<GroundCaster>,
            With<GroundTarget>,
        ),
    >,
    camera_target_query: Query<
        &Transform,
        (
            With<CameraTarget>,
            Without<GroundTarget>,
            Without<GroundCaster>,
        ),
    >,
    mut ground_raycast: Query<
        (&RayCaster, &RayHits, &mut Transform),
        (
            With<GroundCaster>,
            Without<CameraTarget>,
            Without<GroundTarget>,
        ),
    >,
    terrain_query: Query<Option<&Aabb>, With<Terrain>>,
    time: Res<Time>,
) -> Result {
    let (caster, hits, mut caster_transform) = ground_raycast.single_mut()?;
    let camera_action = action_query.single()?;
    let mut ground_transform = ground_target_query.single_mut()?;
    let camera_target_transform = camera_target_query.single()?;
    let terrain_extents = terrain_query.single()?;

    caster_transform.translation = Vec3::new(
        ground_transform.translation.x,
        TERRAIN_HEIGHT * 2.,
        ground_transform.translation.z,
    );

    let distance_scaler = (Vec3::ZERO - camera_target_transform.translation).length();

    let pan_axis = camera_action.axis_pair(&CameraAction::Pan);

    let move_direction = ground_transform
        .rotation
        .mul_vec3(Vec3::new(pan_axis.x, 0., -pan_axis.y));

    ground_transform.translation +=
        move_direction * time.delta_secs() * CAMERA_PAN_SPEED * (distance_scaler / ZOOM_MAX);

    if let Some(hit) = hits.iter().next() {
        let y = (caster.global_origin() + caster.global_direction() * hit.distance).y;
        ground_transform.translation.y = y;
    }

    if let Some(extents) = terrain_extents {
        ground_transform.translation = ground_transform
            .translation
            .clamp(extents.min().into(), extents.max().into());
    }

    Ok(())
}

#[add_system(plugin = Plugin, schedule = Update, after = handle_camera_pan)]
fn handle_camera_zoom(
    mut camera_target_query: Query<
        (&mut Transform, &ActionState<CameraAction>),
        With<CameraTarget>,
    >,
    time: Res<Time>,
) -> Result {
    let (mut target_transform, camera_action) = camera_target_query.single_mut()?;

    let zoom_axis = camera_action.clamped_value(&CameraAction::Zoom);

    // Ground transform will always be (0, 0, 0) relative to the CameraTarget
    let direction_vec = Vec3::ZERO - target_transform.translation;
    let distance = direction_vec.length();
    let direction = direction_vec.normalize_or_zero();

    let min_zoom_vec3 = Vec3::ZERO - (direction * ZOOM_MIN);
    let max_zoom_vec3 = Vec3::ZERO - (direction * ZOOM_MAX);

    let new_trans = (target_transform.translation
        + (direction
            * time.delta_secs()
            * zoom_axis
            * CAMERA_PAN_SPEED
            * 50.
            * (distance / ZOOM_MAX)))
        .clamp(min_zoom_vec3, max_zoom_vec3);

    target_transform.translation = new_trans;

    Ok(())
}

#[add_system(plugin = Plugin, schedule = Update, after = handle_camera_zoom)]
fn handle_camera_rotate(
    mut ground_target_query: Query<&mut Transform, (Without<CameraTarget>, With<GroundTarget>)>,
    camera_action_query: Query<&ActionState<CameraAction>>,
    time: Res<Time>,
) -> Result {
    let mut ground_target_transform = ground_target_query.single_mut()?;
    let camera_action = camera_action_query.single()?;

    let camera_rotation = camera_action.value(&CameraAction::Rotate);

    ground_target_transform
        .rotate_y(Rot2::degrees(camera_rotation * time.delta_secs() * 250.).as_radians());

    Ok(())
}

#[add_system(plugin = Plugin, schedule = Update, after = handle_camera_rotate)]
fn lerp_camera_to_target(
    mut camera_transform: Query<&mut Transform, (With<Camera3d>, Without<CameraTarget>)>,
    target_transform: Query<&GlobalTransform, (With<CameraTarget>, Without<Camera3d>)>,
    time: Res<Time>,
) -> Result {
    let target_transform = target_transform.single()?;
    let mut camera_transform = camera_transform.single_mut()?;

    camera_transform.translation = camera_transform
        .translation
        .lerp(target_transform.translation(), 10. * time.delta_secs());

    camera_transform.rotation = camera_transform
        .rotation
        .lerp(target_transform.rotation(), 10. * time.delta_secs());

    Ok(())
}

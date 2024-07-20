use crate::{OgleMode, OgleRig, OgleSettings, OgleTarget};
use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    math::vec2,
    prelude::*,
    render::camera::CameraProjection,
    window::PrimaryWindow,
};
use dolly::prelude::*;

#[derive(Default)]
pub struct OglePlugin {
    pub initial_settings: OgleSettings,
}

impl Plugin for OglePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<OgleRig>()
            .init_resource::<OgleTarget>()
            .init_state::<OgleMode>()
            .insert_resource(self.initial_settings)
            .add_plugins(bevy_pancam::PanCamPlugin)
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                ((follow_target, keep_within_settings).chain())
                    .run_if(in_state(OgleMode::Following)),
            )
            .add_systems(OnEnter(OgleMode::Frozen), on_enter_frozen)
            .add_systems(OnExit(OgleMode::Frozen), on_exit_frozen)
            .add_systems(OnEnter(OgleMode::Pancam), on_enter_pancam)
            .add_systems(OnExit(OgleMode::Pancam), on_exit_pancam)
            .add_systems(OnEnter(OgleMode::Following), on_enter_following)
            .add_systems(OnExit(OgleMode::Following), on_exit_following);
    }
}

fn setup(mut commands: Commands, settings: Res<OgleSettings>) {
    commands.spawn((
        Camera2dBundle::default(),
        bevy_pancam::PanCam {
            // Because the default mode of ogle is `OgleMode::Frozen`, not `OgleMode::Pancam`, we need to disable it.
            enabled: false,
            min_scale: settings.min_scale,
            max_scale: settings.max_scale,
            min_x: settings.min_x,
            max_x: settings.max_x,
            min_y: settings.min_y,
            max_y: settings.max_y,
            ..default()
        },
    ));
}

fn on_enter_frozen() {
    info!("Freezing camera");
}

fn on_exit_frozen() {
    info!("Unfreezing camera");
}

fn on_enter_pancam(mut query: Query<&mut bevy_pancam::PanCam>) {
    let mut pancam = query.single_mut();
    info!("Enabling pancam");
    pancam.enabled = true;
}

fn on_exit_pancam(mut query: Query<&mut bevy_pancam::PanCam>) {
    let mut pancam = query.single_mut();
    info!("Disabling pancam");
    pancam.enabled = false;
}

fn on_enter_following() {
    info!("Enabling follow camera");
}

fn on_exit_following() {
    info!("Disabling follow camera");
}

#[allow(clippy::too_many_arguments)]
fn follow_target(
    time: Res<Time>,
    target: Res<OgleTarget>,
    mut rig: ResMut<OgleRig>,
    query_transform: Query<&Transform, Without<Camera>>,
    mut query_cam: Query<(&mut OrthographicProjection, &mut Transform), With<Camera>>,
    mut scroll_events: EventReader<MouseWheel>,
) {
    let Ok((mut proj, mut camera_transform)) = query_cam.get_single_mut() else {
        return;
    };
    let prev_z = rig.driver::<Position>().position.z;
    match *target {
        OgleTarget::Position(pos) => {
            rig.driver_mut::<Position>().position = mint::Point3 {
                x: pos.x,
                y: pos.y,
                z: prev_z,
            };
        }
        OgleTarget::Entity(e) => {
            if let Ok(transform) = query_transform.get(e) {
                rig.driver_mut::<Position>().position = mint::Point3 {
                    x: transform.translation.x,
                    y: transform.translation.y,
                    z: prev_z,
                };
            } else {
                error!("entity target has no transform");
            };
        }
        OgleTarget::EntityWithOffset((e, offset)) => {
            if let Ok(transform) = query_transform.get(e) {
                rig.driver_mut::<Position>().position = mint::Point3 {
                    x: transform.translation.x + offset.x,
                    y: transform.translation.y + offset.y,
                    z: prev_z,
                };
            } else {
                error!("entity target has no transform");
            };
        }
        OgleTarget::None => {}
    }

    let pixels_per_line = 100.; // Maybe make configurable?
    let scroll = scroll_events
        .read()
        .map(|ev| match ev.unit {
            MouseScrollUnit::Pixel => ev.y,
            MouseScrollUnit::Line => ev.y * pixels_per_line,
        })
        .sum::<f32>();

    if scroll != 0. {
        rig.driver_mut::<Position>().position.z *= 1. + -scroll * 0.001;
    }

    rig.update(time.delta_seconds());

    camera_transform.rotation = Quat::from_xyzw(
        rig.final_transform.rotation.v.x,
        rig.final_transform.rotation.v.y,
        rig.final_transform.rotation.v.z,
        rig.final_transform.rotation.s,
    );
    camera_transform.translation.x = rig.final_transform.position.x;
    camera_transform.translation.y = rig.final_transform.position.y;
    proj.scale = rig.final_transform.position.z;
}

fn keep_within_settings(
    mut rig: ResMut<OgleRig>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    settings: Res<OgleSettings>,
    mut query_cam: Query<(&mut OrthographicProjection, &mut Transform), With<Camera>>,
) {
    let Ok((mut proj, mut camera_transform)) = query_cam.get_single_mut() else {
        return;
    };

    let window = primary_window.single();
    let window_size = Vec2::new(window.width(), window.height());

    // Apply scaling constraints
    let (min_scale, max_scale) = (
        settings.min_scale,
        settings.max_scale.unwrap_or(f32::INFINITY),
    );

    // If there is both a min and max boundary, that limits how far we can zoom. Make sure we don't exceed that
    let scale_constrained = BVec2::new(
        settings.min_x.is_some() && settings.max_x.is_some(),
        settings.min_y.is_some() && settings.max_y.is_some(),
    );

    if scale_constrained.x || scale_constrained.y {
        let bounds_width = if let (Some(min_x), Some(max_x)) = (settings.min_x, settings.max_x) {
            max_x - min_x
        } else {
            f32::INFINITY
        };

        let bounds_height = if let (Some(min_y), Some(max_y)) = (settings.min_y, settings.max_y) {
            max_y - min_y
        } else {
            f32::INFINITY
        };

        let bounds_size = vec2(bounds_width, bounds_height);
        let max_safe_scale = max_scale_within_bounds(bounds_size, &proj, window_size);

        if scale_constrained.x {
            proj.scale = proj.scale.min(max_safe_scale.x);
        }
        if scale_constrained.y {
            proj.scale = proj.scale.min(max_safe_scale.y);
        }
    }
    proj.scale = proj.scale.clamp(min_scale, max_scale);
    rig.driver_mut::<Position>().position.z = proj.scale;

    // Keep within bounds
    let proj_size = proj.area.size();
    let half_of_viewport = proj_size / 2.;
    if let Some(min_x_bound) = settings.min_x {
        let min_safe_cam_x = min_x_bound + half_of_viewport.x;
        camera_transform.translation.x = camera_transform.translation.x.max(min_safe_cam_x);
    }
    if let Some(max_x_bound) = settings.max_x {
        let max_safe_cam_x = max_x_bound - half_of_viewport.x;
        camera_transform.translation.x = camera_transform.translation.x.min(max_safe_cam_x);
    }
    if let Some(min_y_bound) = settings.min_y {
        let min_safe_cam_y = min_y_bound + half_of_viewport.y;
        camera_transform.translation.y = camera_transform.translation.y.max(min_safe_cam_y);
    }
    if let Some(max_y_bound) = settings.max_y {
        let max_safe_cam_y = max_y_bound - half_of_viewport.y;
        camera_transform.translation.y = camera_transform.translation.y.min(max_safe_cam_y);
    }
}

/// max_scale_within_bounds is used to find the maximum safe zoom out/projection
/// scale when we have been provided with minimum and maximum x boundaries for
/// the camera.
fn max_scale_within_bounds(
    bounds_size: Vec2,
    proj: &OrthographicProjection,
    window_size: Vec2, //viewport?
) -> Vec2 {
    let mut p = proj.clone();
    p.scale = 1.;
    p.update(window_size.x, window_size.y);
    let base_world_size = p.area.size();
    bounds_size / base_world_size
}

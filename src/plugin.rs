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
    pub initial_settings: Option<OgleSettings>,
}

impl Plugin for OglePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<OgleRig>()
            .init_resource::<OgleTarget>()
            .init_state::<OgleMode>()
            .insert_resource(self.initial_settings.unwrap_or_default())
            .add_plugins(bevy_pancam::PanCamPlugin)
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (follow_target).run_if(in_state(OgleMode::Following)),
            )
            // TODO: Implement choreographing
            //.add_systems(
            //    Update,
            //    choreograph_target.run_if(in_state(OgleMode::Choreographed)),
            //)
            .add_systems(OnEnter(OgleMode::Frozen), on_enter_frozen)
            .add_systems(OnExit(OgleMode::Frozen), on_exit_frozen)
            .add_systems(OnEnter(OgleMode::Pancam), on_enter_pancam)
            .add_systems(OnExit(OgleMode::Pancam), on_exit_pancam)
            .add_systems(OnEnter(OgleMode::Following), on_enter_following)
            .add_systems(OnExit(OgleMode::Following), on_exit_following);
    }
}

fn setup(mut commands: Commands) {
    // TODO: Settings for pancam handled consistently with other ogle settings
    commands.spawn((
        Camera2dBundle::default(),
        // Because the default mode of ogle is `OgleMode::Frozen`, not `OgleMode::Pancam`, we need to disable it.
        bevy_pancam::PanCam {
            enabled: false,
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

fn on_enter_following(
    query_cam: Query<&Transform, With<Camera>>,
    query_proj: Query<&OrthographicProjection>,
    mut rig: ResMut<OgleRig>,
) {
    let camera_transform = query_cam.single(); // TODO: error handling
    let projection = query_proj.single();

    rig.driver_mut::<Position>().position = mint::Point3 {
        x: camera_transform.translation.x,
        y: camera_transform.translation.y,
        z: projection.scale,
    };
    //rig.driver_mut::<Arm>().offset.z = projection.scale;
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
    settings: Res<OgleSettings>,
    mut query_cam: Query<(&mut OrthographicProjection, &mut Transform), With<Camera>>,
    mut scroll_events: EventReader<MouseWheel>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
) {
    // TODO: Handle errors
    let (mut proj, mut camera_transform) = query_cam.single_mut();
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
            // TODO: Handle errors
            let transform = query_transform
                .get(e)
                .expect("entity target has no transform");
            rig.driver_mut::<Position>().position = mint::Point3 {
                x: transform.translation.x,
                y: transform.translation.y,
                z: prev_z,
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
        let window = primary_window.single();
        let window_size = Vec2::new(window.width(), window.height());

        let old_scale = rig.driver::<Arm>().offset.z;
        let mut new_scale = (old_scale * (1. + -scroll * 0.001)).max(settings.min_scale);

        // Apply max scale constraint
        if let Some(max_scale) = settings.max_scale {
            new_scale = new_scale.min(max_scale);
        }

        // If there is both a min and max boundary, that limits how far we can zoom. Make sure we don't exceed that
        let scale_constrained = BVec2::new(
            settings.min_x.is_some() && settings.max_x.is_some(),
            settings.min_y.is_some() && settings.max_y.is_some(),
        );

        if scale_constrained.x || scale_constrained.y {
            let bounds_width = if let (Some(min_x), Some(max_x)) = (settings.min_x, settings.max_x)
            {
                max_x - min_x
            } else {
                f32::INFINITY
            };

            let bounds_height = if let (Some(min_y), Some(max_y)) = (settings.min_y, settings.max_y)
            {
                max_y - min_y
            } else {
                f32::INFINITY
            };

            let bounds_size = vec2(bounds_width, bounds_height);
            let max_safe_scale = max_scale_within_bounds(bounds_size, &proj, window_size);

            if scale_constrained.x {
                new_scale = new_scale.min(max_safe_scale.x);
            }

            if scale_constrained.y {
                new_scale = new_scale.min(max_safe_scale.y);
            }
        }

        rig.driver_mut::<Position>().position.z = new_scale;
        rig.driver_mut::<Arm>().offset.z = new_scale;
    }

    rig.update(time.delta_seconds());

    camera_transform.translation.x = rig.final_transform.position.x;
    camera_transform.translation.y = rig.final_transform.position.y;
    camera_transform.rotation = Quat::from_xyzw(
        rig.final_transform.rotation.v.x,
        rig.final_transform.rotation.v.y,
        rig.final_transform.rotation.v.z,
        rig.final_transform.rotation.s,
    );
    proj.scale = rig.final_transform.position.z;
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

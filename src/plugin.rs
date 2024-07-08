use crate::{rig::OgleRig, OgleMode, OgleTarget};
use bevy::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin};
use dolly::prelude::*;

// TODO: These should be configurable and play well with pancam
const MIN_ZOOM: f32 = -0.99;
const MAX_ZOOM: f32 = 1000.0;
const ZOOM_SPEED: f32 = 2.0;

#[derive(Default)]
pub struct OglePlugin;

impl Plugin for OglePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<OgleRig>()
            .init_resource::<OgleTarget>()
            .init_state::<OgleMode>()
            .add_plugins(PanCamPlugin)
            .add_systems(Startup, setup)
            .add_systems(Update, follow_target.run_if(in_state(OgleMode::Following)))
            .add_systems(
                Update,
                choreograph_target.run_if(in_state(OgleMode::Choreographed)),
            )
            .add_systems(OnEnter(OgleMode::Pancam), on_enter_pancam)
            .add_systems(OnExit(OgleMode::Pancam), on_exit_pancam)
            .add_systems(OnEnter(OgleMode::Following), on_enter_following)
            .add_systems(OnExit(OgleMode::Following), on_exit_following);
    }
}

fn setup(mut commands: Commands) {
    // TODO: Settings for pancam handled consistently with other ogle settings
    commands.spawn(bevy_pancam::PanCam::default());
}

fn on_enter_pancam(mut query: Query<&mut PanCam>) {
    let mut pancam = query.single_mut();
    info!("Enabling pancam");
    pancam.enabled = true;
}

fn on_exit_pancam(mut query: Query<&mut PanCam>) {
    let mut pancam = query.single_mut();
    info!("Disabling pancam");
    pancam.enabled = false;
}

fn choreograph_target() {
    todo!("handle camera choreographs, like following a spline")
}

fn on_enter_following() {
    info!("Enabling following");
}

fn on_exit_following() {
    info!("Disabling following");
}

fn follow_target(
    time: Res<Time>,
    target: Res<OgleTarget>,
    mut rig: ResMut<OgleRig>,
    query_transform: Query<&Transform, Without<Camera>>,
    mut query_cam: Query<&mut Transform, With<Camera>>,
    mut proj_query: Query<&mut OrthographicProjection>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let mut proj = proj_query.single_mut();
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        rig.driver_mut::<Arm>().offset.z -= proj.scale * ZOOM_SPEED * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        rig.driver_mut::<Arm>().offset.z += proj.scale * ZOOM_SPEED * time.delta_seconds();
    }
    rig.driver_mut::<Arm>().offset.z = rig.driver_mut::<Arm>().offset.z.clamp(MIN_ZOOM, MAX_ZOOM);

    match *target {
        OgleTarget::Position(pos) => {
            rig.driver_mut::<Position>().position = mint::Point3 {
                x: pos.x,
                y: pos.y,
                z: 1.0,
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
                z: 1.0,
            };
        }
        OgleTarget::None => {}
    }

    rig.update(time.delta_seconds());

    if let Some(mut camera_transform) = query_cam.iter_mut().next() {
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
}

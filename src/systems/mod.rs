use crate::{OgleCam, OgleMode, OgleTarget};
use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};
use dolly::prelude::*;

pub mod follow;
pub mod pancam;

pub fn do_follow_target(
    query_transform: Query<&Transform, Without<Camera>>,
    mut query_cam: Query<&mut OgleCam, With<Camera>>,
) {
    let Ok(mut cam) = query_cam.get_single_mut() else {
        return;
    };
    if cam.mode != OgleMode::Following {
        return;
    }

    let prev_z = cam.rig.driver::<Position>().position.z;

    match cam.target {
        OgleTarget::Position(pos) => {
            cam.rig.driver_mut::<Position>().position = mint::Point3 {
                x: pos.x,
                y: pos.y,
                z: prev_z,
            }
        }
        OgleTarget::Entity(entity) => {
            if let Ok(transform) = query_transform.get(entity) {
                cam.rig.driver_mut::<Position>().position = mint::Point3 {
                    x: transform.translation.x,
                    y: transform.translation.y,
                    z: prev_z,
                };
            }
        }
        OgleTarget::EntityWithOffset((entity, offset)) => {
            if let Ok(transform) = query_transform.get(entity) {
                cam.rig.driver_mut::<Position>().position = mint::Point3 {
                    x: transform.translation.x + offset.x,
                    y: transform.translation.y + offset.y,
                    z: prev_z,
                };
            }
        }
        OgleTarget::None => {}
    }
}

pub fn do_camera_zooming(
    mut query_cam: Query<&mut OgleCam, With<Camera>>,
    mut scroll_events: EventReader<MouseWheel>,
) {
    let Ok(mut cam) = query_cam.get_single_mut() else {
        return;
    };
    match cam.mode {
        OgleMode::Pancam | OgleMode::Following | OgleMode::ZoomOnly => {}
        OgleMode::Frozen => return,
    };

    // Zoom handling
    let scroll_amount = scroll_events
        .read()
        .map(|ev| match ev.unit {
            MouseScrollUnit::Pixel => ev.y,
            MouseScrollUnit::Line => ev.y * 100., // Adjustable sensitivity
        })
        .sum::<f32>();

    if scroll_amount != 0. {
        cam.rig.driver_mut::<Position>().position.z *= 1.0 - scroll_amount * 0.001;
    }
}

pub fn commit_camera_changes(
    time: Res<Time>,
    mut query_cam: Query<(&mut OgleCam, &mut OrthographicProjection, &mut Transform), With<Camera>>,
) {
    let Ok((mut cam, mut proj, mut camera_transform)) = query_cam.get_single_mut() else {
        return;
    };

    cam.rig.update(time.delta_secs());

    // Update camera transform
    camera_transform.translation = Vec3::new(
        cam.rig.final_transform.position.x,
        cam.rig.final_transform.position.y,
        camera_transform.translation.z,
    );
    proj.scale = cam.rig.final_transform.position.z;
}

use crate::{OgleCam, OgleMode, OgleSettings, OgleTarget};
use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    render::camera::CameraProjection as _,
};
use dolly::prelude::*;
use std::ops::RangeInclusive;

#[allow(clippy::too_many_arguments)]
pub fn follow_target(
    time: Res<Time>,
    query_transform: Query<&Transform, Without<Camera>>,
    mut query_cam: Query<(&mut OgleCam, &mut OrthographicProjection, &mut Transform), With<Camera>>,
    mut scroll_events: EventReader<MouseWheel>,
) {
    let Ok((mut cam, mut proj, mut camera_transform)) = query_cam.get_single_mut() else {
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

    cam.rig.update(time.delta_secs());

    // Update camera transform
    camera_transform.translation = Vec3::new(
        cam.rig.final_transform.position.x,
        cam.rig.final_transform.position.y,
        camera_transform.translation.z,
    );
    proj.scale = cam.rig.final_transform.position.z;
}

pub fn keep_within_settings(
    mut rig: ResMut<OgleCam>,
    settings: Res<OgleSettings>,
    mut query_cam: Query<(&mut OrthographicProjection, &mut Transform), With<Camera>>,
) {
    let Ok((mut proj, mut camera_transform)) = query_cam.get_single_mut() else {
        return;
    };

    if settings.min_x >= settings.max_x || settings.min_y >= settings.max_y {
        error!("Invalid camera settings. Ogle movement constrained.");
        return;
    }

    // Clamping position and scale
    let half_viewport = proj.area.half_size();

    let min_x = settings.min_x + half_viewport.x;
    let max_x = settings.max_x - half_viewport.x;
    let min_y = settings.min_y + half_viewport.y;
    let max_y = settings.max_y - half_viewport.y;
    let bounded_area = Rect::new(min_x, min_y, max_x, max_y);
    if min_x < max_x {
        camera_transform.translation.x = camera_transform.translation.x.clamp(min_x, max_x);
    }
    if min_y < max_y {
        camera_transform.translation.y = camera_transform.translation.y.clamp(min_y, max_y);
    }

    proj.scale = proj.scale.clamp(settings.min_scale, settings.max_scale);
    let window_size = proj.area.size();
    constrain_proj_scale(
        &mut proj,
        bounded_area.size(),
        &(settings.min_scale..=settings.max_scale),
        window_size,
    );
    //let proj_size = proj.area.max / old_scale;
    rig.driver_mut::<Position>().position.z = proj.scale; // Sync rig scale
}

/// `max_scale_within_bounds` is used to find the maximum safe zoom out/projection
/// scale when we have been provided with minimum and maximum x boundaries for
/// the camera.
fn max_scale_within_bounds(
    bounded_area_size: Vec2,
    proj: &OrthographicProjection,
    viewport_size: Vec2,
) -> Vec2 {
    let mut proj = proj.clone();
    proj.scale = 1.;
    proj.update(viewport_size.x, viewport_size.y);
    let base_world_size = proj.area.size();
    bounded_area_size / base_world_size
}

/// Makes sure that the camera projection scale stays in the provided bounds
/// and range.
fn constrain_proj_scale(
    proj: &mut OrthographicProjection,
    bounded_area_size: Vec2,
    scale_range: &RangeInclusive<f32>,
    window_size: Vec2,
) {
    proj.scale = proj.scale.clamp(*scale_range.start(), *scale_range.end());

    // If there is both a min and max boundary, that limits how far we can zoom.
    // Make sure we don't exceed that
    if bounded_area_size.x.is_finite() || bounded_area_size.y.is_finite() {
        let max_safe_scale = max_scale_within_bounds(bounded_area_size, proj, window_size);
        proj.scale = proj.scale.min(max_safe_scale.x).min(max_safe_scale.y);
    }
}

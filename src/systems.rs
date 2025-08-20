use std::ops::DerefMut;

use crate::{OgleCam, OgleMode, OgleTarget};
use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    window::PrimaryWindow,
};
use dolly::prelude::*;

pub fn do_follow_target(query_transform: Query<&Transform>, mut query_cam: Query<&mut OgleCam>) {
    for mut cam in query_cam.iter_mut() {
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
}

pub fn do_camera_zooming(
    mut query_cam: Query<&mut OgleCam>,
    mut scroll_events: EventReader<MouseWheel>,
) {
    for mut cam in query_cam.iter_mut() {
        match cam.mode {
            OgleMode::Pancam | OgleMode::Following | OgleMode::ZoomOnly => {}
            OgleMode::Frozen => return,
        };

        // Zoom handling
        let scroll_amount = scroll_events
            .read()
            .map(|ev| match ev.unit {
                MouseScrollUnit::Pixel => ev.y,
                MouseScrollUnit::Line => ev.y * cam.settings.zoom_sensitivity,
            })
            .sum::<f32>();

        if scroll_amount != 0. {
            cam.rig.driver_mut::<Position>().position.z *= 1.0 - scroll_amount * 0.001;
        }
    }
}

pub fn do_pancam_movement(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    keyboard_buttons: Res<ButtonInput<KeyCode>>,
    mut query_cam: Query<(&mut OgleCam, &mut Transform, &Projection)>,
    mut last_pos: Local<Option<Vec2>>,
    time: Res<Time>,
) {
    let Ok(window) = primary_window.single() else {
        return;
    };

    // Use position instead of MouseMotion, otherwise we don't get acceleration
    // movement
    let current_pos = match window.cursor_position() {
        Some(c) => Vec2::new(c.x, -c.y),
        None => return,
    };
    let delta_device_pixels = current_pos - last_pos.unwrap_or(current_pos);

    for (mut ogle_cam, transform, projection) in query_cam.iter_mut() {
        if ogle_cam.mode != OgleMode::Pancam {
            continue;
        }
        let Projection::Orthographic(projection) = projection else {
            continue;
        };

        let mouse_delta = if !ogle_cam
            .settings
            .pancam
            .grab_buttons
            .iter()
            .any(|btn| mouse_buttons.pressed(*btn) && !mouse_buttons.just_pressed(*btn))
        {
            Vec2::ZERO
        } else {
            delta_device_pixels * projection.scale
        };

        // Keyboard delta
        let keyboard_direction = {
            let mut direction = Vec2::ZERO;
            if ogle_cam
                .settings
                .pancam
                .left_keys
                .iter()
                .any(|key| keyboard_buttons.pressed(*key))
            {
                direction.x -= 1.;
            }
            if ogle_cam
                .settings
                .pancam
                .right_keys
                .iter()
                .any(|key| keyboard_buttons.pressed(*key))
            {
                direction.x += 1.;
            }
            if ogle_cam
                .settings
                .pancam
                .up_keys
                .iter()
                .any(|key| keyboard_buttons.pressed(*key))
            {
                direction.y += 1.;
            }
            if ogle_cam
                .settings
                .pancam
                .down_keys
                .iter()
                .any(|key| keyboard_buttons.pressed(*key))
            {
                direction.y -= 1.;
            }
            direction
        };
        let keyboard_delta = time.delta_secs()
            * keyboard_direction.normalize_or_zero()
            * ogle_cam.settings.pancam.keyboard_speed
            * projection.scale;

        // Get final delta
        let delta = mouse_delta - keyboard_delta;
        if delta == Vec2::ZERO {
            continue;
        }

        // The proposed new camera position
        let new_pos = transform.translation.truncate() - delta;
        ogle_cam.rig.driver_mut::<Position>().position.x = new_pos.x;
        ogle_cam.rig.driver_mut::<Position>().position.y = new_pos.y;
    }
    *last_pos = Some(current_pos);
}
pub fn do_camera_bounding(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut query_cam: Query<(&mut OgleCam, &Camera, &Projection)>,
) {
    let Ok(window) = primary_window.single() else {
        return;
    };
    let window_size = window.size();

    for (mut cam, camera, projection) in query_cam.iter_mut() {
        if !cam.settings.bounds.enabled {
            continue;
        }

        let Projection::Orthographic(_projection) = projection else {
            continue;
        };

        // Calculate the bounds area size
        let bounds_width = cam.settings.bounds.max_x - cam.settings.bounds.min_x;
        let bounds_height = cam.settings.bounds.max_y - cam.settings.bounds.min_y;

        // Get viewport size in pixels
        let viewport_size = camera.logical_viewport_size().unwrap_or(window_size);

        // Calculate what scale would be needed to fit the bounds area exactly in the viewport
        let scale_to_fit_width = bounds_width / viewport_size.x;
        let scale_to_fit_height = bounds_height / viewport_size.y;
        // Use min instead of max to fill the viewport (may crop bounds)
        let scale_to_fit_bounds = scale_to_fit_width.min(scale_to_fit_height);

        // The maximum allowed scale (most zoomed out) should not exceed what's needed to fit bounds
        // But we also respect the user's min/max scale settings
        let max_allowed_scale = cam.settings.bounds.max_scale.min(scale_to_fit_bounds);

        // Ensure we have a valid range - if scale_to_fit_bounds is smaller than min_scale,
        // we prioritize the bounds constraint over the user's min_scale
        let effective_min_scale = cam.settings.bounds.min_scale.min(scale_to_fit_bounds);
        let scale_range = effective_min_scale..=max_allowed_scale;

        // Bound the zoom
        cam.rig.driver_mut::<Position>().position.z = cam
            .rig
            .driver::<Position>()
            .position
            .z
            .clamp(*scale_range.start(), *scale_range.end());

        // Get the current scale after clamping
        let current_scale = cam.rig.driver::<Position>().position.z;

        // Calculate viewport size in world units
        let world_viewport_width = viewport_size.x * current_scale;
        let world_viewport_height = viewport_size.y * current_scale;

        // Calculate half sizes for easier math
        let half_width = world_viewport_width * 0.5;
        let half_height = world_viewport_height * 0.5;

        // Calculate the bounds for the camera center position
        // The camera center must stay within these bounds to keep the entire viewport within the target area
        let effective_min_x = cam.settings.bounds.min_x + half_width;
        let effective_max_x = cam.settings.bounds.max_x - half_width;
        let effective_min_y = cam.settings.bounds.min_y + half_height;
        let effective_max_y = cam.settings.bounds.max_y - half_height;

        // Only apply bounds if they make sense (i.e., the bounded area is larger than the viewport)
        if effective_min_x <= effective_max_x {
            cam.rig.driver_mut::<Position>().position.x = cam
                .rig
                .driver::<Position>()
                .position
                .x
                .clamp(effective_min_x, effective_max_x);
        } else {
            // If bounds are too tight, center the camera
            let bounds_center_x = (cam.settings.bounds.min_x + cam.settings.bounds.max_x) * 0.5;
            cam.rig.driver_mut::<Position>().position.x = bounds_center_x;
        }

        if effective_min_y <= effective_max_y {
            cam.rig.driver_mut::<Position>().position.y = cam
                .rig
                .driver::<Position>()
                .position
                .y
                .clamp(effective_min_y, effective_max_y);
        } else {
            // If bounds are too tight, center the camera
            let bounds_center_y = (cam.settings.bounds.min_y + cam.settings.bounds.max_y) * 0.5;
            cam.rig.driver_mut::<Position>().position.y = bounds_center_y;
        }
    }
}

pub fn commit_camera_changes(
    time: Res<Time>,
    mut query_cam: Query<(&mut OgleCam, &mut Projection, &mut Transform)>,
) {
    for (mut cam, mut projection, mut camera_transform) in query_cam.iter_mut() {
        let Projection::Orthographic(ref mut projection) = projection.deref_mut() else {
            continue;
        };
        // Apply final transform update
        cam.rig.update(time.delta_secs());
        if cam.mode == OgleMode::Pancam {
            let driver_pos = cam.rig.driver::<Position>().position;
            cam.rig.final_transform.position.x = driver_pos.x;
            cam.rig.final_transform.position.y = driver_pos.y;
        }
        camera_transform.translation = Vec3::new(
            cam.rig.final_transform.position.x,
            cam.rig.final_transform.position.y,
            camera_transform.translation.z,
        );
        projection.scale = cam.rig.final_transform.position.z;
    }
}

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
    mut query_cam: Query<(
        &mut OgleCam,
        &Camera,
        &mut Transform,
        &OrthographicProjection,
    )>,
    mut last_pos: Local<Option<Vec2>>,
    time: Res<Time>,
) {
    let Ok(window) = primary_window.get_single() else {
        return;
    };
    let window_size = window.size();

    // Use position instead of MouseMotion, otherwise we don't get acceleration
    // movement
    let current_pos = match window.cursor_position() {
        Some(c) => Vec2::new(c.x, -c.y),
        None => return,
    };
    let delta_device_pixels = current_pos - last_pos.unwrap_or(current_pos);

    for (mut ogle_cam, camera, transform, projection) in query_cam.iter_mut() {
        if ogle_cam.mode != OgleMode::Pancam {
            continue;
        }

        let proj_area_size = projection.area.size();

        let mouse_delta = if !ogle_cam
            .settings
            .pancam
            .grab_buttons
            .iter()
            .any(|btn| mouse_buttons.pressed(*btn) && !mouse_buttons.just_pressed(*btn))
        {
            Vec2::ZERO
        } else {
            let viewport_size = camera.logical_viewport_size().unwrap_or(window_size);
            ogle_cam.settings.pancam.drag_speed * delta_device_pixels * proj_area_size
                / viewport_size
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

pub fn do_camera_bounding(mut query_cam: Query<&mut OgleCam>) {
    for mut cam in query_cam.iter_mut() {
        if !cam.settings.bounds.enabled {
            return;
        }

        // Bound the zoom
        let scale_range = cam.settings.bounds.min_scale..=cam.settings.bounds.max_scale;
        cam.rig.driver_mut::<Position>().position.z = cam
            .rig
            .driver::<Position>()
            .position
            .z
            .clamp(*scale_range.start(), *scale_range.end());

        // Bound position
        cam.rig.driver_mut::<Position>().position.x = cam
            .rig
            .driver::<Position>()
            .position
            .x
            .clamp(cam.settings.bounds.min_x, cam.settings.bounds.max_x);
        cam.rig.driver_mut::<Position>().position.y = cam
            .rig
            .driver::<Position>()
            .position
            .y
            .clamp(cam.settings.bounds.min_y, cam.settings.bounds.max_y);
    }
}

pub fn commit_camera_changes(
    time: Res<Time>,
    mut query_cam: Query<(&mut OgleCam, &mut OrthographicProjection, &mut Transform)>,
) {
    for (mut cam, mut proj, mut camera_transform) in query_cam.iter_mut() {
        // Apply final transform update
        cam.rig.update(time.delta_secs());
        camera_transform.translation = Vec3::new(
            cam.rig.final_transform.position.x,
            cam.rig.final_transform.position.y,
            camera_transform.translation.z,
        );
        proj.scale = cam.rig.final_transform.position.z;
    }
}

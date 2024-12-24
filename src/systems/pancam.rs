use crate::OgleCam;
use bevy::{prelude::*, window::PrimaryWindow};

fn do_camera_movement(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    keyboard_buttons: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&OgleCam, &Camera, &mut Transform, &OrthographicProjection)>,
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

    for (ogle_cam, camera, mut transform, projection) in &mut query {
        if !ogle_cam.enabled {
            continue;
        }

        let proj_area_size = projection.area.size();

        let mouse_delta = if !ogle_cam
            .grab_buttons
            .iter()
            .any(|btn| mouse_buttons.pressed(*btn) && !mouse_buttons.just_pressed(*btn))
        {
            Vec2::ZERO
        } else {
            let viewport_size = camera.logical_viewport_size().unwrap_or(window_size);
            delta_device_pixels * proj_area_size / viewport_size
        };

        let direction = ogle_cam.move_keys.direction(&keyboard_buttons);

        let keyboard_delta =
            time.delta_secs() * direction.normalize_or_zero() * ogle_cam.speed * projection.scale;
        let delta = mouse_delta - keyboard_delta;

        if delta == Vec2::ZERO {
            continue;
        }

        // The proposed new camera position
        let proposed_cam_pos = transform.translation.truncate() - delta;

        transform.translation =
            clamp_to_safe_zone(proposed_cam_pos, ogle_cam.aabb(), proj_area_size)
                .extend(transform.translation.z);
    }
    *last_pos = Some(current_pos);
}

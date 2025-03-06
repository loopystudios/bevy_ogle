use crate::{systems, OgleSystemSet};
use bevy::prelude::*;

#[derive(Default)]
pub struct OglePlugin;

impl Plugin for OglePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                systems::do_follow_target,
                systems::do_camera_zooming,
                systems::do_pancam_movement,
                systems::do_camera_bounding,
                systems::commit_camera_changes,
            )
                .chain()
                .in_set(OgleSystemSet),
        );
    }
}

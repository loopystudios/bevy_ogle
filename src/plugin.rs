use crate::{systems, OgleSettings, OgleSystemSet};
use bevy::prelude::*;

#[derive(Default)]
pub struct OglePlugin {
    pub initial_settings: OgleSettings,
}

impl Plugin for OglePlugin {
    fn build(&self, app: &mut App) {
        if self.initial_settings.min_x >= self.initial_settings.max_x
            || self.initial_settings.min_y >= self.initial_settings.max_y
        {
            panic!("Invalid `OgleSettings`: Ensure min_x < max_x and min_y < max_y.");
        }

        app.add_systems(
            Update,
            (
                systems::do_follow_target,
                systems::do_camera_zooming,
                systems::commit_camera_changes,
            )
                .chain()
                .in_set(OgleSystemSet),
        );
    }
}

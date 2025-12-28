use crate::{systems, OgleSystems};
use bevy::prelude::*;

#[derive(Default)]
pub struct OglePlugin;

impl Plugin for OglePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                OgleSystems::Update,
                OgleSystems::Input,
                OgleSystems::Correction,
                OgleSystems::Commit,
            )
                .chain(),
        );
        app.add_systems(
            Update,
            systems::do_follow_target.in_set(OgleSystems::Update),
        )
        .add_systems(
            Update,
            (systems::do_camera_zooming, systems::do_pancam_movement)
                .chain()
                .in_set(OgleSystems::Input),
        )
        .add_systems(
            Update,
            systems::correct_to_camera_bounding.in_set(OgleSystems::Correction),
        )
        .add_systems(
            Update,
            systems::commit_camera_changes.in_set(OgleSystems::Commit),
        );

        #[cfg(feature = "internal_bevy_egui")]
        app.add_plugins(crate::egui_support::EguiPanCamPlugin);
    }
}

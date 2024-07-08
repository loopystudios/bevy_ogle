use bevy::prelude::*;
use dolly::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct OgleRig(CameraRig);

impl Default for OgleRig {
    fn default() -> Self {
        let pos = mint::Point3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
        Self(
            CameraRig::builder()
                .with(Position::new(pos))
                .with(Arm::new(pos))
                .with(Smooth::new_position(1.5).predictive(false))
                .build(),
        )
    }
}

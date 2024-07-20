use bevy::prelude::*;
use dolly::prelude::*;

mod commands;
pub use commands::OgleCommandExt;

mod plugin;
pub use plugin::OglePlugin;

#[derive(Resource, Debug, PartialEq)]
pub enum OgleTarget {
    Position(Vec2),
    Entity(Entity),
    EntityWithOffset((Entity, Vec2)),
    None,
}

impl Default for OgleTarget {
    fn default() -> Self {
        Self::None
    }
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum OgleMode {
    /// The camera will not move under normal circumstances.
    #[default]
    Frozen,
    /// The camera should exponentially follow its target.
    Following,
    /// The camera is in a detached pancam mode.
    Pancam,
}

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
                .with(Smooth::new_position(1.5).predictive(false))
                .build(),
        )
    }
}

#[derive(Resource, Debug, Clone, Copy, PartialEq)]
pub struct OgleSettings {
    /// The minimum scale for the camera
    ///
    /// The orthographic projection's scale will be clamped at this value when zooming in
    pub min_scale: f32,
    /// The maximum scale for the camera
    ///
    /// If present, the orthographic projection's scale will be clamped at
    /// this value when zooming out.
    pub max_scale: Option<f32>,
    /// The minimum x position of the camera window
    ///
    /// If present, the orthographic projection will be clamped to this boundary both
    /// when dragging the window, and zooming out.
    pub min_x: Option<f32>,
    /// The maximum x position of the camera window
    ///
    /// If present, the orthographic projection will be clamped to this boundary both
    /// when dragging the window, and zooming out.
    pub max_x: Option<f32>,
    /// The minimum y position of the camera window
    ///
    /// If present, the orthographic projection will be clamped to this boundary both
    /// when dragging the window, and zooming out.
    pub min_y: Option<f32>,
    /// The maximum y position of the camera window
    ///
    /// If present, the orthographic projection will be clamped to this boundary both
    /// when dragging the window, and zooming out.
    pub max_y: Option<f32>,
}

impl Default for OgleSettings {
    fn default() -> Self {
        Self {
            min_scale: -(1.0 - 0.00001),
            max_scale: None,
            min_x: None,
            max_x: None,
            min_y: None,
            max_y: None,
        }
    }
}

pub mod prelude {
    pub use super::{OgleCommandExt, OgleMode, OgleRig, OgleSettings, OgleTarget};
}

use bevy::prelude::*;
use dolly::prelude::*;

mod systems;

mod plugin;
pub use plugin::OglePlugin;

/// System set to allow ordering of `OglePlugin`
#[derive(Debug, Clone, Copy, SystemSet, PartialEq, Eq, Hash)]
pub struct OgleSystemSet;

#[derive(Component, Debug)]
pub struct OgleCam {
    pub settings: OgleSettings,
    pub target: OgleTarget,
    pub mode: OgleMode,
    rig: CameraRig,
}

impl OgleCam {
    pub fn new(settings: OgleSettings) -> Self {
        Self {
            settings,
            target: Default::default(),
            mode: Default::default(),
            rig: CameraRig::builder()
                .with(Position::new(mint::Point3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                }))
                .with(Smooth::new_position(1.5).predictive(false))
                .build(),
        }
    }
}

#[derive(Clone, PartialEq, Debug, Default)]
pub enum OgleTarget {
    Position(Vec2),
    Entity(Entity),
    EntityWithOffset((Entity, Vec2)),
    #[default]
    None,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum OgleMode {
    /// The camera will not move under normal circumstances.
    #[default]
    Frozen,
    /// The user can only zoom the camera.
    ZoomOnly,
    /// The camera should exponentially follow its target.
    Following,
    /// The camera is in a detached pancam mode.
    Pancam,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OgleSettings {
    /// The minimum scale for the camera
    pub min_scale: f32,
    /// The maximum scale for the camera
    pub max_scale: f32,
    /// The minimum x position of the camera window
    pub min_x: f32,
    /// The maximum x position of the camera window
    pub max_x: f32,
    /// The minimum y position of the camera window
    pub min_y: f32,
    /// The maximum y position of the camera window
    pub max_y: f32,
}

impl Default for OgleSettings {
    fn default() -> Self {
        Self {
            min_scale: -(1.0 - 0.00001),
            max_scale: f32::INFINITY,
            min_x: f32::NEG_INFINITY,
            max_x: f32::INFINITY,
            min_y: f32::NEG_INFINITY,
            max_y: f32::INFINITY,
        }
    }
}

pub mod prelude {
    pub use super::{OgleCam, OgleMode, OgleSettings, OgleTarget};
}

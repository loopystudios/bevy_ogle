use bevy::prelude::*;

mod rig;

mod commands;
pub use commands::OgleCommandExt;

mod plugin;
pub use plugin::OglePlugin;

#[derive(Resource, Debug, PartialEq)]
pub enum OgleTarget {
    Position(Vec2),
    Entity(Entity),
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
    /// The camera is being choreographed and should mirror its target position exactly.
    ///
    /// This is useful when the camera follows a spline, or you don't want loose following behavior.
    Choreographed,
    /// The camera is in a detached pancam mode.
    Pancam,
}

pub mod prelude {
    pub use super::{OgleCommandExt, OgleMode, OgleTarget};
}

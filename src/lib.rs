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
    pub fn new(settings: OgleSettings, target: OgleTarget, mode: OgleMode) -> Self {
        Self {
            settings,
            target,
            mode,
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

impl Default for OgleCam {
    fn default() -> Self {
        Self::new(Default::default(), Default::default(), Default::default())
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

#[derive(Debug, Clone, PartialEq)]
pub struct OgleSettings {
    /// Zoom sensitivity
    pub zoom_sensitivity: f32,
    /// Bounds for the camera
    pub bounds: OgleBoundingSettings,
    /// Settings for pancam mode
    pub pancam: OglePancamSettings,
}

impl Default for OgleSettings {
    fn default() -> Self {
        Self {
            zoom_sensitivity: 100.0,
            bounds: Default::default(),
            pancam: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OglePancamSettings {
    /// Speed for mouse drag movement
    pub drag_speed: f32,
    /// Speed for keyboard movement
    pub keyboard_speed: f32,
    /// Mouse buttons for dragging the pancam
    pub grab_buttons: Vec<MouseButton>,
    /// Keyboard keys for panning up
    pub up_keys: Vec<KeyCode>,
    /// Keyboard keys for panning down
    pub down_keys: Vec<KeyCode>,
    /// Keyboard keys for panning left
    pub left_keys: Vec<KeyCode>,
    /// Keyboard keys for panning right
    pub right_keys: Vec<KeyCode>,
}

impl Default for OglePancamSettings {
    fn default() -> Self {
        const GRAB_BUTTONS: [MouseButton; 3] =
            [MouseButton::Left, MouseButton::Right, MouseButton::Middle];
        const UP_KEYS: [KeyCode; 2] = [KeyCode::ArrowUp, KeyCode::KeyW];
        const DOWN_KEYS: [KeyCode; 2] = [KeyCode::ArrowDown, KeyCode::KeyS];
        const LEFT_KEYS: [KeyCode; 2] = [KeyCode::ArrowLeft, KeyCode::KeyA];
        const RIGHT_KEYS: [KeyCode; 2] = [KeyCode::ArrowRight, KeyCode::KeyD];
        Self {
            drag_speed: 10.0,
            keyboard_speed: 1000.0,
            grab_buttons: GRAB_BUTTONS.to_vec(),
            up_keys: UP_KEYS.to_vec(),
            down_keys: DOWN_KEYS.to_vec(),
            left_keys: LEFT_KEYS.to_vec(),
            right_keys: RIGHT_KEYS.to_vec(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OgleBoundingSettings {
    /// Whether the camera MUST remain bounded to the safe area.
    pub enabled: bool,
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

impl Default for OgleBoundingSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            min_scale: 0.00001,
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

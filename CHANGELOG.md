# Changelog

This changelog follows the patterns described here:
https://keepachangelog.com/en/1.0.0/

Subheadings to categorize changes are:
Added, Changed, Deprecated, Removed, Fixed, Security.

## [Unreleased]

- This release supports **Bevy 0.18**.

## [0.10.0]

- This release supports **Bevy 0.17**.

### Changed

- Added `OgleSystems::Input` and `OgleSystems::Correction` for finer control.

### Fixed

- Camera following will still happen during egui focus.

## [0.9.0]

- This release supports **Bevy 0.17**.

### Changed

- Split `OgleSystems` into `OgleSystems::Input` and `OgleSystems::Commit`.

### Fixed

- Camera changes made programmatically are now committed even over egui hovers.

## [0.8.0]

- This release supports **Bevy 0.17**.

### Added

- Egui support with the `bevy_egui_0_38` feature. This will make inputs ignored during egui focus.

### Changed

- Updated to Bevy 0.17.
- Renamed `OgleSystemSet` to `OgleSystems`.

## [0.7.0]

- This release supports **Bevy 0.16**.

### Added

- Added new `OgleMode::MoveOnly`.
- Added `OgleCam::teleport(&mut self)` to allow teleporting of the camera immediately.
- Added `OgleCam::position(&self)` to retrieve the current camera position easily.

### Changed

- Renamed `OgleMode::Following` to `OgleMode::Normal`.

## [0.6.3]

- This release supports **Bevy 0.16**.

### Fixed

- Camera bounding is now correctly tight.

## [0.6.2]

- This release supports **Bevy 0.16**.

### Fixed

- Window drag in pancam mode should now be 1 pixel per pixel dragged.

## [0.6.1]

- This release supports **Bevy 0.16**.

### Fixed

- Fixed a panic that occurs when the window is `None`.

## [0.6.0]

- This release supports **Bevy 0.16**.

### Changed

- Removed `OglePancamSettings.drag_speed`, since mouse drag now mirrors the device projection drag.

## [0.5.0]

- This release supports **Bevy 0.16**.

### Changed

- Updated to Bevy 0.16.

## [0.4.0]

- This release supports **Bevy 0.15**.

### Changed

- `OgleCam` now requires `Camera2d` as a required component.

## [0.3.0]

- This release supports **Bevy 0.15**.

### Added

- `ZoomOnly` camera mode.
- Opt-in to camera bounding via settings.

### Changed

- Updated to Bevy 0.15.
- Commands were removed; you should query and adjust target, mode, etc. through the `OgleCam` directly, which is now a component.

## [0.2.0]

- This release supports **Bevy 0.14**.

### Added

- Added entity with offset targeting via
  `commands.ogle_target_entity_with_offset(entity: Entity, offset: Vec2)`.
- Added `commands.ogle_target(target: OgleTarget)`.
- Added `commands.ogle_freeze()`.
- Added `commands.ogle_follow()`.
- Added `commands.ogle_pancam()`.

### Changed

- Renamed `commands.ogle_change_mode` to `commands.ogle_mode`.

## [0.1.0]

- This release supports **Bevy 0.14**.
- Initial release.


[unreleased]: https://github.com/loopystudios/bevy_ogle/compare/v0.10.0...HEAD
[0.10.0]: https://github.com/loopystudios/bevy_ogle/compare/v0.9.0...v0.10.0
[0.9.0]: https://github.com/loopystudios/bevy_ogle/compare/v0.8.0...v0.9.0
[0.8.0]: https://github.com/loopystudios/bevy_ogle/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/loopystudios/bevy_ogle/compare/v0.6.3...v0.7.0
[0.6.3]: https://github.com/loopystudios/bevy_ogle/compare/v0.6.2...v0.6.3
[0.6.2]: https://github.com/loopystudios/bevy_ogle/compare/v0.6.1...v0.6.2
[0.6.1]: https://github.com/loopystudios/bevy_ogle/compare/v0.6.0...v0.6.1
[0.6.0]: https://gith

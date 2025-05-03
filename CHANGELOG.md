# Changelog

<!-- Instructions

This changelog follows the patterns described here: <https://keepachangelog.com/en/1.0.0/>.

Subheadings to categorize changes are `added, changed, deprecated, removed, fixed, security`.

-->

## Unreleased

## 0.6.0

### Changed

- Removed `OglePancamSettings.drag_speed`, since mouse drag now mirrors the device projection drag.

## 0.5.0

### Changed

- Updated to bevy 0.16

## 0.4.0

### Changed

- `OgleCam` now requires `Camera2d` as a required component.

## 0.3.0

### Added

- `ZoomOnly` camera mode.
- Opt in to camera bounding via settings.

### Changed

- Updated to Bevy 0.15
- Commands were removed, you should query and adjust target, mode, etc. through the `OgleCam` directly, which is now a component.

## 0.2.0

### Added

- Added entity with offset targeting through `commands.ogle_target_entity_with_offset(entity:Entity, offset: Vec2)`.
- Added `commands.ogle_target(target: OgleTarget)`.
- Added `commands.ogle_freeze()`.
- Added `commands.ogle_follow()`.
- Added `commands.ogle_pancam()`.

### Changed

- Changed `commands.ogle_change_mode` to `commands.ogle_mode`.

## 0.1.0

- Initial release

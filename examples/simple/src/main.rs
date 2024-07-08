use std::ops::Deref;

use bevy::{color::palettes::css, prelude::*, transform::commands};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_ogle::{prelude::*, OglePlugin};
use rand::random;

#[derive(Component)]
struct ThingToFollow;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(OglePlugin)
        .add_systems(Startup, setup_scene)
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Camera2dBundle::default());
        })
        .add_systems(Update, move_target)
        .add_systems(Update, control_camera_ui)
        .run();
}

fn setup_scene(mut commands: Commands) {
    // Background
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: css::LIME.into(),
            custom_size: Some(Vec2::new(500.0, 500.0)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.),
        ..default()
    });

    // Moving thing for the camera to follow
    commands.spawn((
        ThingToFollow,
        SpriteBundle {
            sprite: Sprite {
                color: css::RED.into(),
                custom_size: Some(Vec2::new(5.0, 5.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.),
            ..default()
        },
    ));
}

fn move_target(
    time: Res<Time>,
    mut query_thing: Query<&mut Transform, With<ThingToFollow>>,
    mut gizmos: Gizmos,
) {
    let mut transform = query_thing.single_mut();
    transform.translation.x += time.delta_seconds() * (random::<f32>() * 500.0 - 500.0 / 2.0);
    transform.translation.y += time.delta_seconds() * (random::<f32>() * 500.0 - 500.0 / 2.0);
    gizmos.rect_2d(transform.translation.xy(), 0.0, (5.0, 5.0).into(), css::RED);
}

fn control_camera_ui(
    mut commands: Commands,
    mut contexts: EguiContexts,
    query_thing: Query<Entity, With<ThingToFollow>>,
    target: Res<OgleTarget>,
    mode: Res<State<OgleMode>>,
    mut next_mode: ResMut<NextState<OgleMode>>,
) {
    let window = egui::Window::new("Camera Controls")
        .anchor(egui::Align2::LEFT_TOP, [25.0, 25.0])
        .resizable(false)
        .title_bar(true);
    window.show(contexts.ctx_mut(), |ui| {
        ui.heading("Mode");
        let mut set_mode = mode.clone();
        if ui
            .radio_value(&mut set_mode, OgleMode::Frozen, "Frozen")
            .clicked()
            || ui
                .radio_value(&mut set_mode, OgleMode::Following, "Following")
                .clicked()
            || ui
                .radio_value(&mut set_mode, OgleMode::Choreographed, "Choreographed")
                .clicked()
            || ui
                .radio_value(&mut set_mode, OgleMode::Pancam, "Pancam")
                .clicked()
        {
            next_mode.set(set_mode);
        }

        ui.separator();
        ui.heading("Mode");
        let target_entity = query_thing.single();
        if ui.radio(*target == OgleTarget::None, "None").clicked() {
            commands.ogle_clear_target();
        }
        if ui
            .radio(*target == OgleTarget::Entity(target_entity), "Entity")
            .clicked()
        {
            commands.ogle_target_entity(target_entity);
        }
    });
}

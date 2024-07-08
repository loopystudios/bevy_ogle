use bevy::{color::palettes::css, prelude::*};
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
        .add_systems(Startup, setup_background)
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Camera2dBundle::default());
            // Create target, begin following it.
            let entity = commands
                .spawn((ThingToFollow, SpatialBundle::default()))
                .id();
            commands.ogle_change_mode(OgleMode::Frozen);
            commands.ogle_target_entity(entity);
        })
        .add_systems(Update, move_target)
        .add_systems(Update, control_camera_ui)
        .run();
}

fn setup_background(mut commands: Commands) {
    let n = 20;
    let spacing = 50.;
    let offset = spacing * n as f32 / 2.;
    let custom_size = Some(Vec2::new(spacing, spacing));
    for x in 0..n {
        for y in 0..n {
            let x = x as f32 * spacing - offset;
            let y = y as f32 * spacing - offset;
            let color = Color::hsl(240., random::<f32>() * 0.3, random::<f32>() * 0.3);
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size,
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 0.),
                ..default()
            });
        }
    }
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
    mut contexts: EguiContexts,
    query_thing: Query<Entity, With<ThingToFollow>>,
    mode: Res<State<OgleMode>>,
    mut next_mode: ResMut<NextState<OgleMode>>,
) {
    let window = egui::Window::new("Camera Controls")
        .anchor(egui::Align2::LEFT_TOP, [25.0, 25.0])
        .resizable(false)
        .title_bar(true);
    window.show(contexts.ctx_mut(), |ui| {
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
    });
}

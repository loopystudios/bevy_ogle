use bevy::{color::palettes::css, prelude::*};
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};
use bevy_ogle::{prelude::*, OgleBoundingSettings, OglePlugin};
use rand::random;

#[derive(Component)]
struct ThingToFollow;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(OglePlugin)
        .insert_resource(ClearColor(css::BLACK.into()))
        .add_systems(Startup, setup_scene)
        .add_systems(Update, move_target)
        .add_systems(EguiPrimaryContextPass, control_camera_ui)
        .run();
}

fn setup_scene(mut commands: Commands) {
    // Camera
    commands.spawn(OgleCam::new(
        OgleSettings {
            bounds: OgleBoundingSettings {
                enabled: true,
                min_x: -250.0,
                max_x: 250.0,
                min_y: -250.0,
                max_y: 250.0,
                min_scale: 0.5,
                max_scale: 2.5,
            },
            ..default()
        },
        Default::default(),
        Default::default(),
    ));

    // Background
    commands.spawn((
        Sprite {
            color: css::ORANGE.into(),
            custom_size: Some(Vec2::new(500.0, 500.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
    commands.spawn((
        Sprite {
            color: css::LIME.into(),
            custom_size: Some(Vec2::new(400.0, 400.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 2.0),
    ));

    // Moving thing for the camera to follow
    commands.spawn((
        ThingToFollow,
        Sprite {
            color: css::RED.into(),
            custom_size: Some(Vec2::new(5.0, 5.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 3.0),
    ));
}

fn move_target(
    time: Res<Time>,
    mut query_thing: Query<&mut Transform, With<ThingToFollow>>,
    mut gizmos: Gizmos,
) -> Result {
    let mut transform = query_thing.single_mut()?;
    transform.translation.x += time.delta_secs() * (random::<f32>() * 500.0 - 500.0 / 2.0);
    transform.translation.y += time.delta_secs() * (random::<f32>() * 500.0 - 500.0 / 2.0);
    gizmos.rect_2d(transform.translation.xy(), (5.0, 5.0).into(), css::RED);
    Ok(())
}

fn control_camera_ui(
    mut contexts: EguiContexts,
    thing: Single<Entity, With<ThingToFollow>>,
    mut cam: Single<&mut OgleCam>,
) -> Result {
    let window = egui::Window::new("Camera Controls")
        .anchor(egui::Align2::LEFT_TOP, [25.0, 25.0])
        .resizable(false)
        .title_bar(true);
    window.show(contexts.ctx_mut()?, |ui| {
        let cam = &mut cam;
        ui.heading("Bounds");
        ui.checkbox(&mut cam.settings.bounds.enabled, "Bounded");
        ui.heading("Mode");
        ui.radio_value(&mut cam.mode, OgleMode::Frozen, "Frozen");
        ui.radio_value(&mut cam.mode, OgleMode::ZoomOnly, "Zoom Only");
        ui.radio_value(&mut cam.mode, OgleMode::MoveOnly, "Move Only");
        ui.radio_value(&mut cam.mode, OgleMode::Normal, "Normal");
        ui.radio_value(&mut cam.mode, OgleMode::Pancam, "Pancam");
        ui.separator();
        ui.heading("Camera Target");
        let target_entity = *thing;
        if ui.radio(cam.target == OgleTarget::None, "None").clicked() {
            cam.target = OgleTarget::None;
        }
        if ui
            .radio(cam.target == OgleTarget::Entity(target_entity), "Entity")
            .clicked()
        {
            cam.target = OgleTarget::Entity(target_entity)
        }
        ui.horizontal(|ui| {
            let mut pos = match cam.target {
                OgleTarget::EntityWithOffset((_, p)) => p,
                _ => Vec2::new(0.0, 0.0),
            };
            if ui
                .radio(
                    matches!(cam.target, OgleTarget::EntityWithOffset(_)),
                    "Entity Offset",
                )
                .clicked()
            {
                cam.target = OgleTarget::EntityWithOffset((target_entity, pos));
            }
            ui.label("X");
            if ui.add(egui::DragValue::new(&mut pos.x)).changed() {
                cam.target = OgleTarget::EntityWithOffset((target_entity, pos));
            }
            ui.label("Y");
            if ui.add(egui::DragValue::new(&mut pos.y)).changed() {
                cam.target = OgleTarget::EntityWithOffset((target_entity, pos));
            }
        });
        ui.horizontal(|ui| {
            let mut pos = match cam.target {
                OgleTarget::Position(p) => p,
                _ => Vec2::new(0.0, 0.0),
            };
            if ui
                .radio(matches!(cam.target, OgleTarget::Position(_)), "Position")
                .clicked()
            {
                cam.target = OgleTarget::Position(pos);
            }
            ui.label("X");
            if ui.add(egui::DragValue::new(&mut pos.x)).changed() {
                cam.target = OgleTarget::Position(pos);
            }
            ui.label("Y");
            if ui.add(egui::DragValue::new(&mut pos.y)).changed() {
                cam.target = OgleTarget::Position(pos);
            }
        });
    });

    Ok(())
}

use std::time::Duration;

use bevy::{math::EulerRot, prelude::*, reflect::TypePath, window::{CursorGrabMode, PrimaryWindow}};
use bevy_tweening::{
    lens::{DirectionalLightIlluminanceLens, StandardMaterialBaseColorLens, TransformScaleLens},
    Animator, AssetAnimator, EaseMethod, RepeatCount, RepeatStrategy, Tween, TweeningPlugin,
};
use leafwing_input_manager::{
    prelude::{ActionState, DualAxis, InputManagerPlugin, InputMap},
    Actionlike, InputManagerBundle,
};
use smooth_bevy_cameras::{
    controllers::fps::{ControlEvent, FpsCameraBundle, FpsCameraController, FpsCameraPlugin},
    LookTransformPlugin,
};

#[derive(Component)]
struct Player;

fn player_move(
    mut events: EventWriter<ControlEvent>,
    action_states: Query<&ActionState<Action>>,
    controllers: Query<&FpsCameraController>,
) {
    // Can only control one camera at a time.
    let controller = if let Some(controller) = controllers.iter().find(|c| c.enabled) {
        controller
    } else {
        return;
    };
    let FpsCameraController {
        translate_sensitivity,
        ..
    } = *controller;

    let action_state = action_states.single();

    if action_state.pressed(Action::Forward) {
        events.send(ControlEvent::TranslateEye(translate_sensitivity * Vec3::Z));
    } else if action_state.pressed(Action::Left) {
        events.send(ControlEvent::TranslateEye(translate_sensitivity * Vec3::X));
    } else if action_state.pressed(Action::Backward) {
        events.send(ControlEvent::TranslateEye(translate_sensitivity * -Vec3::Z));
    } else if action_state.pressed(Action::Right) {
        events.send(ControlEvent::TranslateEye(translate_sensitivity * -Vec3::X));
    } else if action_state.pressed(Action::Down) {
        events.send(ControlEvent::TranslateEye(translate_sensitivity * -Vec3::Y));
    } else if action_state.pressed(Action::Up) {
        events.send(ControlEvent::TranslateEye(translate_sensitivity * Vec3::Y));
    }
}

fn player_look(
    mut events: EventWriter<ControlEvent>,
    action_states: Query<&ActionState<Action>>,
    controllers: Query<&FpsCameraController>,
) {
    // Can only control one camera at a time.
    let controller = if let Some(controller) = controllers.iter().find(|c| c.enabled) {
        controller
    } else {
        return;
    };
    let FpsCameraController {
        mouse_rotate_sensitivity,
        ..
    } = *controller;

    let mut cursor_delta = Vec2::ZERO;

    let action_state = action_states.single();

    if action_state.pressed(Action::Look) {
        cursor_delta += action_state
            .axis_pair(Action::Look)
            .map_or(Vec2::ZERO, |axis| Vec2::new(axis.x(), axis.y()));
        events.send(ControlEvent::Rotate(
            mouse_rotate_sensitivity * cursor_delta,
        ));
    }
}

#[derive(Component)]
struct Ripple {
    wave_movement: f32,
    wave_tiling: f32,
    wave_height: f32,
    wave_speed: f32,
    x: f32,
    y: f32,
    movement_behavior: MovementBehavior,
}

pub enum MovementBehavior {
    Undulating,
    Spiralling,
}

fn animate_ripplers(time: Res<Time>, mut query: Query<(&mut Transform, &mut Ripple)>) {
    let angle = std::f32::consts::PI / 2.0;
    let time_delta = time.delta_seconds();
    for (mut transform, mut rippler) in query.iter_mut() {
        rippler.wave_movement = (rippler.wave_movement + (rippler.wave_speed * time_delta))
            % (2.0 * std::f32::consts::PI);

        transform.translation.y = rippler.wave_height
            * (rippler.wave_movement + rippler.wave_tiling * (rippler.x + rippler.y)).sin();
        match rippler.movement_behavior {
            MovementBehavior::Spiralling => {
                transform.translation.x = transform.translation.x * (time_delta * angle).cos()
                    - transform.translation.y * (time_delta * angle).sin();
                transform.translation.z -= 0.01;
                transform.rotate(Quat::from_euler(
                    EulerRot::YXZ,
                    0.0,
                    0.0,
                    0.5f32.to_radians(),
                ));
            }
            _ => {
                let pitch = transform.translation.y;
                transform.rotate(Quat::from_euler(
                    EulerRot::XYZ,
                    (pitch % std::f32::consts::PI / 4.0).to_radians(),
                    0.0,
                    0.0,
                ));
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    commands
        .spawn(Camera3dBundle::default())
        .insert(FpsCameraBundle::new(
            FpsCameraController {
                translate_sensitivity: 12.0,
                ..Default::default()
            },
            Vec3::new(-2.0, 5.0, 5.0),
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        ))
        .insert(InputManagerBundle {
            action_state: ActionState::default(),
            input_map: InputMap::default()
                .insert(KeyCode::W, Action::Forward)
                .insert(KeyCode::A, Action::Left)
                .insert(KeyCode::S, Action::Backward)
                .insert(KeyCode::D, Action::Right)
                .insert(KeyCode::ShiftLeft, Action::Down)
                .insert(KeyCode::Space, Action::Up)
                .insert(DualAxis::mouse_motion(), Action::Look)
                .build(),
        })
        .insert(Player);

    commands.spawn(PbrBundle {
        transform: Transform::from_translation(Vec3::new(0.0, -8.0, 0.0)),
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 60.0,
            subdivisions: 1,
        })),
        material: materials.add(Color::hex("0047ab").unwrap().into()),
        ..Default::default()
    });

    commands.spawn((
        PbrBundle {
            transform: Transform::from_translation(Vec3::new(0.0, -8.0, 0.0)),
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::PINK.into()),
            ..Default::default()
        },
        Animator::new(
            Tween::new(
                EaseMethod::Linear,
                Duration::from_secs(10),
                TransformScaleLens {
                    start: Vec3::splat(1.0),
                    end: Vec3::splat(15.0),
                },
            )
            // Repeat twice (one per way)
            .with_repeat_count(RepeatCount::Finite(2))
            // After each iteration, reverse direction (ping-pong)
            .with_repeat_strategy(RepeatStrategy::MirroredRepeat),
        ),
    ));

    // Create a unique material per entity, so that it can be animated
    // without affecting the other entities. Note that we could share
    // that material among multiple entities, and animating the material
    // asset would change the color of all entities using that material.
    let unique_material = materials.add(Color::BLACK.into());

    for i in (0..=345usize).step_by(30) {
        let tween = Tween::new(
            EaseMethod::Linear,
            std::time::Duration::from_secs(2),
            StandardMaterialBaseColorLens {
                start: Color::RED,
                end: Color::YELLOW,
            },
        )
        // Repeat twice (one per way)
        .with_repeat_count(RepeatCount::Infinite)
        // After each iteration, reverse direction (ping-pong)
        .with_repeat_strategy(RepeatStrategy::MirroredRepeat);

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.5 })),
                material: unique_material.clone(),
                transform: {
                    let mut trans = Transform::from_translation(Vec3::new(
                        (i as f32).to_radians().cos() * 5.0,
                        (i as f32).to_radians().sin() * 5.0,
                        -5.0,
                    ));
                    trans.rotate(Quat::from_euler(
                        EulerRot::YXZ,
                        0.0,
                        0.0,
                        (i as f32).to_radians(),
                    ));
                    trans
                },
                ..Default::default()
            },
            AssetAnimator::new(unique_material.clone(), tween),
        ));
    }

    for i in (0..=345usize).step_by(30) {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.5 })),
            material: materials.add(Color::hex("041c56").unwrap().into()),
            transform: {
                let mut trans = Transform::from_translation(Vec3::new(
                    (i as f32).to_radians().cos() * 5.0,
                    (i as f32).to_radians().sin() * 5.0,
                    (i as f32).to_radians().cos() * 5.0 - 20.0,
                ));
                trans.rotate(Quat::from_euler(
                    EulerRot::YXZ,
                    0.0,
                    0.0,
                    (i as f32).to_radians(),
                ));
                trans
            },
            ..Default::default()
        });
    }

    for i in (0..=345usize).step_by(30) {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.5 })),
            material: materials.add(Color::hex("041c56").unwrap().into()),
            transform: {
                let mut trans = Transform::from_translation(Vec3::new(
                    (i as f32).to_radians().cos() * 5.0,
                    i as f32 / 15.0,
                    (i as f32).to_radians().sin() * 5.0,
                ));
                trans.rotate(Quat::from_euler(
                    EulerRot::YXZ,
                    0.0,
                    0.0,
                    (i as f32).to_radians(),
                ));
                trans
            },
            ..Default::default()
        });
    }

    for i in (0..=345usize).step_by(30) {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.5 })),
            material: materials.add(Color::hex("041c56").unwrap().into()),
            transform: Transform::from_translation(Vec3::new(
                (i as f32).to_radians().cos() * 10.0,
                i as f32 / 15.0,
                (i as f32).to_radians().sin() * 10.0,
            )),
            ..Default::default()
        });
    }

    for i in (0..=359usize).step_by(2) {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.25 })),
            material: materials.add(Color::hex("041c56").unwrap().into()),
            transform: {
                let mut trans = Transform::from_translation(Vec3::new(
                    (i as f32).to_radians().cos() * 15.0,
                    i as f32 / 7.5,
                    (i as f32).to_radians().sin() * 15.0,
                ));
                trans.rotate(Quat::from_euler(
                    EulerRot::YXZ,
                    0.0,
                    0.0,
                    (i as f32).to_radians(),
                ));
                trans
            },
            ..Default::default()
        });
    }

    for i in (0..=345usize).step_by(30) {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.5 })),
                material: materials.add(Color::hex("041c56").unwrap().into()),
                transform: {
                    let mut trans = Transform::from_translation(Vec3::new(
                        (i as f32).to_radians().cos() * 5.0,
                        (i as f32).to_radians().sin() * 5.0,
                        -(i as f32) / 15.0,
                    ));
                    trans.rotate(Quat::from_euler(
                        EulerRot::YXZ,
                        0.0,
                        0.0,
                        (i as f32).to_radians(),
                    ));
                    trans
                },
                ..Default::default()
            },
            Snakelike,
        ));
    }

    let std_material = materials.add(StandardMaterial::from(Color::FUCHSIA));

    commands
        .spawn(PbrBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .with_children(|parent| {
            for h in 0..20 {
                parent
                    .spawn(PbrBundle {
                        transform: Transform::from_xyz(0.0, 0.0, h as f32),
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
                        material: std_material.clone(),
                        ..Default::default()
                    })
                    .insert(Ripple {
                        wave_movement: 0.0,
                        wave_tiling: 10.0,
                        wave_height: 1.5,
                        wave_speed: 2.0,
                        x: 0.0,
                        y: h as f32 / 19.0,
                        movement_behavior: MovementBehavior::Undulating,
                    });
            }
        });

    commands.spawn((
        DirectionalLightBundle {
            transform: Transform::from_xyz(0.0, 1000.0, 0.0),
            directional_light: DirectionalLight {
                color: Color::hex("0047ab").unwrap(),
                shadows_enabled: true,
                ..Default::default()
            },
            ..Default::default()
        },
        Animator::new(
            Tween::new(
                EaseMethod::Linear,
                std::time::Duration::from_secs(3),
                DirectionalLightIlluminanceLens {
                    start: 0.0001,
                    end: 100000.0,
                },
            )
            // Repeat twice (one per way)
            .with_repeat_count(RepeatCount::Infinite)
            // After each iteration, reverse direction (ping-pong)
            .with_repeat_strategy(RepeatStrategy::MirroredRepeat),
        ),
    ));

    if let Ok(mut window) = primary_window.get_single_mut() {
        window.cursor.grab_mode = CursorGrabMode::Confined;
        window.cursor.visible = false;
    }
}

#[derive(Component)]
struct Snakelike;

fn snakelike_movement(time: Res<Time>, mut positions: Query<&mut Transform, With<Snakelike>>) {
    for mut transform in positions.iter_mut() {
        let angle = std::f32::consts::PI / 2.0;
        let time_delta = time.delta_seconds();
        transform.translation.x = transform.translation.x * (time_delta * angle).cos()
            - transform.translation.y * (time_delta * angle).sin();
        transform.translation.y = transform.translation.y * (time_delta * angle).cos()
            + transform.translation.x * (time_delta * angle).sin();
        transform.translation.z -= 0.01;
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, TypePath)]
enum Action {
    Forward,
    Left,
    Backward,
    Right,
    Down,
    Up,
    Look,
}

#[bevy_main]
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::AZURE))
        .insert_resource(Msaa::default())
        .add_plugins((
            DefaultPlugins,
            TweeningPlugin,
            InputManagerPlugin::<Action>::default(),
            LookTransformPlugin,
            FpsCameraPlugin::new(true),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (player_move, player_look, animate_ripplers, snakelike_movement))
        .run();
}

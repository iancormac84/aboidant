use std::time::Duration;

use bevy::{
    app::{App, Startup, Update},
    asset::Assets,
    color::{palettes::css, Color, Mix},
    math::{EulerRot, Quat, Vec2, Vec3},
    pbr::{DirectionalLight, MeshMaterial3d, StandardMaterial},
    prelude::{
        bevy_main, BuildChildren, Camera3d, ChildBuild, ClearColor, Commands, Component, Cuboid,
        EaseFunction, EventWriter, KeyCode, Mesh, Mesh3d, Plane3d, Query, Res, ResMut, Transform,
        With,
    },
    reflect::Reflect,
    time::Time,
    window::{CursorGrabMode, PrimaryWindow, Window},
    DefaultPlugins,
};
use bevy_tweening::{
    lens::TransformScaleLens,
    Animator, AssetAnimator, EaseMethod, Lens, RepeatCount, RepeatStrategy, Targetable, Tween,
    TweeningPlugin,
};
use leafwing_input_manager::{
    prelude::{ActionState, InputManagerPlugin, InputMap, MouseMove},
    Actionlike, InputManagerBundle,
};
use smooth_bevy_cameras::{
    controllers::fps::{ControlEvent, FpsCameraBundle, FpsCameraController, FpsCameraPlugin},
    LookTransformPlugin,
};

struct StandardMaterialBaseColorLens {
    start: Color,
    end: Color,
}

impl Lens<StandardMaterial> for StandardMaterialBaseColorLens {
    fn lerp(&mut self, target: &mut dyn Targetable<StandardMaterial>, ratio: f32) {
        let value = self.start.mix(&self.end, ratio);
        target.base_color = value;
    }
}

struct DirectionalLightIlluminanceLens {
    start: f32,
    end: f32,
}

impl Lens<DirectionalLight> for DirectionalLightIlluminanceLens {
    fn lerp(&mut self, target: &mut dyn Targetable<DirectionalLight>, ratio: f32) {
        target.illuminance = self.start + (self.end - self.start) * ratio;
    }
}

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

    if action_state.pressed(&Action::Forward) {
        events.send(ControlEvent::TranslateEye(translate_sensitivity * Vec3::Z));
    } else if action_state.pressed(&Action::Left) {
        events.send(ControlEvent::TranslateEye(translate_sensitivity * Vec3::X));
    } else if action_state.pressed(&Action::Backward) {
        events.send(ControlEvent::TranslateEye(translate_sensitivity * -Vec3::Z));
    } else if action_state.pressed(&Action::Right) {
        events.send(ControlEvent::TranslateEye(translate_sensitivity * -Vec3::X));
    } else if action_state.pressed(&Action::Down) {
        events.send(ControlEvent::TranslateEye(translate_sensitivity * -Vec3::Y));
    } else if action_state.pressed(&Action::Up) {
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

    if action_state.pressed(&Action::Look) {
        cursor_delta += action_state
            .axis_pair(&Action::Look);
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
    let time_delta = time.delta_secs();
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
    let input_map = {
        let mut input_map = InputMap::default();
        input_map.insert(Action::Forward, KeyCode::KeyW);
        input_map.insert(Action::Left, KeyCode::KeyA);
        input_map.insert(Action::Backward, KeyCode::KeyS);
        input_map.insert(Action::Right, KeyCode::KeyD);
        input_map.insert(Action::Down, KeyCode::ShiftLeft);
        input_map.insert(Action::Up, KeyCode::Space);
        input_map.insert_dual_axis(Action::Look, MouseMove::default());
        input_map
    };
    commands
        .spawn(Camera3d::default())
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
            input_map,
        })
        .insert(Player);

    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, -8.0, 0.0)),
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(30.0)))),
        MeshMaterial3d(materials.add(Color::srgb_u8(0, 71, 171))),
    ));

    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, -8.0, 0.0)),
        Mesh3d(meshes.add(Cuboid::from_size(Vec3::splat(1.0)))),
        MeshMaterial3d(materials.add(Color::from(css::PINK))),
        Animator::new(
            Tween::new(
                EaseMethod::EaseFunction(EaseFunction::Linear),
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
    let unique_material = materials.add(Color::BLACK);

    for i in (0..=345usize).step_by(30) {
        let tween = Tween::new(
            EaseMethod::EaseFunction(EaseFunction::Linear),
            std::time::Duration::from_secs(2),
            StandardMaterialBaseColorLens {
                start: css::RED.into(),
                end: css::YELLOW.into(),
            },
        )
        // Repeat twice (one per way)
        .with_repeat_count(RepeatCount::Infinite)
        // After each iteration, reverse direction (ping-pong)
        .with_repeat_strategy(RepeatStrategy::MirroredRepeat);

        commands.spawn((
            Mesh3d(meshes.add(Cuboid::from_size(Vec3::splat(1.5)))),
            MeshMaterial3d(unique_material.clone()),
            {
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
            AssetAnimator::new(tween),
        ));
    }

    for i in (0..=345usize).step_by(30) {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::from_size(Vec3::splat(1.5)))),
            MeshMaterial3d(materials.add(Color::srgb_u8(4, 28, 86))),
            {
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
        ));
    }

    for i in (0..=345usize).step_by(30) {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::from_size(Vec3::splat(1.5)))),
            MeshMaterial3d(materials.add(Color::srgb_u8(4, 28, 86))),
            {
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
        ));
    }

    for i in (0..=345usize).step_by(30) {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::from_size(Vec3::splat(1.5)))),
            MeshMaterial3d(materials.add(Color::srgb_u8(4, 28, 86))),
            Transform::from_translation(Vec3::new(
                (i as f32).to_radians().cos() * 10.0,
                i as f32 / 15.0,
                (i as f32).to_radians().sin() * 10.0,
            )),
        ));
    }

    for i in (0..=359usize).step_by(2) {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::from_size(Vec3::splat(0.25)))),
            MeshMaterial3d(materials.add(Color::srgb_u8(4, 28, 86))),
            {
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
        ));
    }

    for i in (0..=345usize).step_by(30) {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::from_size(Vec3::splat(1.5)))),
            MeshMaterial3d(materials.add(Color::srgb_u8(4, 28, 86))),
            {
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
            Snakelike,
        ));
    }

    let std_material = materials.add(StandardMaterial::from(Color::from(css::FUCHSIA)));

    commands
        .spawn(
            Transform::from_xyz(0.0, 0.0, 0.0),
        )
        .with_children(|parent| {
            for h in 0..20 {
                parent
                    .spawn((
                        Transform::from_xyz(0.0, 0.0, h as f32),
                        Mesh3d(meshes.add(Cuboid::from_size(Vec3::splat(0.5)))),
                        MeshMaterial3d(std_material.clone()),
                    ))
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
        Transform::from_xyz(0.0, 1000.0, 0.0),
        DirectionalLight {
            color: Color::srgb_u8(0, 71, 171),
            shadows_enabled: true,
            ..Default::default()
        },
        Animator::new(
            Tween::new(
                EaseMethod::EaseFunction(EaseFunction::Linear),
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
        window.cursor_options.grab_mode = CursorGrabMode::Confined;
        window.cursor_options.visible = false;
    }
}

#[derive(Component)]
struct Snakelike;

fn snakelike_movement(time: Res<Time>, mut positions: Query<&mut Transform, With<Snakelike>>) {
    for mut transform in positions.iter_mut() {
        let angle = std::f32::consts::PI / 2.0;
        let time_delta = time.delta_secs();
        transform.translation.x = transform.translation.x * (time_delta * angle).cos()
            - transform.translation.y * (time_delta * angle).sin();
        transform.translation.y = transform.translation.y * (time_delta * angle).cos()
            + transform.translation.x * (time_delta * angle).sin();
        transform.translation.z -= 0.01;
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum Action {
    Forward,
    Left,
    Backward,
    Right,
    Down,
    Up,
    #[actionlike(DualAxis)]
    Look,
}

#[bevy_main]
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::Srgba(css::AZURE)))
        .add_plugins((
            DefaultPlugins,
            TweeningPlugin,
            InputManagerPlugin::<Action>::default(),
            LookTransformPlugin,
            FpsCameraPlugin::new(true),
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                player_move,
                player_look,
                animate_ripplers,
                snakelike_movement,
            ),
        )
        .run();
}

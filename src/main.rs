use std::time::Duration;

use bevy::math::EulerRot;
use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;
use bevy_tweening::{
    lens::{DirectionalLightIlluminanceLens, StandardMaterialBaseColorLens, TransformScaleLens},
    Animator, AssetAnimator, EaseMethod, RepeatCount, RepeatStrategy, Tween, TweeningPlugin,
};

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
                transform.translation.x = transform.translation.x
                    * (time_delta * angle).cos() as f32
                    - transform.translation.y * (time_delta * angle).sin() as f32;
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
) {
    commands.spawn(PbrBundle {
        transform: Transform::from_translation(Vec3::new(0.0, -8.0, 0.0)),
        mesh: meshes.add(Mesh::from(shape::Plane { size: 60.0 })),
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
}

#[derive(Component)]
struct Snakelike;

fn snakelike_movement(time: Res<Time>, mut positions: Query<&mut Transform, With<Snakelike>>) {
    for mut transform in positions.iter_mut() {
        let angle = std::f32::consts::PI / 2.0;
        let time_delta = time.delta_seconds();
        transform.translation.x = transform.translation.x * (time_delta * angle).cos() as f32
            - transform.translation.y * (time_delta * angle).sin() as f32;
        transform.translation.y = transform.translation.y * (time_delta * angle).cos() as f32
            + transform.translation.x * (time_delta * angle).sin() as f32;
        transform.translation.z -= 0.01;
    }
}

#[bevy_main]
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::AZURE))
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(TweeningPlugin)
        .add_startup_system(setup)
        .add_system(animate_ripplers)
        .add_system(snakelike_movement)
        .run();
}

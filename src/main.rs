use bevy::math::EulerRot;
use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;

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
    for i in (0..=345usize).step_by(30) {
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.5 })),
            material: materials.add(Color::hex("041c56").unwrap().into()),
            transform: {
                let mut trans = Transform::from_translation(Vec3::new(
                    (i as f32).to_radians().cos() * 5.0,
                    (i as f32).to_radians().sin() * 5.0,
                    -5.0,
                ));
                trans.rotate(Quat::from_euler(EulerRot::YXZ, 0.0, 0.0, (i as f32).to_radians()));
                trans
            },
            ..Default::default()
        });
    }

    for i in (0..=345usize).step_by(30) {
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.5 })),
            material: materials.add(Color::hex("041c56").unwrap().into()),
            transform: {
                let mut trans = Transform::from_translation(Vec3::new(
                    (i as f32).to_radians().cos() * 5.0,
                    (i as f32).to_radians().sin() * 5.0,
                    (i as f32).to_radians().cos() * 5.0 - 20.0,
                ));
                trans.rotate(Quat::from_euler(EulerRot::YXZ, 0.0, 0.0, (i as f32).to_radians()));
                trans
            },
            ..Default::default()
        });
    }

    for i in (0..=345usize).step_by(30) {
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.5 })),
            material: materials.add(Color::hex("041c56").unwrap().into()),
            transform: {
                let mut trans = Transform::from_translation(Vec3::new(
                    (i as f32).to_radians().cos() * 5.0,
                    i as f32 / 15.0,
                    (i as f32).to_radians().sin() * 5.0,
                ));
                trans.rotate(Quat::from_euler(EulerRot::YXZ, 0.0, 0.0, (i as f32).to_radians()));
                trans
            },
            ..Default::default()
        });
    }

    for i in (0..=345usize).step_by(30) {
        commands.spawn_bundle(PbrBundle {
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
        commands.spawn_bundle(PbrBundle {
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
        commands
            .spawn_bundle(PbrBundle {
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
            })
            .insert(Snakelike);
    }

    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .with_children(|parent| {
            for h in 0..20 {
                parent
                    .spawn_bundle(PbrBundle {
                        transform: Transform::from_xyz(0.0, 0.0, h as f32),
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
                        material: materials.add(StandardMaterial {
                            base_color: Color::FUCHSIA,
                            metallic: 1.0,
                            roughness: 0.0,
                            ..Default::default()
                        }),
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

    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 20.0)),
        ..Default::default()
    });
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 10.0, 20.0)),
        ..Default::default()
    });
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
        .insert_resource(ClearColor(Color::YELLOW_GREEN))
        .insert_resource(Msaa { samples: 4 })
        .add_startup_system(setup)
        .add_system(animate_ripplers)
        .add_system(snakelike_movement)
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .run();
}

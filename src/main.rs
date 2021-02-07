use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;

mod shapes;

fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    /*for i in (0..=345usize).step_by(30) {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.5 })),
            material: materials.add(Color::hex("041c56").unwrap().into()),
            transform: {
                let mut trans = Transform::from_translation(Vec3::new(
                    (i as f32).to_radians().cos() * 5.0,
                    (i as f32).to_radians().sin() * 5.0,
                    -5.0,
                ));
                trans.rotate(Quat::from_rotation_ypr(0.0, 0.0, (i as f32).to_radians()));
                println!("trans.rotation is {}", trans.rotation);
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
                    (i as f32).to_radians().sin() * 5.0,
                    (i as f32).to_radians().cos() * 5.0 - 20.0,
                ));
                trans.rotate(Quat::from_rotation_ypr(0.0, 0.0, (i as f32).to_radians()));
                println!("trans.rotation is {}", trans.rotation);
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
                trans.rotate(Quat::from_rotation_ypr(0.0, 0.0, (i as f32).to_radians()));
                println!("trans.rotation is {}", trans.rotation);
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
    }*/

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
                trans.rotate(Quat::from_rotation_ypr(0.0, 0.0, (i as f32).to_radians()));
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
                    (i as f32).to_radians().sin() * 5.0,
                    -(i as f32) / 15.0,
                ));
                trans.rotate(Quat::from_rotation_ypr(0.0, 0.0, (i as f32).to_radians()));
                trans
            },
            ..Default::default()
        }).with(Snakelike);
    }

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 60.0 })),
            material: materials.add(Color::hex("7ed957").unwrap().into()),
            ..Default::default()
        })
        .spawn(LightBundle {
            light: Light {
                fov: 200.0,
                depth: 1.0..1000.0,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 10.0, 0.0)),
            ..Default::default()
        });
}

struct Snakelike;

fn snakelike_movement(
    time: Res<Time>,
    mut positions: Query<&mut Transform, With<Snakelike>>,
) {
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

/*struct Rotator;

fn rotator_movement(time: Res<Time>, mut rotator_positions: Query<&mut Transform, With<Rotator>>) {
    let angle = std::f32::consts::PI;
    for mut transform in rotator_positions.iter_mut() {
        let time_delta = time.delta_seconds();
        transform.translation.x = transform.translation.x * (time_delta * angle).cos() as f32
            - transform.translation.y * (time_delta * angle).sin() as f32;
        transform.translation.y = transform.translation.y * (time_delta * angle).cos() as f32
            + transform.translation.x * (time_delta * angle).sin() as f32;
        transform.translation.z -= 0.01;
    }
}*/

#[bevy_main]
fn main() {
    App::build()
        .insert_resource(ClearColor(Color::MIDNIGHT_BLUE))
        .insert_resource(Msaa { samples: 4 })
        .add_startup_system(setup.system())
        .add_system(snakelike_movement.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .run();
}

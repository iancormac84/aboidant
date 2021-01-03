use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;

mod boid;

fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
        })
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.5 })),
            material: materials.add(Color::hex("041c56").unwrap().into()),
            transform: Transform::from_translation(Vec3::new(0.0, 15.0, 0.0)),
            ..Default::default()
        })
        .with(Rotator)
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::hex("38b6ff").unwrap().into()),
            transform: Transform::from_translation(Vec3::new(0.0, 10.0, 10.0)),
            ..Default::default()
        })
        .with(Spinner)
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
            material: materials.add(Color::hex("ffb6ff").unwrap().into()),
            transform: Transform::from_translation(Vec3::new(0.0, 25.0, 0.0)),
            ..Default::default()
        })
        .with(Rotator2);
}

struct Rotator2;

fn rotator2_movement(
    time: Res<Time>,
    mut rotator2_positions: Query<&mut Transform, With<Rotator2>>,
) {
    for mut transform in rotator2_positions.iter_mut() {
        let time_delta = time.seconds_since_startup();
        println!("time_delta is {}", time_delta);
        transform.translation.x =
            ((5.0 * time_delta).sin() as f32 * 5.0) + (time_delta.sin() as f32 * 20.0);
        transform.translation.y = (5.0 * time_delta).cos() as f32 * 5.0;
        transform.translation.z = time_delta.cos() as f32 * 20.0;
        let lookat = transform.translation;
        transform.look_at(lookat, Vec3::new(0.0, 1.0, 0.0));
        println!("transform is {:?}", transform);
    }
}

struct Spinner;

fn spinner_movement(time: Res<Time>, mut spinner_positions: Query<&mut Transform, With<Spinner>>) {
    let angle = std::f32::consts::PI / 4.0;
    for mut transform in spinner_positions.iter_mut() {
        transform.translation.x = time.seconds_since_startup().sin() as f32 * 15.0;
        transform.translation.z = time.seconds_since_startup().cos() as f32 * 15.0;
        transform.rotate(Quat::from_axis_angle(
            Vec3::new(0.33, 0.33, 0.33),
            angle * time.delta_seconds(),
        ));
    }
}

struct Rotator;

fn rotator_movement(time: Res<Time>, mut rotator_positions: Query<&mut Transform, With<Rotator>>) {
    let angle = std::f32::consts::PI / 4.0;
    for mut transform in rotator_positions.iter_mut() {
        transform.translation.x = transform.translation.x
            * (time.delta_seconds() * angle).cos() as f32
            - transform.translation.y * (time.delta_seconds() * angle).sin() as f32;
        transform.translation.y = transform.translation.y
            * (time.delta_seconds() * angle).cos() as f32
            + transform.translation.x * (time.delta_seconds() * angle).sin() as f32;
    }
}

#[bevy_main]
fn main() {
    App::build()
        .add_resource(ClearColor(Color::MIDNIGHT_BLUE))
        .add_resource(Msaa { samples: 4 })
        .add_startup_system(setup.system())
        .add_system(rotator_movement.system())
        .add_system(rotator2_movement.system())
        .add_system(spinner_movement.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .run();
}

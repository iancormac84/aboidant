use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;

struct Ripple {
    wave_movement: f32,
    wave_tiling: f32,
    wave_height: f32,
    wave_speed: f32,
    x: f32,
    y: f32,
}

fn animate_ripplers(time: Res<Time>, mut query: Query<(&mut Transform, &mut Ripple)>) {
    let angle = std::f32::consts::PI / 2.0;
    let time_delta = time.delta_seconds();
    for (mut transform, mut rippler) in query.iter_mut() {
        rippler.wave_movement = (rippler.wave_movement
            + (rippler.wave_speed * time_delta))
            % (2.0 * std::f32::consts::PI);

        transform.translation.x = transform.translation.x * (time_delta * angle).cos() as f32
            - transform.translation.y * (time_delta * angle).sin() as f32;
        transform.translation.y = rippler.wave_height
            * (rippler.wave_movement + rippler.wave_tiling * (rippler.x + rippler.y)).sin();
        transform.translation.z -= 0.01;
        transform.rotate(Quat::from_rotation_ypr(0.0, 0.0, 0.5f32.to_radians()));
        /*let new_yaw = transform.translation.y / 300.0;
        let newz = (rippler.wave_movement + (transform.translation.z / 19.0)) * time.delta_seconds() % std::f32::consts::PI;
        transform.rotate(Quat::from_rotation_ypr(0.0, 0.0, newz));*/
    }
}

fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .with_children(|parent| {
            for h in 0..20 {
                parent
                    .spawn(PbrBundle {
                        transform: {
                            let mut trans = Transform::from_xyz(0.0, 0.0, h as f32);
                            trans.rotate(Quat::from_rotation_ypr(
                                0.0,
                                0.0,
                                (h as f32 * 360.0 / 20.0).to_radians(),
                            ));
                            trans
                        },
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                        material: materials.add(Color::hex("7ed957").unwrap().into()),
                        ..Default::default()
                    })
                    .with(Ripple {
                        wave_movement: 0.0,
                        wave_tiling: 10.0,
                        wave_height: 1.5,
                        wave_speed: 2.0,
                        x: 0.0,
                        y: h as f32 / 19.0,
                    });
            }
        });

    commands.spawn(LightBundle {
        light: Light {
            fov: 200.0,
            depth: 1.0..1000.0,
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 10.0, 0.0)),
        ..Default::default()
    });
}

#[bevy_main]
fn main() {
    App::build()
        .insert_resource(ClearColor(Color::MIDNIGHT_BLUE))
        .insert_resource(Msaa { samples: 4 })
        .add_startup_system(setup.system())
        .add_system(animate_ripplers.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .run();
}

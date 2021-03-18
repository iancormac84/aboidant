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
    for (mut transform, mut rippler) in query.iter_mut() {
        rippler.wave_movement = (rippler.wave_movement + (rippler.wave_speed * time.delta_seconds()))
            % (2.0 * std::f32::consts::PI);

        transform.translation.y = rippler.wave_height
            * (rippler.wave_movement + rippler.wave_tiling * (rippler.x + rippler.y)).sin();
    }
}

fn setup(
    mut commands: Commands,
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
                        transform: Transform::from_xyz(0.0, 0.0, h as f32),
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                        material: materials.add(StandardMaterial {
                            base_color: Color::FUCHSIA,
                            metallic: 1.0,
                            roughness: 0.0,
                            ..Default::default()
                        }),
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
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 20.0)),
        ..Default::default()
    }).spawn(LightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 10.0, 20.0)),
        ..Default::default()
    });
}

#[bevy_main]
fn main() {
    App::build()
        .insert_resource(ClearColor(Color::YELLOW_GREEN))
        .insert_resource(Msaa { samples: 4 })
        .add_startup_system(setup.system())
        .add_system(animate_ripplers.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .run();
}

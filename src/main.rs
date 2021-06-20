use bevy::prelude::*;

fn main() {
    App::build()
    // Set antialiasing to use 4 samples
    .add_resource(Msaa{samples: 4})
    // Set WindowDescriptor Resource to change title and size
    .add_resource(WindowDescriptor{
        title: "SETHS CHESS GAME".to_string(),
        width: 1600., 
        height: 1600.,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup.system())
    .add_startup_system(create_board.system())
    .run(); 
}

// start up system
fn setup(commands: &mut Commands) {
    commands
    // this block is for the camera
        .spawn(Camera3dBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
                Vec3::new(-7.0, 20.0, 4.0),
            )),
            ..Default::default()
        })
        // this block is for the light source
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });
}


fn create_board (
    // these are all perameters. they just look odd right now.
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // expression starts here
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 1. }));
    let white_material = materials.add(Color::rgb(1., 0.9, 0.9).into());
    let black_material = materials.add(Color::rgb(0., 0.1, 0.1).into());
    
    // for i in range(1 - 8) nested four loop. makes 64 squares.
    for i in 0..8 {
        for j in 0..8{
            commands.spawn(PbrBundle {
                mesh: mesh.clone(),
                // if it's even, then make white_material, else make black_material
                material: if (i + j + 1) % 2 == 0 {
                    white_material.clone()
                } else {
                    black_material.clone()
                },
                transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
                ..Default::default()
            });
        }
    }
}
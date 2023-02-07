use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod tower;
use tower::*;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Lifetime {
    timer: Timer,
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(Name::new("Camera"));
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.9, 0.25, 0.2).into()),
            ..default()
        })
        .insert(Name::new("Ground"));
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.2, 0.2, 0.2).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(Name::new("Cube"));
}

fn spawn_light(mut commands: Commands) {
    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(2.0, 3.0, 2.0),
            ..default()
        })
        .insert(Name::new("Point Light"));
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        //Inspector Setup
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WIDTH,
                height: HEIGHT,
                title: "Bevy Tower Defense".to_string(),
                resizable: false,
                ..Default::default()
            },
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin)
        .register_type::<Tower>()
        // Our Systems
        .add_startup_system(spawn_basic_scene)
        .add_startup_system(spawn_light)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_tower)
        //
        .add_system(tower_shooting)
        .add_system(bullet_despawn)
        //
        .run();
}

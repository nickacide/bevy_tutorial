use bevy::prelude::*;

pub fn camera_controls(
    keyboard: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let mut camera = camera_query.single_mut();

    let mut forward = camera.forward();
    forward.y = 0.0;
    forward = forward.normalize();

    let mut left = camera.left();
    left.y = 0.0;
    left = left.normalize();

    let mut top = camera.up();
    top.x = 0.0;
    top.z = 0.0;
    top = top.normalize();

    // let mut look_up = camera.forward();
    // look_up.x = 0.0;
    // look_up.z = 0.0;
    // look_up = look_up.normalize();

    let speed = 3.0;
    let rotation_speed = 1.5;

    if keyboard.pressed(KeyCode::W) {
        camera.translation += forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::S) {
        camera.translation -= forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::A) {
        camera.translation += left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::D) {
        camera.translation -= left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::Left) {
        camera.rotate_axis(Vec3::Y, rotation_speed * time.delta_seconds());
    }
    if keyboard.pressed(KeyCode::Right) {
        camera.rotate_axis(Vec3::Y, -rotation_speed * time.delta_seconds());
    }
    if keyboard.pressed(KeyCode::Space) {
        camera.translation += top * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::LShift) {
        camera.translation -= top * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::Up) {
        camera.rotate_local_axis(Vec3::X, rotation_speed);
    }
    if keyboard.pressed(KeyCode::Down) {
        camera.rotate_local_axis(Vec3::X, -rotation_speed);
    }
}

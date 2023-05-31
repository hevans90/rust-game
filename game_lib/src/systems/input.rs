use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

use crate::components::{main_camera::MainCamera, mover::Mover};

fn radian_to_vector2(radian: f32) -> Vec2 {
    return Vec2::from_angle(radian).perp();
}

pub fn scroll_events(
    mut scroll_evr: EventReader<MouseWheel>,
    mut q: Query<&mut OrthographicProjection, With<MainCamera>>,
) {
    let mut projection = q.single_mut();

    // always ensure you end up with sane values
    // (pick an upper and lower bound for your application)

    for ev in scroll_evr.iter() {
        match ev.unit {
            MouseScrollUnit::Line => {
                println!(
                    "Scroll (line units): vertical: {}, horizontal: {}",
                    ev.y, ev.x
                );
            }
            MouseScrollUnit::Pixel => {
                println!(
                    "Scroll (pixel units): vertical: {}, horizontal: {}",
                    ev.y, ev.x
                );
                let min = 0.5;
                let max = 5.0;

                projection.scale += ev.y * 0.001;

                projection.scale = projection.scale.clamp(min, max);
            }
        }
    }
}

pub fn move_entities(keys: Res<Input<KeyCode>>, mut entities: Query<&mut Transform, With<Mover>>) {
    if keys.pressed(KeyCode::W) {
        // W is being held down

        for mut transform in entities.iter_mut() {
            let vec2 = radian_to_vector2(transform.rotation.to_euler(EulerRot::XYZ).2) * 2.0;

            transform.translation = Vec3::new(
                transform.translation.x + vec2.x,
                transform.translation.y + vec2.y,
                transform.translation.z,
            )
        }
    }
    if keys.pressed(KeyCode::S) {
        // S is being held down

        for mut transform in entities.iter_mut() {
            // transform.translation.x += 10.0;

            let vec2 = radian_to_vector2(transform.rotation.to_euler(EulerRot::XYZ).2) * 2.0;

            transform.translation = Vec3::new(
                transform.translation.x - vec2.x,
                transform.translation.y - vec2.y,
                transform.translation.z,
            )
        }
    }
    if keys.pressed(KeyCode::A) {
        // A is being held down

        for mut transform in entities.iter_mut() {
            let (x, y, z) = transform.rotation.to_euler(EulerRot::XYZ);

            transform.rotation = Quat::from_euler(EulerRot::XYZ, x, y, z + 0.1);
        }
    }
    if keys.pressed(KeyCode::D) {
        // D is being held down

        for mut transform in entities.iter_mut() {
            let (x, y, z) = transform.rotation.to_euler(EulerRot::XYZ);

            transform.rotation = Quat::from_euler(EulerRot::XYZ, x, y, z - 0.1);
        }
    }
}

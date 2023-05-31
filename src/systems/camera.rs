use bevy::prelude::*;

use crate::components::{main_camera::MainCamera, mover::Mover};

pub fn camera_follow(
    mut cameras: Query<&mut Transform, (With<MainCamera>, Without<Mover>)>,
    entities: Query<&mut Transform, With<Mover>>,
) {
    let mut camera_transform = cameras.single_mut();

    let reduction = entities
        .iter()
        .map(|a| a.translation)
        .reduce(|a, b| a + b)
        .unwrap();

    camera_transform.translation = reduction / entities.iter().len() as f32;
}

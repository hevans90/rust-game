use bevy::{prelude::*, sprite::MaterialMesh2dBundle, DefaultPlugins};
use components::{main_camera::MainCamera, mover::Mover};
use systems::{
    camera::camera_follow,
    input::{move_entities, scroll_events},
};

use wasm_bindgen::prelude::wasm_bindgen;

mod components {
    pub mod main_camera;
    pub mod mover;
}
mod systems {
    pub mod camera;
    pub mod input;
}

#[wasm_bindgen]
pub fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Noob Game".to_string(),
                #[cfg(target_arch = "wasm32")]
                canvas: Some(String::from(".game")),
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup)
        .add_system(camera_follow)
        .add_system(move_entities)
        .add_system(scroll_events)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default()).insert(MainCamera);

    // Circle
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
        ..default()
    });

    // Rectangle
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(50.0, 100.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
            ..default()
        })
        .insert(Mover);

    // Quad
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(50., 100.)).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::rgb(1., 0.9, 1.1))),
            transform: Transform::from_translation(Vec3::new(70.0, 0., 0.)),
            ..default()
        })
        .insert(Mover);

    // Hexagon
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
            material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
            transform: Transform::from_translation(Vec3::new(150., 0., 0.)),
            ..default()
        })
        .insert(Mover);
}

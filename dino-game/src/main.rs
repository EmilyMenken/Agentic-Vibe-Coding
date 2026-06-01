use bevy::prelude::*;
use bevy::render::settings::{RenderCreation, WgpuSettings, Backends};
use bevy::render::RenderPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Dino Quiz!".into(),
                    resolution: (800., 600.).into(),
                    ..default()
                }),
                ..default()
            })
            .set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    backends: Some(Backends::GL),
                    ..default()
                }),
                ..default()
            })
        )
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite {
            image: asset_server.load("images/Ankylosaurus.png"),
            custom_size: Some(Vec2::new(400., 350.)),
            ..default()
        },
        Transform::from_xyz(0., 0., 1.),
    ));
}
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Dino Quiz!".into(),
                resolution: (800., 600.).into(),
                ..default()
            }),
            ..default()
        }))
        .run();
}
use bevy::{input::InputPlugin, prelude::*, window::WindowMode};
mod charactor;
mod ddd;
mod employ;
mod job;
mod story;
fn main() {
    App::new()
        // .add_plugins(DefaultPlugins.set(WindowPlugin {
        //     primary_window: Some(Window {
        //         mode: WindowMode::BorderlessFullscreen,
        //         ..default()
        //     }),
        //     ..default()
        // }))
        // .add_plugins(InputPlugin::default())
        // .add_systems(Startup, ddd::setup)
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, story::scene1::setup)
        .add_systems(Update, charactor::sys::who_ready)
        .add_systems(Update, job::sys::peopel_with_job)
        .run();
}

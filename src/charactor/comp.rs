use bevy::prelude::Component;

#[derive(Component)]
pub struct Person {
    pub name: String,
}

#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

use bevy::prelude::{Commands, Query, Transform};

use super::comp::Person;

pub fn print_position(query: Query<&Transform>) {
    for transform in &query {
        println!("position: {:?}", transform.translation);
    }
}

pub fn who_ready(query: Query<&Person>) {
    for person in query.iter() {
        println!("{} is ready", person.name);
    }
}

use bevy::prelude::Commands;

use crate::{charactor::comp::Person, job::comp::Job};

pub fn setup(mut command: Commands) {
    command.spawn(Person {
        name: "Alice".to_string(),
    });
    command.spawn(Person {
        name: "Weykon".to_string(),
    });
    command.spawn((
        Person {
            name: "Norci".to_string(),
        },
        Job {
            name: "Programmer".to_string(),
        },
    ));
}

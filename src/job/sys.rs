use bevy::prelude::{Query, With};

use crate::charactor::comp::Person;

use super::comp::Job;

pub fn peopel_with_job(person_query: Query<&Person, With<Job>>) {
    for person in person_query.iter() {
        println!("{} have job", person.name);
    }
}

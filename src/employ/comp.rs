use bevy::prelude::Component;

use crate::job::comp::Job;

#[derive(Component)]
pub struct Employ {
    pub name: String,
    pub job: Job,
}

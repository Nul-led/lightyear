use bevy::prelude::ResMut;

use crate::shared::tick_manager::TickManaged;

pub fn increment_tick<T: TickManaged>(mut service: ResMut<T>) {
    service.increment_tick();
}
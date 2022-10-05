use bevy::prelude::*;

pub struct AmmoPlugin;

impl Plugin for AmmoPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_ammo);
    }
}

// Ammo spawns from dead enemies
fn spawn_ammo() {}

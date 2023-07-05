use std::iter::Map;

use crate::{backend, constants};
use bevy::prelude::*;

#[derive(Default)]
pub struct SoccerControlPlugin;

impl Plugin for SoccerControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_keyboard_control.in_base_set(StartupSet::PostStartup));
        app.add_system(keyboard_to_dynamics);
    }
}

#[derive(Component)]
struct KeyboardControlled;

fn setup_keyboard_control(mut commands: Commands, players: Query<Entity, With<backend::Player>>) {
    for player in &players {
        commands
            .get_entity(player)
            .unwrap()
            .insert(KeyboardControlled);
    }
}

fn keyboard_to_dynamics(
    mut query: Query<(&KeyboardControlled, &mut backend::Dynamics)>,
    keys: Res<Input<KeyCode>>,
) {
    for (_, mut dynamics) in &mut query {
        let mut direction = Vec2::new(0.0, 0.0);

        let mut key_direction_map = std::collections::HashMap::from([
            ([KeyCode::Right, KeyCode::D], Vec2::X),
            ([KeyCode::Up, KeyCode::W], Vec2::Y),
            ([KeyCode::Left, KeyCode::A], Vec2::NEG_X),
            ([KeyCode::Down, KeyCode::S], Vec2::NEG_Y),
        ]);

        for (&k, &v) in key_direction_map.iter() {
            if keys.any_pressed(k) {
                direction += v;
            }
        }

        dynamics.apply_impulse(direction.normalize_or_zero() * constants::IMPULSE_PER_KEYPRESS);
    }
}

use std::time::Duration;

use bevy::{ecs::schedule::ScheduleLabel, prelude::*};

use crate::constants;

impl SoccerBackendPlugin {}
impl Plugin for SoccerBackendPlugin {
    fn build(&self, app: &mut App) {
        // app.configure_sets(
        //     (
        //         StartupSet::StartupFlush,
        //         SoccerBackendStartupSet::Parallel,
        //         SoccerBackendStartupSet::CommandFlush,
        //         StartupSet::PostStartup,
        //     )
        //         .chain(),
        // );
        // app.add_system(apply_system_buffers.in_set(SoccerBackendStartupSet::CommandFlush));
        // app.add_startup_system(create_players.in_set(SoccerBackendStartupSet::Parallel));
        app.add_startup_system(create_players.in_base_set(StartupSet::Startup));
        app.add_system(resolve_dynamics.in_schedule(CoreSchedule::FixedUpdate));
    }
}

#[derive(Component, Debug, Copy, Clone, PartialEq)]
pub struct Dynamics {
    velocity: Vec2,
    mass: f32,
    impulse: Vec2,
    drag: f32,
    friction: f32,
}

impl Dynamics {
    fn resolve_drag(self: &mut Self) {
        self.impulse -= self.drag * self.velocity.normalize_or_zero() * self.velocity.length() * self.velocity.length();
    }

    fn resolve_friction(self: &mut Self) {
        self.impulse -= self.friction * self.velocity.normalize_or_zero() * self.mass;
    }

    pub fn resolve(self: &mut Self) {
        self.resolve_friction();
        self.resolve_drag();

        self.velocity += self.impulse / self.mass / constants::TICKRATE as f32;
        self.impulse = Vec2::new(0.0, 0.0);
    }

    pub fn update_transform(self: &Self, transform: &mut Transform) {
        transform.translation += self.velocity.extend(0.0) / constants::TICKRATE as f32;
    }

    pub fn apply_impulse(self: &mut Self, impulse: Vec2) {
        self.impulse += impulse;
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
struct Ball;

fn create_players(mut commands: Commands) {
    commands.spawn((
        Player,
        Dynamics {
            velocity: Vec2::new(-100.0, 0.0),
            mass: 80.0,
            impulse: Vec2::new(0.0, 0.0),
            drag: 10.0,
            friction: 10.0,
        },
        TransformBundle::from_transform(Transform::from_xyz(40.0, 0.0, 1.0)),
    ));
}

fn resolve_dynamics(mut objects: Query<(&mut Dynamics, &mut Transform)>) {
    for (mut dynamics, mut transform) in &mut objects {
        dynamics.resolve();
        dynamics.update_transform(&mut transform);
    }
}

#[derive(Default)]
pub struct SoccerBackendPlugin {}

#[derive(Hash, Eq, Clone, PartialEq, Debug, SystemSet)]
pub enum SoccerBackendStartupSet {
    Parallel,
    CommandFlush,
}

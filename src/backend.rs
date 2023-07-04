use bevy::{ecs::schedule::ScheduleLabel, prelude::*};

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
    }
}

#[derive(Component, Debug)]
pub struct Dynamics;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
struct Ball;

pub fn create_players(mut commands: Commands) {
    commands.spawn((
        Player,
        Dynamics,
        TransformBundle::from_transform(Transform::from_xyz(40.0, 0.0, 1.0)),
    ));
}

const TICKRATE: i16 = 1000; // ticks per second

#[derive(Default)]
pub struct SoccerBackendPlugin {}

#[derive(Hash, Eq, Clone, PartialEq, Debug, SystemSet)]
pub enum SoccerBackendStartupSet {
    Parallel,
    CommandFlush,
}



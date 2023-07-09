use bevy::prelude::*;

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
        app.add_startup_systems((create_players, create_ball).in_base_set(StartupSet::Startup));
        app.add_system(resolve_dynamics.in_schedule(CoreSchedule::FixedUpdate));
        app.add_system(resolve_collisions.in_schedule(CoreSchedule::FixedUpdate));
        app.insert_resource(FixedTime::new_from_secs(1.0 / constants::TICKRATE as f32));
    }
}

#[derive(Component, Debug, Copy, Clone, PartialEq, Reflect)]
pub struct Dynamics {
    velocity: Vec2,
    inv_mass: f32,
    impulse: Vec2,
    drag: f32,
    friction: f32,
    brake: bool,
}

impl Dynamics {
    fn new(mass: f32, drag: f32, friction: f32) -> Dynamics {
        Dynamics {
            velocity: Vec2::new(0.0, 0.0),
            inv_mass: 1.0 / mass,
            impulse: Vec2::new(0.0, 0.0),
            drag: drag,
            friction: friction,
            brake: false,
        }
    }
    fn resolve_drag(self: &mut Self) {
        let mut impulse = self.drag;
        impulse *= self.velocity.length_squared();
        impulse *= self.inv_mass;
        impulse /= constants::TICKRATE as f32;

        self.apply_impulse(-self.velocity.normalize_or_zero() * impulse);
    }

    fn resolve_friction(self: &mut Self) {
        let mut impulse = self.friction;
        impulse /= constants::TICKRATE as f32;

        impulse += if self.brake {
            self.brake = false;
            constants::IMPULSE_PER_KEYPRESS
        } else {
            0.0
        };

        self.apply_impulse(-self.velocity.normalize_or_zero() * impulse);
    }

    pub fn resolve(self: &mut Self) {
        self.resolve_friction();
        self.resolve_drag();

        self.velocity += self.impulse * self.inv_mass;
        self.impulse = Vec2::new(0.0, 0.0);
    }

    pub fn update_transform(self: &Self, transform: &mut Transform) {
        transform.translation += self.velocity.extend(0.0) / constants::TICKRATE as f32;
    }

    pub fn apply_impulse(self: &mut Self, impulse: Vec2) {
        self.impulse += impulse;
    }

    pub fn brake(self: &mut Self) {
        self.brake = true;
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
struct Collision {
    mesh: Handle<Mesh>,
}

fn create_players(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let mesh = meshes.add(Mesh::from(shape::Circle::new(constants::PLAYER_RADIUS)));
    commands.spawn((
        Player,
        Dynamics::new(constants::PLAYER_MASS, 700.0, 10.0),
        TransformBundle::from_transform(Transform::from_xyz(40.0, 0.0, 0.0)),
        Collision { mesh: mesh },
    ));
}

fn create_ball(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let mesh = meshes.add(Mesh::from(shape::Circle::new(constants::BALL_RADIUS)));
    commands.spawn((
        Ball,
        Dynamics::new(constants::BALL_MASS, 1.0, 100.0),
        TransformBundle::from_transform(Transform::from_xyz(0.0, 0.0, 0.0)),
        Collision { mesh: mesh },
    ));
}

fn resolve_dynamics(mut objects: Query<(&mut Dynamics, &mut Transform)>) {
    for (mut dynamics, mut transform) in &mut objects {
        dynamics.resolve();
        dynamics.update_transform(&mut transform);
    }
}

fn resolve_collisions(
    mut objects: Query<(Entity, &Collision, &Transform, &mut Dynamics)>,
    meshes: Res<Assets<Mesh>>,
) {
    let mut colliding_entities = vec![];
    for (object_a, _collision_a, transform_a, dynamics_a) in &objects {
        for (object_b, _collision_b, transform_b, dynamics_b) in
            objects.iter().filter(|item| item.0 < object_a)
        {
            let relative_velocity = dynamics_a.velocity - dynamics_b.velocity;
            let relative_position = (transform_a.translation - transform_b.translation).truncate();

            if relative_velocity.dot(relative_position) > 0.0 {
                continue;
            }

            if relative_position.length() < (constants::BALL_RADIUS + constants::PLAYER_RADIUS) {
                let mut penetration_factor = (constants::BALL_RADIUS + constants::PLAYER_RADIUS)
                    - relative_position.length();
                penetration_factor /= (constants::BALL_RADIUS + constants::PLAYER_RADIUS);
                penetration_factor = f32::sqrt(penetration_factor);
                colliding_entities.push((object_a, object_b, penetration_factor));
            }
        }
    }

    for (object_a, object_b, penetration_factor) in colliding_entities {
        let [(_, _collision_a, transform_a, mut dynamics_a), (_, _collision_b, transform_b, mut dynamics_b)] =
            objects.many_mut([object_a, object_b]);

        let mut impulse = 2.0 / (dynamics_a.inv_mass + dynamics_b.inv_mass);
        impulse *= (transform_a.translation - transform_b.translation)
            .truncate()
            .dot(dynamics_a.velocity - dynamics_b.velocity);
        impulse /= (transform_a.translation - transform_b.translation)
            .truncate()
            .length();
        impulse *= 0.9;

        impulse -= constants::EJECTION_IMPULSE * penetration_factor;
        dynamics_a.apply_impulse(
            impulse
                * (transform_b.translation - transform_a.translation)
                    .truncate()
                    .normalize(),
        );
        dynamics_b.apply_impulse(
            impulse
                * (transform_a.translation - transform_b.translation)
                    .truncate()
                    .normalize(),
        );
    }
}

#[derive(Default)]
pub struct SoccerBackendPlugin {}

#[derive(Hash, Eq, Clone, PartialEq, Debug, SystemSet)]
pub enum SoccerBackendStartupSet {
    Parallel,
    CommandFlush,
}

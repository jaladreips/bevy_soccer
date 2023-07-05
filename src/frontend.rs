use bevy::prelude::shape::{Circle, Quad};
use bevy::prelude::*;

use bevy::render::camera::ScalingMode;
use bevy::sprite::MaterialMesh2dBundle;

use crate::backend;
use crate::constants;

#[derive(Default)]
pub struct SoccerFrontendPlugin;

impl Plugin for SoccerFrontendPlugin {
    // fn _build(&self, app: &mut App) {
    //     app.add_startup_system(camera_setup);
    //     app.configure_sets(
    //         (
    //             backend::SoccerBackendStartupSet::CommandFlush,
    //             SoccerFrontendStartupSet::Parallel,
    //             SoccerFrontendStartupSet::CommandFlush,
    //             StartupSet::PostStartup,
    //         )
    //             .chain(),
    //     );
    //     app.add_system(apply_system_buffers.in_set(SoccerFrontendStartupSet::CommandFlush));
    //     app.add_startup_system(prepare_player.in_set(SoccerFrontendStartupSet::Parallel));
    // }

    fn build(&self, app: &mut App) {
        app.add_startup_system(camera_setup);
        app.add_startup_system(draw_field);
        app.add_startup_system(draw_player.in_base_set(StartupSet::PostStartup));
    }
}

#[derive(Component)]
struct MainCamera;

fn camera_setup(mut commands: Commands) {
    const FIELD_SCALE: f32 = 1.2;
    commands.spawn((
        Camera2dBundle {
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::Fixed {
                    width: FIELD_SCALE * constants::FIELD_SIZE.x,
                    height: FIELD_SCALE * constants::FIELD_SIZE.y,
                },
                ..Default::default()
            },
            ..Default::default()
        },
        MainCamera,
    ));
}

fn draw_player(
    mut commands: Commands,
    players: Query<Entity, With<backend::Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for player in &players {
        let mesh = meshes.add(Mesh::from(Circle::new(2.0)));
        let material = materials.add(ColorMaterial::from(Color::PURPLE));

        commands
            .spawn(MaterialMesh2dBundle {
                mesh: mesh.into(),
                material: material,
                transform: Transform::from_xyz(0.0, 0.0, ZLayerOrder::PLAYERS.z()),
                ..Default::default()
            })
            .set_parent(player);

        commands
            .get_entity(player)
            .unwrap()
            .insert(VisibilityBundle::default());
    }
}

fn draw_field(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(Quad {
        size: constants::FIELD_SIZE,
        flip: false,
    }));
    let material = materials.add(ColorMaterial::from(Color::YELLOW_GREEN));
    commands.spawn(MaterialMesh2dBundle {
        mesh: mesh.into(),
        material: material,
        transform: Transform::from_xyz(0.0, 0.0, ZLayerOrder::FIELD.z()),
        ..Default::default()
    });
}

#[derive(Hash, Eq, Clone, PartialEq, Debug, SystemSet)]
pub enum SoccerFrontendStartupSet {
    Parallel,
    CommandFlush,
}

#[repr(C)]
enum ZLayerOrder {
    // from background to foreground
    FIELD,
    PLAYERS,
    NUM_LAYERS,
}

impl ZLayerOrder {
    fn z(self: Self) -> f32 {
        let current_layer = self as u32 as f32;
        let num_layers = Self::NUM_LAYERS as u32 as f32;
        current_layer / num_layers
    }
}

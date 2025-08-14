use avian3d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::{input::leafwing::SnapshotBuffer, *};

use reclipsis_assets::*;
use reclipsis_common::{protocol::*, *};

use crate::AppState;

mod camera;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(AppState = AppState::Game)]
enum SpawnedState {
    Spawned,
    #[default]
    NotSpawned,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<SpawnedState>()
            .add_plugins(camera::CameraPlugin)
            .add_systems(
                FixedUpdate,
                handle_character_actions.run_if(in_state(AppState::Game)),
            )
            .add_systems(
                Update,
                (handle_new_character, handle_new_floor, handle_new_block)
                    .run_if(in_state(AppState::Game)),
            );
    }
}

fn handle_character_actions(
    time: Res<Time>,
    spatial_query: SpatialQuery,
    mut query: Query<
        (
            &ActionState<CharacterAction>,
            &SnapshotBuffer<CharacterAction>,
            CharacterQuery,
        ),
        With<Predicted>,
    >,
    timeline: Single<&LocalTimeline>,
) {
    let tick = timeline.tick();
    for (action_state, input_buffer, mut character) in &mut query {
        if input_buffer.get(tick).is_some() {
            apply_character_action(&time, &spatial_query, action_state, &mut character);
            continue;
        }

        if let Some((_, prev_action_state)) = input_buffer.get_last_with_tick() {
            apply_character_action(&time, &spatial_query, prev_action_state, &mut character);
        } else {
            apply_character_action(&time, &spatial_query, action_state, &mut character);
        }
    }
}

fn handle_new_character(
    mut commands: Commands,
    mut character_query: Query<
        (Entity, Has<Controlled>),
        (Added<Predicted>, With<character::CharacterMarker>),
    >,
    mut next_state: ResMut<NextState<SpawnedState>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, is_controlled) in &mut character_query {
        if is_controlled {
            info!("Adding InputMap to controlled and predicted entity {entity:?}");

            // Add InputMap
            commands.entity(entity).insert(
                InputMap::new([(CharacterAction::Jump, KeyCode::Space)])
                    .with(CharacterAction::Jump, GamepadButton::South)
                    .with_dual_axis(CharacterAction::Move, GamepadStick::LEFT)
                    .with_dual_axis(CharacterAction::Move, VirtualDPad::wasd()),
            );

            next_state.set(SpawnedState::Spawned);
        } else {
            info!("Remote character predicted for us: {entity:?}");
        }
        commands
            .entity(entity)
            .insert(character::CharacterPhysicsBundle::default())
            .insert((
                Mesh3d(meshes.add(Capsule3d::new(
                    character::CHARACTER_CAPSULE_RADIUS,
                    character::CHARACTER_CAPSULE_HEIGHT,
                ))),
                MeshMaterial3d(materials.add(StandardMaterial { ..default() })),
            ));
    }
}

fn handle_new_floor(
    mut commands: Commands,
    floor_query: Query<Entity, (Added<Replicated>, With<floor::FloorMarker>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for entity in &floor_query {
        info!("Handling new floor");
        commands
            .entity(entity)
            .insert(floor::FloorPhysicsBundle::default())
            .insert((
                Mesh3d(meshes.add(Cuboid::new(
                    floor::FLOOR_WIDTH,
                    floor::FLOOR_HEIGHT,
                    floor::FLOOR_WIDTH,
                ))),
                MeshMaterial3d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
            ));
    }
}

fn handle_new_block(
    mut commands: Commands,
    block_query: Query<Entity, (Added<Predicted>, With<block::BlockMarker>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for entity in &block_query {
        info!("Handling new block");
        commands
            .entity(entity)
            .insert(block::BlockPhysicsBundle::default())
            .insert((
                Mesh3d(meshes.add(Cuboid::new(
                    block::BLOCK_WIDTH,
                    block::BLOCK_HEIGHT,
                    block::BLOCK_WIDTH,
                ))),
                MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 1.0))),
            ));
    }
}

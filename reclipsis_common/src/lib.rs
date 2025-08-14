use avian3d::prelude::*;
use bevy::{ecs::query::QueryData, prelude::*};
use leafwing_input_manager::prelude::*;
use reclipsis_assets::character;

use crate::protocol::CharacterAction;

pub const FIXED_TIMESTEP_HZ: f64 = 60.0;

pub mod protocol;

pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(protocol::ProtocolPlugin).add_plugins(
            PhysicsPlugins::default()
                .build()
                .disable::<PhysicsInterpolationPlugin>(),
        );
    }
}

#[derive(QueryData)]
#[query_data(mutable, derive(Debug))]
pub struct CharacterQuery {
    pub external_force: &'static mut ExternalForce,
    pub external_impulse: &'static mut ExternalImpulse,
    pub linear_velocity: &'static LinearVelocity,
    pub mass: &'static ComputedMass,
    pub transform: &'static mut Transform,
    pub entity: Entity,
}

pub fn apply_character_action(
    time: &Res<Time>,
    spatial_query: &SpatialQuery,
    action_state: &ActionState<CharacterAction>,
    character: &mut CharacterQueryItem,
) {
    const MAX_SPEED: f32 = 5.0;
    const MAX_ACCELERATION: f32 = 25.0;

    let max_velocity_delta_per_tick = MAX_ACCELERATION * time.delta_secs();

    if action_state.just_pressed(&CharacterAction::Jump) {
        let ray_cast_origin = character.transform.translation
            + Vec3::new(
                0.0,
                -character::CHARACTER_CAPSULE_HEIGHT / 2.0 - character::CHARACTER_CAPSULE_RADIUS,
                0.0,
            );

        if spatial_query
            .cast_ray(
                ray_cast_origin,
                Dir3::NEG_Y,
                0.01,
                true,
                &SpatialQueryFilter::from_excluded_entities([character.entity]),
            )
            .is_some()
        {
            character
                .external_impulse
                .apply_impulse(Vec3::new(0.0, 5.0, 0.0));
        }
    }

    // Rotate character
    let rotate_dir = action_state
        .value(&CharacterAction::Rotate);
    character.transform.rotation = Quat::from_rotation_y(rotate_dir);

    // Move character
    let move_dir = action_state
        .axis_pair(&CharacterAction::Move)
        .clamp_length_max(1.0);
    let move_dir = Vec3::new(move_dir.x, 0.0, -move_dir.y);

    let local_move_dir = character.transform.rotation * move_dir;

    let ground_linear_velocity = Vec3::new(
        character.linear_velocity.x,
        0.0,
        character.linear_velocity.z,
    );

    let desired_ground_linear_velocity = local_move_dir * MAX_SPEED;

    let new_ground_linear_velocity = ground_linear_velocity
        .move_towards(desired_ground_linear_velocity, max_velocity_delta_per_tick);

    let required_acceleration =
        (new_ground_linear_velocity - ground_linear_velocity) / time.delta_secs();

    character
        .external_force
        .apply_force(required_acceleration * character.mass.value());
}

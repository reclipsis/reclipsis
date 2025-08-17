use avian3d::prelude::*;
use bevy::{
    ecs::query::QueryData,
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    prelude::*,
    window::PrimaryWindow,
};
use leafwing_input_manager::{plugin::InputManagerSystem, prelude::ActionState};
use reclipsis_common::protocol::CharacterAction;
use std::{f32::consts::FRAC_PI_2, ops::Range};

use lightyear::{input::client::InputSet, prelude::Controlled};
use reclipsis_assets::character::CharacterMarker;

use crate::game::SpawnedState;

const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;

const SCROLL_SENSITIVITY: f32 = 1.0;
const PITCH_SPEED: f32 = 0.003;
const PITCH_RANGE: Range<f32> = -PITCH_LIMIT..PITCH_LIMIT;
const ORBIT_RANGE: Range<f32> = 5.0..40.0;
const YAW_SPEED: f32 = 0.004;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraYaw>()
            .init_resource::<OrbitDistance>()
            .add_systems(Update, orbit.run_if(in_state(SpawnedState::Spawned)))
            .add_systems(
                FixedPreUpdate,
                update_character_rotation
                    .after(orbit)
                    .before(InputSet::BufferClientInputs)
                    .in_set(InputManagerSystem::ManualControl)
                    .run_if(in_state(SpawnedState::Spawned)),
            );
    }
}

#[derive(Debug, Resource, Reflect)]
pub struct OrbitDistance(f32);

impl Default for OrbitDistance {
    fn default() -> Self {
        OrbitDistance(20.0)
    }
}

#[derive(Debug, Resource, Default, Reflect)]
pub struct CameraYaw(Option<f32>);

#[derive(QueryData)]
#[query_data(mutable, derive(Debug))]
pub struct CharacterQuery {
    pub entity: Entity,
    pub transform: &'static Transform,
    pub action_state: &'static mut ActionState<CharacterAction>,
}

pub fn orbit(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut camera: Single<&mut Transform, With<Camera>>,
    character: Single<CharacterQuery, (With<CharacterMarker>, With<Controlled>, Without<Camera>)>,
    mouse: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
    mut orbit_distance: ResMut<OrbitDistance>,
    mut camera_yaw: ResMut<CameraYaw>,
    spatial_query: SpatialQuery,
) {
    let motion_delta = mouse_motion.delta;
    let scroll_delta = -mouse_scroll.delta.y * SCROLL_SENSITIVITY;

    orbit_distance.0 = (orbit_distance.0 + scroll_delta).clamp(ORBIT_RANGE.start, ORBIT_RANGE.end);

    let mut delta_pitch = motion_delta.y * PITCH_SPEED;
    let mut delta_yaw = motion_delta.x * YAW_SPEED;

    if !mouse.pressed(MouseButton::Right) {
        // Unlock cursor
        for mut window in windows.iter_mut() {
            window.cursor_options.grab_mode = bevy::window::CursorGrabMode::None;
        }

        // Ignore input
        delta_pitch = 0.0;
        delta_yaw = 0.0;
    } else {
        // Lock cursor
        for mut window in windows.iter_mut() {
            window.cursor_options.grab_mode = bevy::window::CursorGrabMode::Locked;
        }
    }

    let (yaw, pitch, _roll) = camera.rotation.to_euler(EulerRot::YXZ);

    let pitch = (pitch + delta_pitch).clamp(PITCH_RANGE.start, PITCH_RANGE.end);
    let yaw = yaw + delta_yaw;
    camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);

    // If the camera collides with something, move it closer
    let mut local_orbit_distance = orbit_distance.0;
    if let Ok(dir) = Dir3::new(camera.translation - character.transform.translation)
        && let Some(hit) = spatial_query.cast_shape(
            &Collider::sphere(0.1),
            character.transform.translation,
            Quat::default(),
            dir,
            &ShapeCastConfig::from_max_distance(orbit_distance.0),
            &SpatialQueryFilter::default().with_excluded_entities(vec![character.entity]),
        )
    {
        local_orbit_distance = hit.distance;
    }

    camera.translation = character.transform.translation - camera.forward() * local_orbit_distance;
    if delta_yaw != 0.0 {
        camera_yaw.0 = Some(yaw);
    }
}

fn update_character_rotation(
    mut character_action_state: Single<
        &mut ActionState<CharacterAction>,
        (With<CharacterMarker>, With<Controlled>),
    >,
    mut camera_yaw: ResMut<CameraYaw>,
) {
    if let Some(yaw) = camera_yaw.0 {
        character_action_state.set_value(&CharacterAction::Rotate, yaw);
        camera_yaw.0 = None;
    }
}

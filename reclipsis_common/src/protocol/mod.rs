use avian3d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use lightyear::{input::config::InputConfig, prelude::{input::leafwing::InputPlugin, *}};
use serde::{Deserialize, Serialize};

use reclipsis_assets::*;

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputPlugin::<CharacterAction> {
            config: InputConfig::<CharacterAction> {
                rebroadcast_inputs: true,
                ..default()
            },
        });

        //
        // Objects
        app.register_component::<Name>()
            .add_prediction(PredictionMode::Once);

        app.register_component::<character::CharacterMarker>()
            .add_prediction(PredictionMode::Once)
            .add_interpolation(InterpolationMode::Once);

        app.register_component::<floor::FloorMarker>()
            .add_prediction(PredictionMode::Once);

        app.register_component::<block::BlockMarker>()
            .add_prediction(PredictionMode::Once);

        //
        // Physics
        app.register_component::<LinearVelocity>()
            .add_prediction(PredictionMode::Full);

        app.register_component::<AngularVelocity>()
            .add_prediction(PredictionMode::Full);

        app.register_component::<ExternalForce>()
            .add_prediction(PredictionMode::Full);

        app.register_component::<ExternalImpulse>()
            .add_prediction(PredictionMode::Full);

        app.register_component::<ComputedMass>()
            .add_prediction(PredictionMode::Full);

        app.register_component::<Position>()
            .add_prediction(PredictionMode::Full)
            .add_should_rollback(position_should_rollback)
            .add_linear_correction_fn()
            .add_interpolation(InterpolationMode::Full)
            .add_linear_interpolation_fn();

        app.register_component::<Rotation>()
            .add_prediction(PredictionMode::Full)
            .add_should_rollback(rotation_should_rollback)
            .add_linear_correction_fn()
            .add_interpolation(InterpolationMode::Full)
            .add_linear_interpolation_fn();
    }
}

#[derive(Serialize, Deserialize, Debug, Reflect, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CharacterAction {
    Move,
    Jump,
}

impl Actionlike for CharacterAction {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            Self::Move => InputControlKind::DualAxis,
            Self::Jump => InputControlKind::Button,
        }
    }
}

fn position_should_rollback(this: &Position, that: &Position) -> bool {
    (this.0 - that.0).length() >= 0.01
}

fn rotation_should_rollback(this: &Rotation, that: &Rotation) -> bool {
    this.angle_between(that.0) >= 0.01
}

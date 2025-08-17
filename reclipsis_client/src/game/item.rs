use bevy::{input::keyboard::KeyboardInput, prelude::*};
use leafwing_input_manager::{plugin::InputManagerSystem, prelude::ActionState};
use lightyear::{input::client::InputSet, prelude::Controlled};
use reclipsis_assets::character::CharacterMarker;
use reclipsis_common::protocol::CharacterAction;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedPreUpdate,
            equip_item
                .before(InputSet::BufferClientInputs)
                .in_set(InputManagerSystem::ManualControl),
        );
    }
}

fn equip_item(
    mut character_action_state: Single<
        &mut ActionState<CharacterAction>,
        (With<CharacterMarker>, With<Controlled>),
    >,
    mut keyboard_events: EventReader<KeyboardInput>,
) {
    for event in keyboard_events.read() {
        let slot: f32 = match event.key_code {
            KeyCode::Digit0 => 0.0,
            KeyCode::Digit1 => 1.0,
            KeyCode::Digit2 => 2.0,
            KeyCode::Digit3 => 3.0,
            KeyCode::Digit4 => 4.0,
            KeyCode::Digit5 => 5.0,
            KeyCode::Digit6 => 6.0,
            KeyCode::Digit7 => 7.0,
            KeyCode::Digit8 => 8.0,
            KeyCode::Digit9 => 9.0,
            _ => continue,
        };

        character_action_state.set_value(&CharacterAction::Equip, slot);
    }
}

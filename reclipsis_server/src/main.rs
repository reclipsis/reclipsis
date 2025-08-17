use avian3d::prelude::*;
use bevy::{log::LogPlugin, prelude::*, render::mesh::MeshPlugin, scene::ScenePlugin};
use leafwing_input_manager::prelude::*;
use lightyear::prelude::{server::*, *};

use reclipsis_common::{
    CharacterQuery, FIXED_TIMESTEP_HZ, apply_character_action, protocol::CharacterAction,
};
use std::time::Duration;

use reclipsis_assets::*;

pub const SEND_INTERVAL: Duration = Duration::from_millis(100);

fn main() {
    App::new()
        .add_plugins(
            MinimalPlugins
                .build()
                .add(AssetPlugin::default())
                .add(MeshPlugin)
                .add(ScenePlugin)
                .add(LogPlugin::default()),
        )
        .add_plugins(reclipsis_common::SharedPlugin)
        .add_plugins(server::ServerPlugins {
            tick_duration: Duration::from_secs_f64(1.0 / FIXED_TIMESTEP_HZ),
        })
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, handle_character_actions)
        .add_observer(handle_new_client)
        .add_observer(handle_connected)
        .run();
}

fn setup(mut commands: Commands) {
    let server = commands
        .spawn((
            NetcodeServer::new(NetcodeConfig::default()),
            LocalAddr("127.0.0.1:8080".parse().unwrap()),
            ServerUdpIo::default(),
        ))
        .id();
    commands.trigger_targets(Start, server);

    commands.spawn((
        Name::new("Floor"),
        floor::FloorPhysicsBundle::default(),
        floor::FloorMarker,
        Position::new(Vec3::ZERO),
        Replicate::to_clients(NetworkTarget::All),
    ));

    commands.spawn((
        Name::new("Block"),
        block::BlockPhysicsBundle::default(),
        block::BlockMarker,
        Position::new(Vec3::new(1.0, 1.0, 0.0)),
        Replicate::to_clients(NetworkTarget::All),
        PredictionTarget::to_clients(NetworkTarget::All),
    ));
}

fn handle_new_client(trigger: Trigger<OnAdd, LinkOf>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .insert(ReplicationSender::new(
            SEND_INTERVAL,
            SendUpdatesMode::SinceLastAck,
            false,
        ));
}

fn handle_connected(
    trigger: Trigger<OnAdd, Connected>,
    query: Query<&RemoteId, With<ClientOf>>,
    mut commands: Commands,
) {
    let Ok(client_id) = query.get(trigger.target()) else {
        return;
    };
    let client_id = client_id.0;
    info!("Client connected with client-id {client_id:?}. Spawning character entity.");

    let character = commands
        .spawn((
            Name::new("Character"),
            ActionState::<CharacterAction>::default(),
            Position(Vec3::new(0.0, 5.0, 0.0)),
            Replicate::to_clients(NetworkTarget::All),
            PredictionTarget::to_clients(NetworkTarget::All),
            ControlledBy {
                owner: trigger.target(),
                lifetime: Default::default(),
            },
            character::CharacterPhysicsBundle::default(),
            character::CharacterMarker,
            inventory::Inventory::default(),
        ))
        .id();

    info!("Created entity {character:?} for client {client_id:?}");
}

fn handle_character_actions(
    time: Res<Time>,
    spatial_query: SpatialQuery,
    mut query: Query<(&ActionState<CharacterAction>, CharacterQuery)>,
) {
    for (action_state, mut character) in &mut query {
        apply_character_action(&time, &spatial_query, action_state, &mut character);
    }
}

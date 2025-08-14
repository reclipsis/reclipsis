use bevy::prelude::*;
use lightyear::{
    netcode::{Key, NetcodeClient},
    prelude::{client::NetcodeConfig, *},
};

use crate::AppState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), debug_connect);
    }
}

fn debug_connect(mut commands: Commands, mut next_state: ResMut<NextState<AppState>>) {
    let client_addr = "127.0.0.1:1337".parse().unwrap();
    let server_addr = "127.0.0.1:8080".parse().unwrap();

    let auth = Authentication::Manual {
        server_addr,
        client_id: 0,
        private_key: Key::default(),
        protocol_id: 0,
    };
    let client = commands
        .spawn((
            Client::default(),
            LocalAddr(client_addr),
            PeerAddr(server_addr),
            Link::new(None),
            ReplicationReceiver::default(),
            PredictionManager::default(),
            InterpolationManager::default(),
            NetcodeClient::new(auth, NetcodeConfig::default()).unwrap(),
            UdpIo::default(),
        ))
        .id();
    commands.trigger_targets(Connect, client);

    next_state.set(AppState::Game);
}

use crate::network::{TargetServer, TargetServerState};
use bevy::prelude::*;
use bevy_renet::renet::{DefaultChannel, RenetClient};
use bincode::Options;
use shared::messages::{ClientToServerMessage, ExitOrder};

pub fn terminate_server_connection(
    mut client: ResMut<RenetClient>,
    mut target: ResMut<TargetServer>,
) {
    info!("Terminating server connection");
    let order = ClientToServerMessage::Exit(ExitOrder {
        session_token: target.session_token.unwrap_or_default(),
    });
    let payload = bincode::options().serialize(&order).unwrap();
    client.send_message(DefaultChannel::ReliableOrdered, payload);

    target.address = None;
    target.username = None;
    target.session_token = None;
    target.state = TargetServerState::Initial;
}

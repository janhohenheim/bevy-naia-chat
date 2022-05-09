use bevy::log;
use bevy::prelude::*;
use naia_bevy_server::{
    events::{AuthorizationEvent, ConnectionEvent, DisconnectionEvent, MessageEvent},
    shared::Random,
    Server,
};
use shared::{Channels, Protocol};

pub fn authorization(
    mut event_reader: EventReader<AuthorizationEvent<Protocol>>,
    mut server: Server<Protocol, Channels>,
) {
    for event in event_reader.iter() {
        if let AuthorizationEvent(user_key, Protocol::Auth(auth)) = event {
            if *auth.username == "charlie" && *auth.password == "12345" {
                // Accept incoming connection
                server.accept_connection(user_key);
                log::info!("Accepted connection");
            } else {
                // Reject incoming connection
                server.reject_connection(user_key);
                log::info!("Rejected connection");
            }
        }
    }
}

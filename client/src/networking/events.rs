use naia_bevy_client::Client;
use shared::{Channels, Protocol};

pub fn connect_event(client: Client<Protocol, Channels>) {
    info!("Client connected to: {}", client.server_address());
}

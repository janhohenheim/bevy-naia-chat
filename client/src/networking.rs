mod events;
use bevy::log;
use bevy::prelude::*;
use naia_bevy_client::{Client, ClientConfig, Plugin as ClientPlugin, Stage};
use shared::{protocol, shared_config, Channels, Protocol};

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ClientPlugin::<Protocol, Channels>::new(
            ClientConfig::default(),
            shared_config(),
        ))
        .add_system_to_stage(Stage::Connection, events::connect_event)
        .add_startup_system(auth);
    }
}

fn auth(mut client: Client<Protocol, Channels>) {
    log::info!("Authenticating...");
    client.auth(protocol::Auth::new("charlie", "12345"));
    client.connect("http://127.0.0.1:14191");
}

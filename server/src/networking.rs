use bevy::prelude::*;
use naia_bevy_server::{Plugin as ServerPlugin, ServerConfig, Stage};
use shared::{shared_config, Channels, Protocol};
mod events;

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ServerPlugin::<Protocol, Channels>::new(
            ServerConfig::default(),
            shared_config(),
        ));
    }
}

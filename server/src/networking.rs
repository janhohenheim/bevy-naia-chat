use naia_bevy_server::{Plugin as ServerPlugin, ServerConfig, Stage};
use shared::{shared_config, Channels, Protocol};
use bevy::prelude::*;
mod events;

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(ServerPlugin::<Protocol, Channels>::new(
            ServerConfig::default(),
            shared_config(),
        ))
    }
}

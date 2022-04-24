use naia_shared::{
    derive_channels, Channel, ChannelDirection, ChannelMode, ReliableSettings, TickBufferSettings,
};

#[derive_channels]
pub enum Channels {
    Bar,
}

pub const CHANNEL_CONFIG: &[Channel<Channels>] = &[Channel {
    index: Channels::Bar,
    direction: ChannelDirection::ServerToClient,
    mode: ChannelMode::UnorderedReliable(ReliableSettings::default()),
}];

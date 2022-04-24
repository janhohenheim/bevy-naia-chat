use bevy::prelude::*;

use naia_shared::Replicate;

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct Foo {}

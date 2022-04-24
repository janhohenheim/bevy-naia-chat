use naia_shared::Protocolize;

mod auth;
mod foo;
pub use auth::Auth;
use foo::Foo;

#[derive(Protocolize)]
pub enum Protocol {
    Auth(Auth),
    Foo(Foo),
}

use naia_shared::Protocolize;

mod foo;
use foo::Foo;

#[derive(Protocolize)]
pub enum Protocol {
    Foo(Foo),
}

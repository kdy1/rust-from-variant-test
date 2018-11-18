extern crate ast_node;

use ast_node::FromVariant;

pub struct Foo {}
pub struct Bar {}

#[derive(FromVariant)]
pub enum Example {
    Foo(Foo),
    Bar(Bar),
}

fn main() {
    println!("Hello, world!");
}

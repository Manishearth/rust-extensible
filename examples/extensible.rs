#![feature(plugin)]
#![allow(dead_code, unused_attributes)]
#[plugin] extern crate extensible;

#[extensible]
enum Foo {
    Bar,
    Baz(u8),
    Quux
}
pub use Foo::*;

fn main() {
    let x = Bar;
    let mut out = match x {
        Bar => 1u8,
        Baz(y) => y,
        Quux => 0u8, 
    };
    println!("{}", out);
    out = match x {
        Bar => 1u8,
        Baz(y) => y,
        _ => 0u8, 
    };
    println!("{}", out);
}
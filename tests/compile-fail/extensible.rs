#![feature(plugin, custom_attribute)]
#![plugin(extensible)]

#![allow(dead_code, unused_attributes)]
#[extensible]
enum Foo {
    Bar,
    Baz(u8),
    Quux
}
pub use Foo::*;

fn main() {
    let x = Bar;
    let mut out = match x { //~ ERROR The enum Foo is marked as extensible
        Bar => 1u8,
        Baz(y) => y,
        Quux => 0u8, 
    };
    println!("{}", out);

    // This is fine
    out = match x {
        Bar => 1u8,
        Baz(y) => y,
        _ => 0u8, 
    };
    println!("{}", out);
}
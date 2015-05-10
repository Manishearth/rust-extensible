# rust-extensible

[![Build Status](https://travis-ci.org/Manishearth/rust-extensible.svg)](https://travis-ci.org/Manishearth/rust-extensible)

Extensible enums for Rust

This is a plugin form of [this RfC](https://github.com/rust-lang/rfcs/pull/757).

Basically, if an enum is marked `#[extensible]`, this plugin will prevent its use in a match statement lacking a wildcard. This lets library authors define stable enums whilst
keeping the flexibility of extending them later.


```rust
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
        // There is no wildcard here, so it will not compile
    };
    println!("{}", out);

    // This is fine
    out = match x {
        Bar => 1u8,
        Baz(y) => y,
        _ => 0u8, // THis will compile fine
    };
    println!("{}", out);
}
```
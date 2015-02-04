# rust-extensible

Extensible enums for Rust

This is a plugin form of [this RfC](https://github.com/rust-lang/rfcs/pull/757).

Basically, if an enum is marked `#[extensible]`, this plugin will prevent its use in a match statement lacking a wildcard. This lets library authors define stable enums whilst
keeping the flexibility of extending them later.
# `mobile-entry-point`

[![crates.io badge](http://meritbadge.herokuapp.com/mobile-entry-point)](https://crates.io/crates/mobile-entry-point)
[![docs.rs badge](https://docs.rs/mobile-entry-point/badge.svg)](https://docs.rs/mobile-entry-point)
[![CI Status](https://github.com/BrainiumLLC/mobile-entry-point/workflows/CI/badge.svg)](https://github.com/BrainiumLLC/mobile-entry-point/actions)

This attribute macro wraps a function to make it the entry-point for an iOS or Android mobile app. Use [`cargo-mobile`](https://github.com/BrainiumLLC/cargo-mobile) to generate matching project files!

```rust
use mobile_entry_point::mobile_entry_point;

#[mobile_entry_point]
fn main() {
    println!("Hello world!");
}
```

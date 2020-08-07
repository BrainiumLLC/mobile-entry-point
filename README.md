# `mobile-entry-point`

This attribute macro wraps a function to make it the entry-point for an iOS or Android mobile app. Use [`cargo-mobile`](https://github.com/BrainiumLLC/yes-or-no) to generate matching project files!

```rust
use mobile_entry_point::mobile_entry_point;

#[mobile_entry_point]
fn main() {
    println!("Hello world!");
}
```

# Introspectable
![https://crates.io/crates/introspectable](https://img.shields.io/crates/l/introspectable?style=for-the-badge)
![https://crates.io/crates/introspectable](https://img.shields.io/crates/v/introspectable?style=for-the-badge)
![https://docs.rs/introspectable/](https://img.shields.io/docsrs/introspectable?style=for-the-badge)

Provides extremely simple introspection through the Introspectable trait, which enables runtime type introspection. A derive macro is available through the crate [introspectable_derive](https://github.com/peperworx/introspectable_derive).

This crate only supports owned types, excepting specifically 'static lifetimes.

## Example

```rust
#[macro_use]
extern crate introspectable_derive;

use introspectable::Introspectable;


#[derive(Introspectable)]
struct TestIntrospect {
    a: u32,
    b: &'static u32,
    c: [u128; 2]
}

fn main() {
    println!("{:?}", TestIntrospect::introspect());
}
```
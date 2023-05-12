# Introspectable

Provides an Introspectable trait, which enables runtime type introspection. Derive macro is available through the crate [introspectable_derive](https://github.com/peperworx/introspectable_derive).

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
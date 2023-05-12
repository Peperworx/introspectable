# Introspectable

Provides an Introspectable trait, which enables runtime type introspection. Derive macro is available through the crate [introspectable_derive](https://github.com/peperworx/introspectable_derive).

This crate does not yet support arrays. [introspectable_derive](https://github.com/peperworx/introspectable_derive) is being reworked currently to support this.

## Example

```rust
#[macro_use]
extern crate introspectable_derive;

use introspectable::Introspectable;


#[derive(Introspectable)]
struct TestIntrospect {
    a: u32,
    b: i8,
}

fn main() {
    println!("{:?}", TestIntrospect::introspect());
}
```
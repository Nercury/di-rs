### [Broken right now, because mechanism that made it work is gone](https://github.com/rust-lang/rust/issues/20770)

However, I have a plan: [See the Roadmap issue #1](https://github.com/Nercury/di-rs/issues/1).

# Dependency Injection container for Rust

[![Build Status](https://travis-ci.org/Nercury/di-rs.svg?branch=master)](https://travis-ci.org/Nercury/di-rs)

This library implements dependency injection container for Rust
mimicking the way it is done in other languages and
frameworks.

It differs from other popular implementations by providing
a simple way to group factories together using `one_of` method.

## Example

```rust
let mut registry = di::Registry::new();

registry
    .one_of("values", || -> i32 { 1 })
    .with_id("a")
    .insert();

registry
    .one_of("values", |a: i32| -> i32 { a + 2 })
    .with_arg("a")
    .insert();

match registry.compile() {
    Ok(container) => {
        if let Some(a) = container.get::<Vec<i32>>("values") {
            assert_eq!(vec![ 1, 1 + 2 ], a.take());
        }
    },
    Err(errors) => di::error_printer::pretty_print(&errors),
}
```

Of course, ungrouped dependencies are also available.

## Documentation

- [Read overview and motivation for creating this library](http://nercury.github.io/di-rs)
- [Jump directly to `Registry` examples](http://nercury.github.io/di-rs/di/registry/struct.Registry.html)

## Usage

Put this in your `Cargo.toml`:

```toml
[dependencies]
di = "*"
```

And this in your crate root:

```rust
extern crate di;
```

## License

MIT

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

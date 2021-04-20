# This is not rocket science‽

A project demonstrating how to use [UniFFI] to write a cross-platform Rust library and auto-generate bindings in different languages.

This project was used as an example at the [Rust Linz Meetup in April 2021](https://www.meetup.com/Rust-Linz/events/276521001/) during my talk "Leveraging Rust to build cross-platform mobile libraries".

## Tests

Tests are implemented in the respective languages. See [`tests/bindings/`](tests/bindings/) for a list of all implemented language bindings.

You can run all binding tests using `cargo`:

```
cargo xtask test
```

Caution: This will require `swift`, `kotlinc` and `python3`.
If you want to run tests for only a single language you can do so:

```
cargo xtask test kotlin
cargo xtask test swift
cargo xtask test python
```

## Generate bindings code

When just testing all code is generated at build time.
If you want to generate the code for inspection,
to check it into source control or for other purposes
use [`uniffi_bindgen`].

To install it:

```
cargo install uniffi_bindgen
```

To generate the bindings:

```
uniffi-bindgen generate ./src/rocketscience.udl -l kotlin -o bindings
uniffi-bindgen generate ./src/rocketscience.udl -l swift -o bindings
uniffi-bindgen generate ./src/rocketscience.udl -l python -o bindings
```


## License

MIT. See [LICENSE](LICENSE).  
Copyright (c) 2021 Jan-Erik Rediger <janerik@fnordig.de>

[uniffi]: https://github.com/mozilla/uniffi-rs
[`uniffi_bindgen`]: https://crates.io/crates/uniffi_bindgen

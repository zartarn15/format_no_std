# `format_no_std`

> Implements `write_str` to get `write_fmt`, which is used in the `format!()` and
> `format_args!()` macros. For `no_std` formatting in a bare metal environment.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  https://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Usage

Library usage example

```rust
let mut buf = [0u8; 64];
let s = format_no_std::show(
    &mut buf,
    format_args!("Test String {}: {}", "foo", 42),
).unwrap();

assert_eq!("Test String foo: 42", s);
```

# `format_no_std`

> Implement **write_str** to get **write_fmt** used in macro **format!()** and
> **format_args!()** for **no_std** formatting bare metal environment

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

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

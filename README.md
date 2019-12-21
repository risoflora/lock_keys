# `lock_keys`

[![Build Status][travis-badge]][travis-url]
[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![License][license-badge]][license-url]

[travis-badge]: https://travis-ci.org/risoflora/lock_keys.svg
[travis-url]: https://travis-ci.org/risoflora/lock_keys
[crates-badge]: https://img.shields.io/crates/v/lock_keys.svg
[crates-url]: https://crates.io/crates/lock_keys
[docs-badge]: https://docs.rs/lock_keys/badge.svg
[docs-url]: https://docs.rs/lock_keys
[license-badge]: https://img.shields.io/crates/l/lock_keys.svg
[license-url]: https://github.com/risoflora/lock_keys#license

`lock_keys` provides a cross platform way for lock keys handling.

Supported platforms: Linux ([Xlib](https://en.wikipedia.org/wiki/Xlib) static) and Windows ([winuser API](https://docs.microsoft.com/en-us/windows/win32/api/winuser)).

## Example

The example below shows how to turn on the Number Lock key:

```rust
extern crate lock_keys;

use lock_keys::*;

fn main() {
    let lockkey = LockKey::new();
    lockkey.enable(LockKeys::NumberLock).unwrap();
}
```

## Usage

Add this to your `Cargo.toml`:

```ini
[dependencies]
lock_keys = "1.0.0"
```

and this to your crate root:

```rust
extern crate lock_keys;
```

## Contributions

Pull Requests and Issues are welcome!

## License

`lock_keys` is licensed under either of the following, at your option:

- Apache License 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

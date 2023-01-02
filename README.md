# `lock_keys`

[![CI/CD][ci-cd-badge]][ci-cd-url]
[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![License][license-badge]][license-url]

`lock_keys` provides a cross platform way for lock keys handling.

Supported platforms: Linux ([Xlib][xlib-wiki-url] static),
Windows ([winuser API][winuser-api-url]) and macOS ([IOKit][iokit-url]).

## Usage

Add this to your `Cargo.toml`:

```ini
[dependencies]
lock_keys = "*"
```

and then:

```rust
use lock_keys::*;

fn main() {
    let lock_key = LockKey::new();
    lock_key.enable(LockKeys::CapitalLock).unwrap();
}
```

## Contributions

Pull Requests are welcome! =)

## License

`lock_keys` is licensed under either of the following, at your option:

- [Apache License 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

[ci-cd-badge]: https://github.com/risoflora/lock_keys/actions/workflows/CI.yml/badge.svg
[ci-cd-url]: https://github.com/risoflora/lock_keys/actions/workflows/CI.yml
[crates-badge]: https://img.shields.io/crates/v/lock_keys.svg
[crates-url]: https://crates.io/crates/lock_keys
[docs-badge]: https://docs.rs/lock_keys/badge.svg
[docs-url]: https://docs.rs/lock_keys
[license-badge]: https://img.shields.io/crates/l/lock_keys.svg
[license-url]: https://github.com/risoflora/lock_keys#license
[xlib-wiki-url]: https://en.wikipedia.org/wiki/Xlib
[winuser-api-url]: https://docs.microsoft.com/en-us/windows/win32/api/winuser
[iokit-url]: https://developer.apple.com/documentation/iokit

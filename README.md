# Omango-Futex

This is implement futex for Rust .<br />

Compared with [atomic-wait](https://github.com/m-ou-se/atomic-wait/blob/main),
it provides additional support [wait_until](https://github.com/tqtrungse/omango-futex/blob/master/src/lib.rs#L56)
on Windows, Unix, FreeBSD.

## Table of Contents

- [Usage](#usage)
- [Compatibility](#compatibility)
- [License](#license)

## Usage

Add this to your `Cargo.toml`:
```toml
[dependencies]
omango-futex = "0.1.0"
```

## Compatibility

The minimum supported Rust version is 1.49.

## License

The crate is licensed under the terms of the MIT
license. See [LICENSE](LICENSE) for more information.

#### Third party software

This product includes copies and modifications of software developed by third parties:

* [atomic-wait](https://github.com/m-ou-se/atomic-wait/blob/main)
See the source code files for more details.

The third party licenses can be found in [here](https://github.com/m-ou-se/atomic-wait/blob/main/LICENSE).

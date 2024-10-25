# codex-percnet

[![Discord chat][discord-badge]][discord-url]
[![Crates.io](https://img.shields.io/crates/v/codex-percent.svg)](https://crates.io/crates/codex-percent)
[![Docs](https://docs.rs/codex-percent/badge.svg)](https://docs.rs/codex-percent)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![MSRV](https://img.shields.io/badge/MSRV-1.70.0-blue)


`no_std` friendly percent en-de-coding for the 1%

## Usage

### Fixed Array

This uses fixed bytearray as output where you are responsible of ensuring large enough output size.

With `FixedEncoder`
```ignore
cargo add codex-percent --no-default-features --features fixed,encoder
```

With `FixedDecoder`
```ignore
cargo add codex-percent --no-default-features --features fixed,decoder
```

### Allocating Vec

This uses allocating vec as output where you provide a mutable Vec that will re-allocate if needed.

With `VecEncoder` (uses `alloc::vec`)
```ignore
cargo add codex-percent --no-default-features --features vec,encoder
```

With `VecDecoder` (uses `alloc::vec`)
```ignore
cargo add codex-percent --no-default-features --features vec,decoder
```

See [examples](./examples).

## Decoder Benchmarks

In MacBook 13" M1 comparitative results:


| Type         | Scenario                       | Perf                            |
| :---         | :---                           | :---                            |
| FixedDecoder | Poop-U1-4B                     | [16.897 ns 16.928 ns 16.969 ns] |
| FixedDecoder | Poop-U3-12B                    | [38.939 ns 39.021 ns 39.113 ns] |
| FixedDecoder | Poop-U6-24B                    | [73.677 ns 73.746 ns 73.830 ns] |
| VecDecoder   | Poop-U6-24B straightline       | [149.90 ns 151.21 ns 152.46 ns] |
| VecDecoder   | Poop-U6-24B capped, re-use     | [83.460 ns 83.646 ns 83.889 ns] |

## Encoder Benchmarks

In MacBook 13" M1 comparitative results:

| Type         | Scenario                       | Perf                            |
| :---         | :---                           | :---                            |
| FixedEncoder | Poop-U1-3B >> 12B              | [8.7721 ns 8.7840 ns 8.7991 ns] |
| FixedEncoder | Poop-U3-9B >> 36B              | [18.073 ns 18.149 ns 18.228 ns] |
| FixedEncoder | Poop-U6-18B >> 72B             | [29.173 ns 29.267 ns 29.375 ns] |
| VecEncoder   | no-init-cap Poop-U1-3B >> 12B  | [42.965 ns 43.090 ns 43.221 ns] |
| VecEncoder   | no-init-cap Poop-U3-9B >> 36B  | [112.44 ns 112.84 ns 113.36 ns] |
| VecEncoder   | no-init-cap Poop-U6-18B >> 72B | [153.92 ns 154.25 ns 154.59 ns] |
| VecEncoder   | capped Poop-U1-3B >> 12B       | [24.478 ns 24.507 ns 24.535 ns] |
| VecEncoder   | capped Poop-U3-9B >> 36B       | [37.446 ns 37.578 ns 37.724 ns] |
| VecEncoder   | capped Poop-U6-18B >> 72B      | [52.808 ns 52.976 ns 53.167 ns] |
| VecEncoder   | re-use Poop-U1-3B >> 12B       | [12.565 ns 12.605 ns 12.649 ns] |
| VecEncoder   | re-use Poop-U3-9B >> 36B       | [21.859 ns 21.906 ns 21.957 ns] |
| VecEncoder   | re-use Poop-U6-18B >> 72B      | [37.303 ns 37.371 ns 37.445 ns] |

If you know the output size and can deal with a potential panic out-of-bounds, `FixedEncoder` is fast.

Panic free `VecEncoder` *capped* means `Vec::with_capacity()` and re-use if needed e.g. with `.clear()`

If you have large output, use `VecEncoder` as arrays are more suitable for small outputs.

[discord-badge]: https://img.shields.io/discord/934761553952141402.svg?logo=discord
[discord-url]: https://discord.gg/rXVsmzhaZa

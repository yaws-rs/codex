# quoted-values

[![Discord chat][discord-badge]][discord-url]
[![Crates.io](https://img.shields.io/crates/v/codex-percent.svg)](https://crates.io/crates/codex-percent)
[![Docs](https://docs.rs/codex-percent/badge.svg)](https://docs.rs/codex-percent)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![MSRV](https://img.shields.io/badge/MSRV-1.70.0-blue)

`no_std` and alloc free quoted US-ASCII only backslash remover

Intended to be used in context of tearing down various network protocol layers that require passing header values in US-ASCII context and which contain quoted values inside double-quote (`"`) characters.

Decoder is intended to replace all `\"` and `\\` characters with `"` and `\` respectively.

It would be trivial with an allocator but this focused on in-place removal with minimal amount of memmoves using slice::copy_within in byte / US-ASCII context without copying or allocating.

[discord-badge]: https://img.shields.io/discord/934761553952141402.svg?logo=discord
[discord-url]: https://discord.gg/rXVsmzhaZa

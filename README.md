<div align="center">

# Bevy Ogle

**A multi-mode camera for 2D vector games in [Bevy](https://bevyengine.org).**

[![Discord](https://img.shields.io/discord/913957940560531456.svg?label=Loopy&logo=discord&logoColor=ffffff&color=ffffff&labelColor=000000)](https://discord.gg/zrjnQzdjCB)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#license)
[![Following released Bevy versions](https://img.shields.io/badge/bevy%20tracking-released%20version-lightblue)](https://bevyengine.org/learn/quick-start/plugin-development/#main-branch-tracking)\
[![Dependency status](https://deps.rs/repo/github/loopystudios/bevy_ogle/status.svg)](https://deps.rs/repo/github/loopystudios/bevy_ogle)
[![Crates.io](https://img.shields.io/crates/v/bevy_ogle.svg)](https://crates.io/crates/bevy_ogle)
[![Docs](https://img.shields.io/docsrs/bevy_ogle)](https://docs.rs/bevy_ogle)
[![Build status](https://github.com/loopystudios/bevy_ogle/workflows/CI/badge.svg)](https://github.com/loopystudios/bevy_ogle/actions)

</div>

Quickstart to run an example:

```shell
cargo run -p demo
```

## Bevy version support

|bevy|bevy_ogle|
|---|---|
|0.17|0.8-0.9,main|
|0.16|0.5-0.7|
|0.15|0.3-0.4|
|0.14|0.1-0.2|
|< 0.13| unsupported |

## Usage

There are several [examples](examples/) for reference.

You can also run examples on web:

```shell
# Make sure the Rust toolchain supports the wasm32 target
rustup target add wasm32-unknown-unknown

cargo run_wasm -p demo
```

## Community

All Loopy projects and development happens in the [Loopy Discord](https://discord.gg/zrjnQzdjCB). The discord is open to the public.

Contributions are welcome by pull request. The [Rust code of conduct](https://www.rust-lang.org/policies/code-of-conduct) applies.

## License

Licensed under either of

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

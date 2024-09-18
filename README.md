# 🗒⛏ `notemine_hw` ⚡⚙️

This is a fork of [`notemine`](https://github.com/sandwichfarm/notemine).

`notemine_hw` is a rust-based tool for mining [nostr](https://nostr.com) notes.

While its parent project compiles to [WASM](https://webassembly.org/) and caters for a web-native experience,
`notemine_hw` aims to leverage hardware acceleration for mining notes.

The UI is CLI-based, allowing for:
- Configurable PoW difficulty.
- Realtime hashrate logging.
- Events to be mined are provided as JSON files.

## platform support

- [ ] MacBook Pro GPU via [`metal-rs`](https://crates.io/crates/metal)
- [ ] Linux GPU via [`opencl3`](https://crates.io/crates/opencl3)

## dependencies

xxx todo instructions to install metal and/or openCL xxx

## build

xxx todo xxx

## run demo

xxx todo xxx

## license
GNU General Public License v3.0
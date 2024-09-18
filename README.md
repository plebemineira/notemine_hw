# 🗒⛏ `notemine_hw` ⚡⚙️

This is a fork of [`notemine`](https://github.com/sandwichfarm/notemine).

`notemine_hw` is a rust-based tool for mining [nostr](https://nostr.com) notes under [NIP13](https://github.com/nostr-protocol/nips/blob/master/13.md).

While its parent project compiles to [WASM](https://webassembly.org/) and caters for a web-native experience,
`notemine_hw` aims to leverage hardware acceleration for mining notes.

The UI is CLI-based, allowing for:
- Configurable PoW difficulty.
- Multithreaded workers.
- Realtime hashrate logging.
- Events to be mined are provided as JSON files.

## platform support

- [ ] MacBook Pro GPU via [`metal-rs`](https://crates.io/crates/metal) (TODO)
- [ ] Linux GPU via [`opencl3`](https://crates.io/crates/opencl3) (TODO)

## dependencies

xxx todo instructions to install metal and/or openCL xxx

## build

xxx todo build instructions with cargo feature flags xxx

## usage

```shell
$ notemine_hw -h
Usage: notemine_hw --n-workers <N_WORKERS> --difficulty <DIFFICULTY> --event-json <EVENT_JSON>

Options:
  -n, --n-workers <N_WORKERS>    number of workers
  -d, --difficulty <DIFFICULTY>  difficulty
  -e, --event-json <EVENT_JSON>  path to event JSON file
  -h, --help                     Print help

$ notemine_hw --n-workers 3 --difficulty 20 --event-json event.json
2024-09-18T19:21:20.665505Z  INFO notemine_hw: 🗒⛏ notemine_hw ⚡⚙️
2024-09-18T19:21:20.665857Z  INFO notemine: starting worker with parameters: worker id: 1 | difficulty: 20 | start_nonce: 1 | nonce_step: 6148914691236517205
2024-09-18T19:21:20.665857Z  INFO notemine: starting worker with parameters: worker id: 0 | difficulty: 20 | start_nonce: 0 | nonce_step: 6148914691236517205
2024-09-18T19:21:20.665857Z  INFO notemine: starting worker with parameters: worker id: 2 | difficulty: 20 | start_nonce: 2 | nonce_step: 6148914691236517205
2024-09-18T19:21:21.665960Z  INFO notemine: worker id: 1 | hashrate: 52254 h/s | best pow: 15 | best nonce: 6148914691236514724 | best hash: "0001b2395292f878dd9283888659dc9fd23d4001a08ad7a567266d8759247e8f"
2024-09-18T19:21:21.665960Z  INFO notemine: worker id: 0 | hashrate: 52080 h/s | best pow: 15 | best nonce: 6148914691236514724 | best hash: "0001b2395292f878dd9283888659dc9fd23d4001a08ad7a567266d8759247e8f"
2024-09-18T19:21:21.665982Z  INFO notemine: worker id: 2 | hashrate: 51518 h/s | best pow: 15 | best nonce: 6148914691236514724 | best hash: "0001b2395292f878dd9283888659dc9fd23d4001a08ad7a567266d8759247e8f"
2024-09-18T19:21:22.665970Z  INFO notemine: worker id: 0 | hashrate: 52592 h/s | best pow: 16 | best nonce: 18446744073709519723 | best hash: "00008d51ef546ec45059e5113d39ec65459bb7160981e3a917265a5f19e23b03"
2024-09-18T19:21:22.665998Z  INFO notemine: worker id: 2 | hashrate: 52680 h/s | best pow: 16 | best nonce: 18446744073709519723 | best hash: "00008d51ef546ec45059e5113d39ec65459bb7160981e3a917265a5f19e23b03"
2024-09-18T19:21:22.665970Z  INFO notemine: worker id: 1 | hashrate: 52612 h/s | best pow: 16 | best nonce: 18446744073709519723 | best hash: "00008d51ef546ec45059e5113d39ec65459bb7160981e3a917265a5f19e23b03"
2024-09-18T19:21:23.665983Z  INFO notemine: worker id: 1 | hashrate: 52805 h/s | best pow: 16 | best nonce: 18446744073709519723 | best hash: "00008d51ef546ec45059e5113d39ec65459bb7160981e3a917265a5f19e23b03"
2024-09-18T19:21:23.665995Z  INFO notemine: worker id: 0 | hashrate: 52773 h/s | best pow: 16 | best nonce: 18446744073709519723 | best hash: "00008d51ef546ec45059e5113d39ec65459bb7160981e3a917265a5f19e23b03"
2024-09-18T19:21:23.666028Z  INFO notemine: worker id: 2 | hashrate: 52625 h/s | best pow: 16 | best nonce: 18446744073709519723 | best hash: "00008d51ef546ec45059e5113d39ec65459bb7160981e3a917265a5f19e23b03"
2024-09-18T19:21:24.666001Z  INFO notemine: worker id: 1 | hashrate: 52837 h/s | best pow: 16 | best nonce: 18446744073709519723 | best hash: "00008d51ef546ec45059e5113d39ec65459bb7160981e3a917265a5f19e23b03"
2024-09-18T19:21:24.666011Z  INFO notemine: worker id: 0 | hashrate: 52813 h/s | best pow: 16 | best nonce: 18446744073709519723 | best hash: "00008d51ef546ec45059e5113d39ec65459bb7160981e3a917265a5f19e23b03"
2024-09-18T19:21:24.666042Z  INFO notemine: worker id: 2 | hashrate: 52719 h/s | best pow: 16 | best nonce: 18446744073709519723 | best hash: "00008d51ef546ec45059e5113d39ec65459bb7160981e3a917265a5f19e23b03"
2024-09-18T19:21:25.665994Z  INFO notemine: worker id: 1 | hashrate: 52815 h/s | best pow: 16 | best nonce: 18446744073709519723 | best hash: "00008d51ef546ec45059e5113d39ec65459bb7160981e3a917265a5f19e23b03"
2024-09-18T19:21:25.666025Z  INFO notemine: worker id: 0 | hashrate: 52654 h/s | best pow: 16 | best nonce: 18446744073709519723 | best hash: "00008d51ef546ec45059e5113d39ec65459bb7160981e3a917265a5f19e23b03"
2024-09-18T19:21:25.666067Z  INFO notemine: worker id: 2 | hashrate: 52822 h/s | best pow: 16 | best nonce: 18446744073709519723 | best hash: "00008d51ef546ec45059e5113d39ec65459bb7160981e3a917265a5f19e23b03"
2024-09-18T19:21:26.666019Z  INFO notemine: worker id: 1 | hashrate: 52816 h/s | best pow: 16 | best nonce: 18446744073709519723 | best hash: "00008d51ef546ec45059e5113d39ec65459bb7160981e3a917265a5f19e23b03"
2024-09-18T19:21:26.666036Z  INFO notemine: worker id: 0 | hashrate: 52671 h/s | best pow: 16 | best nonce: 18446744073709519723 | best hash: "00008d51ef546ec45059e5113d39ec65459bb7160981e3a917265a5f19e23b03"
2024-09-18T19:21:26.666075Z  INFO notemine: worker id: 2 | hashrate: 52807 h/s | best pow: 16 | best nonce: 18446744073709519723 | best hash: "00008d51ef546ec45059e5113d39ec65459bb7160981e3a917265a5f19e23b03"
2024-09-18T19:21:27.150643Z  INFO notemine_hw: successfully mined event in 6.4855957 seconds
2024-09-18T19:21:27.150668Z  INFO notemine_hw: MinedResult { event: NostrEvent { pubkey: "e771af0b05c8e95fcdf6feb3500544d2fb1ccd384788e9f490bb3ee28e8ed66f", kind: 1, content: "hello world", tags: [["nonce", "12297829382472920563", "20"]], id: Some("0000047edf56cefcdc1d35f352b7453011b3bc24552b0fb36880efde721fb915"), created_at: Some(1668680774) }, total_time: 6.484595042, khs: 3.9180549957925965 }
2024-09-18T19:21:27.150692Z  INFO notemine_hw: exiting...
```

## license
GNU General Public License v3.0

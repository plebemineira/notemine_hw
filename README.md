# note⛏️
**notemine** is wasm miner for nostr notes written in rust. Variable difficulty and realtime hashrate. There's a [demo](https://sandwichfarm.github.io/notemine). It should be easy to import into your project.

# deps 
```
cargo install wasm-pack
```

# build
```
cargo clean
wasm-pack build --target web --release
```

# run demo
```
cd demo && npx serve 
```

# license
GNU General Public License v3.0

<h1 align="center">
  <br>
  <img width="200" src="./img/logo.png">
  <br>
🗒⛏ notemine_hw ⚡⚙️
<br>
</h1>

This is a fork of [`notemine`](https://github.com/sandwichfarm/notemine).

`notemine_hw` is a rust-based tool for mining [nostr](https://nostr.com) notes under [`NIP-13`](https://github.com/nostr-protocol/nips/blob/master/13.md).

While its parent project compiles to [WASM](https://webassembly.org/) and caters for a web-native experience,
`notemine_hw` aims to leverage hardware acceleration for mining notes.

## UI/UX

The UI/UX aims for:
- Configurable PoW difficulty.
- Multithreaded workers.
- Realtime hashrate logging.
- User can mine and publish their own PoW events.
- User can sell PoW for BOLT11 invoices (via [LDK](https://lightningdevkit.org/)).

> "Why not zaps?"

you might wonder... well, at the time of writing, the [`nostr-zapper`](https://crates.io/crates/nostr-zapper) 
crate seems WIP, while LDK is a more mature library. So BOLT11 seems like a better choice, at least for now.

> "Why not BOLT12?"

you might wonder... well, there's no real need for invoice reusability. In fact, each PoW event payment
should have a unique invoice. Therefore, BOLT11 is a more adequate choice.

### CLI

Assuming the user's system has some hardware available for mining, the CLI provides the following subcommands:
- `notemine_hw mine`: takes an event as a JSON file from disk, and writes the mined PoW event into a new JSON file.
- `notemine_hw publish`: takes an event as a JSON file from disk and publishes the PoW event (signed with user `nsec`) to some specified relay.
- `notemine_hw sell`: spawns a JSON-RPC server + LN node for selling PoW.

Assuming the user's system does not have some hardware available for mining, the CLI provides the following subcommands:
- `notemine_hw buy`: spawns a JSON-RPC client for buying PoW.

#### `notemine_hw mine`

#### `notemine_hw publish`
```shell
$ notemine_hw publish -h
Usage: notemine_hw publish --n-workers <N_WORKERS> --log-interval <LOG_INTERVAL> --difficulty <DIFFICULTY> --event-json <EVENT_JSON> --relay-url <RELAY_URL> --nsec <NSEC>

Options:
      --n-workers <N_WORKERS>        number of workers
      --log-workers                  log individual workers
  -d, --difficulty <DIFFICULTY>      difficulty
  -e, --event-json <EVENT_JSON>      path to event JSON file
  -r, --relay-url <RELAY_URL>        relay URL
      --nsec <NSEC>                  nsec
  -h, --help                         Print help

$ notemine_hw publish --n-workers 3 --log-workers --difficulty 20 --event-json event.json --relay-url 'wss://plebemineira.xyz' --nsec nsec13ezg388stxfnxe72nc20428r7lrxzst85d60vxynk6zr57lpxu6svjam98
2024-09-29T02:31:53.871924Z  INFO notemine_hw: 🗒⛏ notemine_hw ⚡⚙️
2024-09-29T02:31:53.876172Z  INFO notemine::service: starting miner service to mine and publish the JSON event
2024-09-29T02:31:53.878672Z  INFO notemine::miner: starting worker with parameters: worker id: 0 | difficulty: 20 | start_nonce: 0
2024-09-29T02:31:53.878684Z  INFO notemine::miner: starting worker with parameters: worker id: 2 | difficulty: 20 | start_nonce: 12297829382473034410
2024-09-29T02:31:53.878690Z  INFO notemine::miner: starting worker with parameters: worker id: 1 | difficulty: 20 | start_nonce: 6148914691236517205
2024-09-29T02:31:57.879543Z  INFO notemine::hashrate: 
 worker id    hashrate  
     0       52093 h/s  
     1       52390 h/s  
     2       52401 h/s  
  global     156884 h/s 
2024-09-29T02:31:58.878956Z  INFO notemine::hashrate: 
 worker id    hashrate  
     0       52097 h/s  
     1       52427 h/s  
     2       52454 h/s  
  global     156978 h/s 
2024-09-29T02:31:59.878997Z  INFO notemine::hashrate: 
 worker id    hashrate  
     0       52094 h/s  
     1       52456 h/s  
     2       52496 h/s  
  global     157046 h/s 
2024-09-29T02:32:00.879029Z  INFO notemine::hashrate: 
 worker id    hashrate  
     0       52108 h/s  
     1       52472 h/s  
     2       52507 h/s  
  global     157087 h/s 
2024-09-29T02:32:00.879186Z  INFO notemine::service: successfully mined event in 7.002937 seconds
2024-09-29T02:32:00.879202Z  INFO notemine::service: MinedResult { event: PoWEvent { pubkey: "98590c0f4959a49f3524b7c009c190798935eeaa50b1232ba74195b419eaa2f2", kind: 1, content: "hello world", tags: [["nonce", "12297829382473392311", "20"]], id: Some("0000042f019fec1dff79c7dd893349bd325ffee2f0df9f322b219bb6b1902831"), created_at: Some(1727577113), sig: None }, total_time: 6.8147005830000005 }
2024-09-29T02:32:00.882981Z  INFO nostr_sdk::client::handler: Spawned client notification handler
2024-09-29T02:32:00.883819Z  INFO notemine::client: connecting to relay: wss://plebemineira.xyz
2024-09-29T02:32:02.010302Z  INFO nostr_relay_pool::relay::internal: Connected to 'wss://plebemineira.xyz/'
2024-09-29T02:32:02.276292Z  INFO notemine::client: send mined event output: Output { val: EventId(0000042f019fec1dff79c7dd893349bd325ffee2f0df9f322b219bb6b1902831), success: {Url { scheme: "wss", cannot_be_a_base: false, username: "", password: None, host: Some(Domain("plebemineira.xyz")), port: None, path: "/", query: None, fragment: None }}, failed: {} }
2024-09-29T02:32:02.276651Z  INFO notemine_hw: exiting...
2024-09-29T02:32:02.276842Z  INFO nostr_relay_pool::pool::internal: Relay pool shutdown
```

The input JSON must contain the following fields:
- `pubkey`
- `kind`
- `tags`
- `content`

Optionally, the `created_at` field can also be provided. If not, the current UNIX timestamp is used.

for example:
```shell
$ cat event.json
{
  "pubkey": "98590c0f4959a49f3524b7c009c190798935eeaa50b1232ba74195b419eaa2f2",
  "created_at": 1668680774,
  "kind": 1,
  "tags": [],
  "content": "hello world",
}
```

#### `notemine_hw sell`

This subcommand allows the user to sell PoW for BOLT11 invoices.

PoW Price is calculated according to this formula:

$$ P = 2^{(d \cdot p)} $$

where:
- $P$: PoW Price [sats]
- $p$: PoW Price factor
- $d$: PoW difficulty

![](./img/pow_price.png)

PoW sellers modulate their PoW Price factor $p$ in order to charge more or less sats according to PoW difficulty.

The `notemine_hw sell` subcommand is used to sell PoW.

xxx todo more LN info on CLI args xxx

```shell
$ notemine_hw sell -h
Usage: notemine_hw sell --n-workers <N_WORKERS> --log-interval <LOG_INTERVAL> --rpc-port <RPC_PORT> --pow-price-factor <POW_PRICE_FACTOR>

Options:
      --n-workers <N_WORKERS>                number of workers
      --log-workers                          log individual workers
  -r, --rpc-port <RPC_PORT>                  RPC port
  -p, --pow-price-factor <POW_PRICE_FACTOR>  PoW price factor
  -h, --help                                 Print help
```

A potential PoW buyer quotes the PoW price like this:
```shell 
$ curl -X POST -H "Content-Type: application/json" -d '{
   "jsonrpc": "2.0",
   "method": "quote",
   "params": {
      "difficulty": 20,
   },
   "id": 1
}' http://localhost:1337
{
  "jsonrpc": "2.0",
  "result": {
    "difficulty": 20,
    "pow-price": 1048576.0,
    "pow-price-factor": 1.0
  },
  "id": 1
}
```

In the example above, the buyer needs to zap `1048576.0` sats to mine a note with difficulty `20`, because `pow-price-factor` is set to `1.0`.

xxx todo JSON-RPC for BOLT11 invoice xxx

### `notemine_hw buy`

This command allows for the user to buy PoW by paying BOLT11 invoices.

xxx todo CLI xxx

## platform support

- [ ] Linux GPU via [`opencl3`](https://crates.io/crates/opencl3) (TODO)

## dependencies

xxx todo instructions to install openCL xxx

## build

xxx todo build instructions with cargo feature flags xxx

## license
GNU General Public License v3.0

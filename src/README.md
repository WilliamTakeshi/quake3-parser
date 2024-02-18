# Quake3-Parser

A basic parser for Quake3 Arena logs.

## How to Run

Before running Quake3-Parser, you need to have Rust installed in your system. I recommend [rustup](https://rustup.rs/).

```
> git clone https://github.com/WilliamTakeshi/quake3-parser.git
> cd quake3-parser
> cargo run -- qgames.log
```


## Usage examples

```
> cargo run -- test.log
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/quake3-parser test.log`
{
  "match_2": {
    "total_kills": 11,
    "players": [
      "Isgalamido",
      "Dono da Bola",
      "Mocinha"
    ],
    "kills": {
      "Isgalamido": -5,
      "Dono da Bola": 0,
      "Mocinha": 0
    },
    "kills_by_means": {
      "MOD_TRIGGER_HURT": 7,
      "MOD_ROCKET_SPLASH": 3,
      "MOD_FALLING": 1
    }
  },
  "match_1": {
    "total_kills": 0,
    "players": [
      "Isgalamido"
    ],
    "kills": {
      "Isgalamido": 0
    },
    "kills_by_means": {}
  }
}
```

## Running tests

Quake3-Parser comes with a set of tests to ensure it's working. To run these tests you simply need to run
the following command:

```
> cargo test
```


# Bevy tests

Boid simulation in the bevy engine.


## Setup

### Wasm

Install the wasm32 target and download a wasm web server for testing.
```
rustup target install wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown
cargo install wasm-server-runner
```

Add this to .cargo/config.toml to tell cargo to build for this web server specifically.
```
[target.wasm32-unknown-unknown]
runner = "wasm-server-runner"
```

Run and open the project in browser.
```
$ cargo run --target wasm32-unknown-unknown
```

### Hot Reloading

Bacon can be used for hot reloading:
```
cargo install bacon
bacon --init
bacon run-long
```

Set default job:
```bacon.toml
default_job = "run-long"
```
Then run with:
```
bacon
```

This sets the default build target:
```.cargo/config.toml
[build]
target = "wasm32-unknown-unknown"
```

## Optimization history

2026-05-27 - 500  - Naive 
2026-05-27 - 2000 - Spatial partitions
2026-06-02 - 4000 - Limit entity per cell
2026-06-03 -  8000 - Reduce calls to spatial partition



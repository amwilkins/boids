# Bevy tests

## Wasm

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

## Hot Reloading

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

This sets the default build target:
```.cargo/config.toml
[build]
target = "wasm32-unknown-unknown"

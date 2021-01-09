# `For Actinius`

This is an example application that will run the lights in an uplifting manner every 10 seconds.

## Building

Assuming Rust is installed.

```
# Add target for the M33 (thumbv8m.main-none-eabihf works too)
rustup target add thumbv8m.main-none-eabi

# Build it in release mode. The default target is set in .cargo/config.toml
cargo build --release
```

## Running



To run with RTT output:

```
cargo install --git=https://github.com/probe-rs/cargo-embed

# I have to run this 3 times for it to work.
cargo embed --release
cargo embed --release
cargo embed --release
```

## Getting a .hex file

```
cargo install cargo-binutils
rustup component add llvm-tools-preview

cargo objcopy --release -- -O ihex test.hex
```
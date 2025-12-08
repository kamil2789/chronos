# Chronos

Game engine written in Rust. Currently in very early development.

## About

This is an educational project for learning how game/graphic engines work under the hood and practicing Rust programming. It's not meant to be a full-fledged game engine - for actual game development I'd use something like Godot. This is just for technical demos and experimentation.

## Project structure

- `chronos/` - Main library crate
- `sandbox/` - Example application for testing and experimenting with engine features

## Building

```bash
cargo build
cargo run -p sandbox
```

## Current Status

Just the foundations. ECS works, components exist, window opens. No actual rendering yet.

## License

See LICENSE file.
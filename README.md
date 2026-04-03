# Chronos

Graphics engine written in Rust. Currently in very early development.

## About

This is an educational project focused on learning how graphics engines work under the hood and on improving Rust skills. The project started with the idea of becoming a game engine, but the scope was intentionally reduced to a graphics engine with a small set of game-engine-like systems.

Chronos includes minimal supporting features such as ECS and similar engine infrastructure because they make the renderer and experiments more practical to use as a standalone project. The goal is not to build a full game engine, but a usable graphics-focused foundation for technical demos, rendering experiments, and low-level engine practice.

## Project structure

- `chronos/` - Main library crate
- `sandbox/` - Example application for testing and experimenting with engine features

## Building

```bash
cargo build
cargo run -p sandbox
```

## Current Status

Early foundations only. Core engine structures exist, ECS is in place, and the windowing setup works. Rendering is still at an early stage.

## License

See LICENSE file.
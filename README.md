# Kostka

### Your portable Rubik's Cube

![Cube of God](https://raw.githubusercontent.com/hejsztynx/kostka/main/readme/cube.gif)

##### A fast and lightweight Rubik’s Cube simulator written in Rust, featuring smooth real-time rendering and intuitive controls.

## Features ✨

- `Pixels + winit rendering` *– lightweight, fast, and perfect for pixel-based graphics.* [pixels crate.io page](https://crates.io/crates/pixels)

- `Manual 2D projection` – all cube transformations and perspective math are done by hand, no external 3D engine.

- `Intuitive key mappings` – *speedcube-ready* controls designed to feel natural for cubers and provide highest mobility. Nothing better than relearning all of your algorithms on the keyboard all over again!

- `Smooth performance` – Fluid experience with **60 FPS**.

- `Timer functionality` – measure your solve times directly in the app.

## Controls 🎮

Key mappings are designed to resemble real cube rotations as much as possible. Experiment a bit — after a few tries it becomes second nature.

![controls](https://raw.githubusercontent.com/hejsztynx/kostka/main/readme/controls.png)

Timer can be switched on and off with the `2` key. You can only start the timer after resetting the game and scrambling the cube with the `1` key.

## In Progress 🚧

Currently working on:

- Custom performance and cube speed settings.

## Installing 🔧

Make sure that *cargo bin* directory is added to system's `PATH`

```
cargo install kostka
kostka
```

*Requires Rust installed (latest stable recommended).*

# Kostka

### Your portable Rubik's Cube

![Cube of God](https://raw.githubusercontent.com/hejsztynx/kostka/main/readme/cube.gif)

##### A fast and lightweight Rubik’s Cube simulator written in Rust, featuring smooth real-time rendering and intuitive controls.

## Features ✨

- `Pixels rendering` *– lightweight, fast, and perfect for pixel-based graphics.* [pixels crate.io page](https://crates.io/crates/pixels)


- `Manual 2D projection` – all cube transformations and perspective math are done by hand, no external 3D engine.

- `Intuitive key mappings` – designed to feel natural for cubers and provide highest mobility. Nothing better than relearning all of your algorithms on the keyboard all over again!

- `Smooth performance` – Fluid experience with **60 FPS**.

## Controls 🎮

Key mappings are designed to resemble real cube rotations as much as possible. Experiment a bit — after a few tries it becomes second nature.

![controls](https://raw.githubusercontent.com/hejsztynx/kostka/main/readme/controls.png)

## In Progress 🚧

Currently working on:

- Timer functionality ⏱️ – measure your solve times directly in the app.

## Building 🔧

```
git clone https://github.com/hejsztynx/kostka-cube
cd kostka-cube
cargo run
```

*Requires Rust installed (latest stable recommended).*
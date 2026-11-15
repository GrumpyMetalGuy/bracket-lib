# Welcome to bracket-lib

> Note that you may need to run `cargo update` to pull some dependency updates. If you are coming from Hands-on Rust, the code is maintained [here](https://github.com/thebracket/handsonrust)

![](https://github.com/thebracket/bracket-lib/workflows/Rust/badge.svg)

> You can read a tutorial series on writing a Roguelike with this library at: [https://bfnightly.bracketproductions.com/rustbook/](https://bfnightly.bracketproductions.com/rustbook/)

> *Bracket-lib* is the primary support library for my book, [Hands-on Rust](https://hands-on-rust.com/). Please consider checking out my book.

Early work has begun on writing a manual. You can find it in the `manual` folder, or [read it online](https://bfnightly.bracketproductions.com/bracket-lib/what_is_it.html).

**BREAKING CHANGE ALERT**: The `crossterm` feature is now `cross_term` if you are using `bracket-terminal` directly. It's still `crossterm` for `bracket-lib` and `rltk`.

**IMPORTANT**: If you are running the `webgpu` backend, you need to add `resolver = 2` to your `Cargo.toml` file. WGPU requires it for platform selection.

## Known deviations from upstream

This fork applies a set of bug fixes and dependency upgrades on top of
upstream `amethyst/bracket-lib`. Notable behavioral notes:

- Bumped `bracket-bevy` to bevy 0.13. `bracket-color`'s `RGB`/`RGBA` ->
  `bevy::Color` conversions map to bevy's sRGB variants to preserve
  pre-0.13 behavior. `RGBA::as_rgba_f32()` still returns *linear* RGBA
  despite its name; this matches the historical implementation and is
  deliberately kept for backwards compatibility (see the inline comment
  in `bracket-color/src/rgba.rs`).

- Bumped `bracket-bevy` further to bevy 0.16 (skipping the broken 0.15.x
  umbrella releases, which had unresolvable sub-crate patch drift). The
  0.15 "bundle removal" rework was applied: the 2D camera, and 2D mesh
  entities, are now spawned as plain components (`Camera2d` + `Msaa::Off`;
  `Mesh2d` + `MeshMaterial2d<ColorMaterial>`) instead of `Camera2dBundle`/
  `MaterialMesh2dBundle`. MSAA moved from a global `Msaa` resource to a
  per-camera `Msaa` component on the camera entity.

- Bumped `bracket-bevy` to bevy 0.17. This was mostly module re-export moves
  (`bevy::mesh`, `bevy::camera`, `bevy::sprite_render`, `bevy::asset`), plus
  `GlobalTransform::compute_matrix` -> `to_matrix`. `Camera::target` gained a
  `RenderTarget::None` variant; the mouse-unproject code treats it as
  no-op (no window to project into).

## What happened to RLTK?

This *is* RLTK, renamed because it is increasingly finding usage outside of just Roguelikes. It's also been divided into a number of crates, to make it easy to pick-and-choose the features you need.

* `rltk` crate wraps `bracket-lib` and re-exports in the `rltk::` and `rltk::prelude` namespace. This preserves compatibility with all existing RLTK projects.
* `bracket-algorithm-traits` exposes the traits required for the various algorithm systems in other crates.
* `bracket-color` is my RGB/HSV color management system.
* `bracket-geometry` exposes various geometric primitives and helpers. Supports other crates.
* `bracket-noise` is a port of [Auburn's FastNoise](https://github.com/Auburns/FastNoise) to Rust.
* `bracket-pathfinding` provides a high-performance A* (A-Star) pathing system, as well as Dijkstra maps.
* `bracket-random` is a dice-oriented random number generator, including parsing of RPG-style dice strings such as `3d6+12`.

## Using `bracket-lib`

In your `Cargo.toml` file, include:

```toml
[dependencies]
bracket-lib = "~0.8"
```

## Feature Flags

There are a few feature flags designed to aide integration with other systems:

* `specs` tells various `bracket-lib` sub-systems to export important primitives as having Specs' `Component` type applied.
* `serde` tells various `bracket-lib` sub-systems to support using `Serde` for serialization/de-serialization.

Performance:

* `threaded` enables multi-threading on some sub-systems.

Terminal mode:

By default, `bracket-lib` runs in OpenGL mode (or WebGL if it detects that you are compiling for `wasm32-unknown-unknown`). If you want to use other rendering back-ends, *disable default features* and apply *one* of the following feature flags:

* `webgpu` to use the `wgpu` system as a back-end, supporting Vulkan, Metal and WebGPU.
* `crossterm` to use the excellent `Crossterm` terminal library.
* `curses` to use `pancurses` for `ncurses` or `pdcurses` support depending upon your platform.

## Sample Projects

- https://github.com/Micutio/innit
- https://github.com/amethyst/shotcaller
- https://github.com/bofh69/rouge
- https://github.com/carsin/miners
- https://github.com/baszalmstra/my-little-robots
- https://github.com/Havegum/Terrain-Generator
- https://github.com/Bobox214/rs-gliphus
- https://github.com/Maxgy/blademaster
- https://github.com/Maxgy/text-rts
